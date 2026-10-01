"""A small, resilient HTTP client for pulling files from NSE.

NSE blocks naive scrapers: you need a browser-ish User-Agent and you must first
touch the main site so the WAF hands you cookies. This module wraps that plus
retry/backoff so the rest of the pipeline can just call ``get_bytes(url)``.
"""
from __future__ import annotations

import time

import requests

import config

_UA = (
    "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 "
    "(KHTML, like Gecko) Chrome/125.0 Safari/537.36"
)

_HEADERS = {
    "User-Agent": _UA,
    "Accept": "text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8",
    "Accept-Language": "en-US,en;q=0.9",
    "Referer": "https://www.nseindia.com/",
}


class NSEClient:
    def __init__(self) -> None:
        self.session = requests.Session()
        self.session.headers.update(_HEADERS)
        self._primed = False

    def _prime(self) -> None:
        """Grab WAF cookies from the landing pages once per process."""
        try:
            self.session.get("https://www.nseindia.com", timeout=15)
            self.session.get(
                "https://www.nseindia.com/all-reports", timeout=15
            )
            self._primed = True
        except requests.RequestException:
            # Not fatal - archive host sometimes serves without cookies.
            self._primed = True

    def get_bytes(self, url: str) -> bytes | None:
        """Return response body, or None if the resource is a genuine 404.

        Retries transient failures (timeouts, 403 from a stale cookie, 5xx).
        """
        if not self._primed:
            self._prime()

        last_exc: Exception | None = None
        for attempt in range(1, config.MAX_RETRIES + 1):
            try:
                resp = self.session.get(url, timeout=30)
                if resp.status_code == 404:
                    return None
                if resp.status_code == 403:
                    # cookie likely expired - re-prime and retry
                    self._primed = False
                    self._prime()
                    raise requests.RequestException("403 from NSE (re-primed)")
                resp.raise_for_status()
                if not resp.content:
                    return None
                return resp.content
            except requests.RequestException as exc:  # noqa: PERF203
                last_exc = exc
                if attempt < config.MAX_RETRIES:
                    time.sleep(config.RETRY_BACKOFF_SEC * attempt)
        raise RuntimeError(f"Failed to fetch {url} after {config.MAX_RETRIES} tries: {last_exc}")


_client: NSEClient | None = None


def client() -> NSEClient:
    global _client
    if _client is None:
        _client = NSEClient()
    return _client

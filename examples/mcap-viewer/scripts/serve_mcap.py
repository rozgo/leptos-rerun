#!/usr/bin/env python3

import argparse
import functools
import http.server
import os
import pathlib
import shutil
import socketserver
import tempfile
import urllib.error
import urllib.parse
import urllib.request


DEFAULT_PORT = 4318
DEFAULT_NAME = "recording.mcap"


class CORSRequestHandler(http.server.SimpleHTTPRequestHandler):
    def end_headers(self) -> None:
        self.send_header("Access-Control-Allow-Origin", "*")
        self.send_header("Access-Control-Allow-Headers", "*")
        self.send_header("Cache-Control", "no-store")
        super().end_headers()

    def log_message(self, format: str, *args) -> None:
        print(f"{self.address_string()} - {format % args}")


class ThreadingHTTPServer(socketserver.ThreadingMixIn, http.server.HTTPServer):
    daemon_threads = True


def copy_from_url(source_url: str, target: pathlib.Path) -> None:
    try:
        with urllib.request.urlopen(source_url) as response, target.open("wb") as output:
            shutil.copyfileobj(response, output)
    except urllib.error.HTTPError as error:
        raise SystemExit(
            f"Failed to download {source_url}: HTTP {error.code}. "
            "If this is a private SharePoint or Drive link, download the file in the browser "
            "first and rerun this script with --file /path/to/file.mcap."
        ) from error
    except urllib.error.URLError as error:
        raise SystemExit(f"Failed to download {source_url}: {error}") from error


def link_or_copy(source: pathlib.Path, target: pathlib.Path) -> None:
    try:
        target.symlink_to(source)
    except OSError:
        shutil.copy2(source, target)


def build_parser() -> argparse.ArgumentParser:
    parser = argparse.ArgumentParser(
        description="Serve a local or downloaded MCAP file over HTTP with permissive CORS.",
    )
    source = parser.add_mutually_exclusive_group(required=True)
    source.add_argument("--file", help="Path to an existing .mcap file")
    source.add_argument("--url", help="Direct URL to download before serving")
    parser.add_argument(
        "--port",
        type=int,
        default=DEFAULT_PORT,
        help=f"Port to listen on (default: {DEFAULT_PORT})",
    )
    parser.add_argument(
        "--name",
        default=DEFAULT_NAME,
        help=f"Served filename (default: {DEFAULT_NAME})",
    )
    parser.add_argument(
        "--viewer-port",
        type=int,
        default=8080,
        help="Port where the mcap-viewer example is running (default: 8080)",
    )
    return parser


def main() -> None:
    args = build_parser().parse_args()
    served_name = pathlib.Path(args.name).name

    with tempfile.TemporaryDirectory(prefix="leptos-rerun-mcap-") as temp_dir:
        temp_path = pathlib.Path(temp_dir)
        served_path = temp_path / served_name

        if args.file:
            source_path = pathlib.Path(args.file).expanduser().resolve()
            if not source_path.is_file():
                raise SystemExit(f"File not found: {source_path}")
            link_or_copy(source_path, served_path)
        else:
            copy_from_url(args.url, served_path)

        handler = functools.partial(CORSRequestHandler, directory=temp_dir)
        server = ThreadingHTTPServer(("127.0.0.1", args.port), handler)

        local_file_url = f"http://127.0.0.1:{args.port}/{urllib.parse.quote(served_name)}"
        viewer_query = urllib.parse.urlencode(
            {
                "url": local_file_url,
                "follow_if_http": "1",
            }
        )
        viewer_url = f"http://127.0.0.1:{args.viewer_port}/?{viewer_query}"

        print(f"Serving: {served_path}")
        print(f"File URL: {local_file_url}")
        print(f"Viewer URL: {viewer_url}")
        print("Press Ctrl+C to stop.")

        try:
            server.serve_forever()
        except KeyboardInterrupt:
            pass
        finally:
            server.server_close()


if __name__ == "__main__":
    main()

#!/usr/bin/env python3

import http.server
import socketserver

PORT = 8887

class CORSRequestHandler(http.server.SimpleHTTPRequestHandler):
    def end_headers(self):
        self.send_header('Access-Control-Allow-Origin', '*')
        http.server.SimpleHTTPRequestHandler.end_headers(self)

Handler = CORSRequestHandler

Handler.extensions_map[".wasm"] = "application/wasm"

with socketserver.TCPServer(("", PORT), Handler) as httpd:
    print("serving at port", PORT)
    httpd.serve_forever()

#!/usr/bin/env python

import SimpleHTTPServer
import SocketServer

PORT = 8001

class Handler(SimpleHTTPServer.SimpleHTTPRequestHandler):
    pass

Handler.extensions_map['.wasm'] = 'application/wasm'

httpd = SocketServer.TCPServer(("", PORT), Handler)
httpd.serve_forever()

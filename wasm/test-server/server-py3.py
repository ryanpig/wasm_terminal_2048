import http.server
import socketserver
import os
import argparse

def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('--port', '-p', default=6001, type=int)
    parser.add_argument('--dir', '-d', default='web', type=str)
    args = parser.parse_args()
    server_port = args.port
    server_dir = args.dir

    rootdir = os.path.join(os.path.dirname(__file__), '../')
    os.chdir(rootdir)
    web_dir = os.path.join(rootdir, server_dir)
    os.chdir(web_dir)

    Handler = http.server.SimpleHTTPRequestHandler
    httpd = socketserver.TCPServer(("", server_port), Handler)
    print("Serving at port", server_port)
    httpd.serve_forever()

if __name__ == '__main__':
  main()


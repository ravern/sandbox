from flask import Flask, render_template, request
from pymemcache.client.base import Client
import logging, sys

app = Flask(__name__)
mclient = Client(('localhost', 11211))
logging.basicConfig(stream=sys.stderr)

@app.route('/')
def stats():
    stats = mclient.stats()

    get_miss_percentage = "%.3f" % (float(stats["get_misses"]) / float(stats["cmd_get"]) * 100)
    memory_percentage = "%.3f" % (float(stats["bytes"]) / float(stats["limit_maxbytes"]) * 100)

    return render_template("app.html.j2", get_miss_percentage=get_miss_percentage,
                                          memory_percentage=memory_percentage,
                                          stats=stats)

if __name__ == '__main__':
    app.run(host="0.0.0.0", port=5000, debug=True)

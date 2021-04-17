from logging import StreamHandler
from .dlog_python import PythonLogger


class DlogLogger(StreamHandler):
    def __init__(self, api_key):
        StreamHandler.__init__(self)
        self.instance = PythonLogger(api_key)

    def emit(self, record):
        self.instance.log(record.levelno, self.format(record))

    def __del__(self):
        self.instance.clean_up()
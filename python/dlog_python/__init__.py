from logging import StreamHandler as _StreamHandler
from .dlog_python import PythonLogger as _PythonLogger


class DlogLogger(_StreamHandler):
    def __init__(self, api_key):
        _StreamHandler.__init__(self)
        self.instance = _PythonLogger(api_key)

    def emit(self, record):
        self.instance.log(record.levelno, self.format(record))

    def __del__(self):
        self.instance.clean_up()
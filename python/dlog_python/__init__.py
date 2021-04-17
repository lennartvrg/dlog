from logging import StreamHandler as _StreamHandler
from .dlog_python import PythonLogger as _PythonLogger


class DlogLogger(_StreamHandler):
    def __init__(self, api_key):
        _StreamHandler.__init__(self)
        try:
            self.instance = _PythonLogger(api_key)
        except ValueError as ex:
            print(ex)

    def emit(self, record):
        if hasattr(self, 'instance'):
            self.instance.log(record.levelno, self.format(record))

    def close(self):
        if hasattr(self, 'instance'):
            self.instance.clean_up()
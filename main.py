import ctypes
import os
import re

the_lib = ctypes.CDLL("./target/debug/diagnostica.dll")

the_lib.process_diagnostics.argtypes = [ctypes.c_char_p,
                            ctypes.POINTER(ctypes.c_char_p), 
                            ctypes.c_size_t, 
                            ctypes.POINTER(ctypes.c_char_p), 
                            ctypes.c_size_t]

def to_c_string_array(py_list):
    arr = (ctypes.c_char_p * len(py_list))()
    arr[:] = [s.encode("utf-8") for s in py_list]
    return arr

zip_path = ""

for file in os.path.curdir:
    if file.endswith(".zip"):
        zip_path = file
        break

zip_path = zip_path.encode("utf-8")
file_filters = to_c_string_array(["*OLM*"])
file_filter_count = len(file_filters)
log_filters = to_c_string_array([r"\] ERROR"])
log_filter_count = len(log_filters)

result = the_lib.process_diagnostics(zip_path, 
                         file_filters, 
                         file_filter_count, 
                         log_filters, 
                         log_filter_count)

result_str = ctypes.c_char_p(result)
print(result_str)

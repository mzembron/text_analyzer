from text_analyzer import parallel_text_analysis
import streamlit as st
from gui import Gui
import os


# dirname = os.path.dirname(__file__)
# data_filename = os.path.join(dirname, '../../data/sample_long.txt')
# data = ""

# with open(data_filename, 'r') as file:
#     data = file.read()




    # The main file of the application - to run it, just type in
    # scripts directory: `streamlit run main.py`


st.config.dataFrameSerialization = "arrow"
if __name__ == "__main__":
    gui = Gui()
    gui.main()
import streamlit as st
import plotly.express as px
from text_analyzer import parallel_text_analysis
import numpy as np
import pandas as pd
import os

DIRNAME = os.path.dirname(__file__)
DATA_FILENAME = os.path.join(DIRNAME, '../../data/sample_long.txt')

class Gui():
    
    def __init__(self):
        pass

    def main(self):
        self._prepare_widgets()

    def _prepare_widgets(self):
        st.markdown("# Analyze the text")
        self._button = st.button('analyze', on_click=self._analyze_button_callback)

    def _get_text_analysis(self, text_dir):
        data = ""

        with open(text_dir, 'r') as file:
            data = file.read()

        return data

    def _analyze_button_callback(self):
        data = ""
        with open(DATA_FILENAME, 'r') as file:
            data = file.read()
        (word_dict, avg_word_length,  shortest_word) = parallel_text_analysis(data)
        self._display_chart(word_dict)

    def _display_chart(self, word_dict):
        entry_list = np.array(list(word_dict.items()))
        print(entry_list[:,1])
        print(entry_list)
        df1 = pd.DataFrame(dict(word=entry_list[:,0], number_of_occurrences=list(map(int, entry_list[:,1]))))
        # print(type(entry_list[1,1]))
        df1.sort_values(['number_of_occurrences'], inplace=True)
        # df1.reset_index(drop=True)
        print(df1)
        fig = px.bar(df1, x= df1.word, y =df1.number_of_occurrences)
        st.plotly_chart(fig)


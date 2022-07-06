import streamlit as st
import plotly.express as px
from text_analyzer import parallel_text_analysis
import numpy as np
import pandas as pd
import os

DIRNAME = os.path.dirname(__file__)
DATA_FILENAME = os.path.join(DIRNAME, '../../data/sample_long.txt')
DATA_INPUT = os.path.join(DIRNAME, '../../data/input.txt')


class Gui():
    
    def __init__(self):
        pass

    def main(self):
        self._prepare_widgets()
    
    def _prepare_widgets(self):
        file = st.file_uploader("Uploade some txt file", ["txt"])
        
        if(file is not None):
            data = file.read().decode("utf-8")
            with open(DATA_INPUT, 'w') as save_input_file:
                save_input_file.write(data)


        st.markdown("# Analyze the text")
        self._button = st.button('analyze', on_click=self._analyze_button_callback)

    def _get_text_analysis(self, text_dir):
        data = ""

        with open(text_dir, 'r') as file:
            data = file.read()

        return data

    def _analyze_button_callback(self):
        data = ""
        with open(DATA_INPUT, 'r') as file:
            data = file.read()
        if (data is not None):
            (word_dict, avg_word_length,  shortest_word) = parallel_text_analysis(data)
            self._display_chart(word_dict)
        else: 
            st.markdown("## Please upload txt file ")

    def _display_chart(self, word_dict):
        df1 = pd.DataFrame(word_dict.items(), columns = ['word', 'number_of_occurrences'])
        df1 = df1.nlargest(n=10, columns=['number_of_occurrences'])
        df1.sort_values(['number_of_occurrences'], inplace=True)
        fig = px.bar(df1, x= df1.word, y =df1.number_of_occurrences)
        st.plotly_chart(fig)


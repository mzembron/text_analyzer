import streamlit as st
import plotly.express as px
from text_analyzer import parallel_text_analysis
from text_analyzer import sequential_text_analysis
import numpy as np
import pandas as pd
from timeit import default_timer as timer
import os

DIRNAME = os.path.dirname(__file__)
DATA_INPUT = os.path.join(DIRNAME, '../../data/input.txt')


class Gui():
    
    def __init__(self):
        pass

    def main(self):
        self._prepare_widgets()
    
    def _prepare_widgets(self):
        st.markdown("# Analyze the text")
        file = st.file_uploader("Uploade some txt file", ["txt"])
        
        if(file is not None):
            data = file.read().decode("utf-8")
            with open(DATA_INPUT, 'w') as save_input_file:
                save_input_file.write(data)
        self._button = st.button('analyze', on_click=self._analyze_button_callback)

    def _analyze_button_callback(self):
        data = ""
        with open(DATA_INPUT, 'r') as file:
            data = file.read()

        if (data != ""):
            # start_parallel = timer()
            (word_dict, avg_word_length,  shortest_word, longest_word) = parallel_text_analysis(data)
            # end_parallel = timer()
            # print("Computation time of parallel vesrion: " + str(end_parallel - start_parallel))

            # start_sequential = timer()
            (word_dict_ref, avg_word_length_ref,  shortest_word_ref, longest_word_ref) = sequential_text_analysis(data)
            # end_sequential = timer()
            # print("Computation time of sequential vesrion: " + str(end_sequential - start_sequential))

            print("Longest word: " + longest_word + "\nShortest word: " + shortest_word + 
            "\nAvg word length: " + str(avg_word_length))
            self._display_chart(word_dict)
        else: 
            st.markdown("## Please upload txt file ")

    def _display_chart(self, word_dict):
        df1 = pd.DataFrame(word_dict.items(), columns = ['word', 'number_of_occurrences'])
        df1 = df1.nlargest(n=10, columns=['number_of_occurrences'])
        df1.sort_values(['number_of_occurrences'], inplace=True)
        st.plotly_chart(px.bar(df1, x= df1.word, y =df1.number_of_occurrences))


# Capicú Bot

Author: Ashley Leonard

---

## Description

This project is about making a bot to play Puerto Rican Dominoes where 4 players split into 2 teams and try to cause the end of the game to score points based on the pip-totals of the remaining dominoes. Then whichever team reaches a pre-set score (e.g. 100 or 200) after n rounds wins. Currently there is just a bot which plays based on the knowledge of everyone's pieces (which is of course cheating).

A design using a simple neural network trained on solved games using the unfair bot in the works, but there are still some problems that I need to figure out first before I can start on it. To help the neural network, I have created a [domino tracker](https://github.com/Inverted-Square/PythonCapicu) that figures out what dominoes players can or can't have, based on what numbers they and others have skipped on. 

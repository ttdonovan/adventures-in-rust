# Simple Quality

* [https://www.gitbook.com/book/vitiral/simple-quality/details]()

## Purpose:
Write a flash card quizzer from scratch and learn about
quality best practices while doing so.

The example tutorial can be found here:
    http://wiki.openhatch.org/Flash_card_challenge

It should be easy for users to input questions to the
quizzer in a simple and open format. Additionally, the
quizzer should use effective memorization techniques. Some possible ideas
include:

- asking items in a random order
- telling the correct answer after the user anwsers incorrectly
- asking items more often if they were answered incorrectly
- allowing users to configure time limits, so they can compare results between quizzes

## High Level Design

### Execution Method

The minimum viable product shall be a command line utility
that is given the path to one or more question files as
arguments

Additional arguments will include:
- `-t`: specify the time allowed for each question
- `-T`: specify the total time allowed for the whole quiz
- `-r NUM`: repeat questions only a certain number of times.
    By default there is no limit

The program will ask one question at a time, recording how
many answers the user got correct/incorrect and weighting
future questions accordingly.

When the program is complete it will report:
- time taken, broken up by whole quiz and each question
- the user's score

### Question File Format
The user shall be able to easily configure the quiz
questions through a simple csv format consisting of two
columns: the question and the answer.

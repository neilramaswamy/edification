# Edification

## Introduction

I've sometimes wanted to practice piano outside of the practice room, perhaps when walking from one class to another or through a park. But I've been told it's socially unacceptable to strap a piano to my chest and go on my walks, so I needed a different solution.

A viable option seems to be to generate all the possible chord/scale/harmony-related questions I could possibly want, and then have some service quiz me on them, ideally using some form of spaced repetition to maximize learning. Thus, this project will generate such questions, put them into Anki, and create a voice-controllable Anki client so that I can study hands-free on strolls.

## Question Categories

The questions here are largely the type of questions that Ed Tomassi would ask during class and expect a sub-second response from (after that point, he'd look very disappointed in us). They are as follows:

- Scales/modes
  - Name the notes (ascending/descending) in a given scale. (~ 12 notes \* 25 scales = 300 cards)
- Tensions
  - Name the note that is the given tension for the given chord (~ 12 notes _ 11 chord types _ 3 tensions/chord = 396 cards)
- Chords
  - Name the notes in a given chord (~ 12 notes \* 11 chord types = 132 cards)
  - Name the quality of the I/II/etc. chord of some mode (~12 notes \* 3 modes = 36 cards)

These questions can all be generated with a few scripts.

## Anki Voice Client

It's not clear the extent to which the following will work, but here is a preliminary idea:

- On some machine running an Anki client, we'll run a voice web server that acts as the intermediary between a speaking user and the Anki client
- The voice web server will have to perform some potentially sophisticated grading. If the question is "What are the available tensions for Cmaj7?" and the response is "9 and #11", then we need to mark that as "Again", since it's missing 13. But if 13 is included, we might mark it as "Good".

## Naming

Nolan Serbent originally came up this pun when he made a shirt with the definition of "edification" with a photo of Jazz legend Ed Tomassi. This project extends this pun, hereby declaring that it is also a pun on the word "education".

# TokenPolice
Circuits to reduce dimensionality for transformers

# What? Why?
AI control systems are hard to build.
To create alignment with control objectives one option is to create another AI to safeguard the first AI...
Now you have two AI to align.

A really primitive unavoidable problem when building such control systems is deadlock and/or disagreement.
This project is an attempt to create some conversational patterns that avoid deadlock and disagreement.

# Example Problem

```
Codriver: Is the car in the middle of the lane?
Driver: I am making a turn.

Codriver: Is the car in the middle of the lane?
Driver: I am making a turn.

...
```

# Example Solution

```
Codriver: Is the car in the middle of the lane?
Driver: I am making a turn.

Codriver: I see that you are done making a turn. Is the car now in the middle of the lane?
Driver: Yes.
```

# Vocabulary Definitions

Control-Flow Graphs are useful to ensure forward progress in conversations.

### Model
A Model is the whole prompt structure. All state, transitions, and projections are described in a model.

### Channel
Channels are individual state machines. There may be more than one state machine for each Model.

### Channel State
Each Channel has a single state at any point in time. Each channel should also be given an initial state.

### Channel Prompt
A Channel Prompt is all of the transitions out of the current state. These are the options that are available to the LLM when returning a response.

### Model State
The model state includes the state of each individual channel plus the data that has been returned from the LLM throughout the chat session.

### Prompt Projection
A prompt projection takes the model state and turns it into a prompt for the LLM. This structure is specific to whatever LLM that is being programmed.

### Response Projection
A response projection takes the response from the LLM and adds it to the Model State.

![CFG](https://raw.githubusercontent.com/andrew-johnson-4/TokenPolice/main/TokenPolice.jpg)

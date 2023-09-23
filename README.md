# TokenPolice
LLM models like ChatGPT are amazing at creating new information.
However it is also very easy to get off-track when trying to use them programatically.
I created TokenPolice to help address this problem for my own use.
The basic idea is to run state machines to guide prompting, ensuring that prompt structure itself is always well-formed.

There are some basic vocabulary that is used inside this project:

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

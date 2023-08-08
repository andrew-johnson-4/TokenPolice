# TokenPolice
Circuits to reduce dimensionality for transformers

# What? Why?
AI control systems are hard to build.
To create alignment with control objectives one option is to create another AI to safeguard the first AI...
Now you have two AI to align.

A really primitive unavoidable problem when building such control systems is deadlock and/or disagreement.
This project is an attempt to create some conversational patterns that avoid deadlock and disagreement.

# Example

```
Codriver: Is the car in the middle of the lane?
Driver: I am making a turn.

Codriver: Is the car in the middle of the lane?
Driver: I am making a turn.

...
```

# Proposals

Control-Flow Graphs are useful to ensure forward progress in conversations.

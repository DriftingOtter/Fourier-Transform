# Fourier Transform 

> A Rust implmentation & interpretation.

## What is Fourier Transform?
---

A Fourier transformation is a mathematical function that can sepearte a inputed wave function into its consitituante parts, by performing different operations on different wave forms that combine to make the fundamental wave form.

### Signal Decomposition
---

When sampling a certain signal, if you find the derivative of that function to the time domain, you can determine the fundamental frequencies that make that signal by looking at the maximum extrema on a graph.

This fundamental concept helps us understand that properties of a discrete signal that we are trying to transform from the continuous world to the discrete.

These fundamental frequencies highlight the property that the superposition of the fundamental frequencies is the same as the superposition of the double derivated values of the orignal signal (the derivation of the fundamental frequencies*).

```
F_s1 + F_s2 = 'F_s1 + 'F_s2
```



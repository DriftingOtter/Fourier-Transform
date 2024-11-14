# Fourier Transform
---

> A Rust implmentation & interpretation.

## What is Fourier Transform?
---

A Fourier transformation is a mathematical function that can separate an input wave function into its constituent parts by performing different operations on different waveforms that combine to make the fundamental waveform.

### Signal Decomposition
---

When sampling a specific signal, if you find the derivative of that function to the time domain, you can determine the fundamental frequencies that make that signal by looking at the maximum extrema on a graph of the magnitude of x-values from the Fourier transform.

This fundamental concept helps us understand the properties of a discrete signal that we are trying to transform from the continuous world to the discrete.

These fundamental frequencies highlight the property that the superposition of the fundamental frequencies is the same as the superposition of the double derivated values of the original signal (the derivation of the fundamental frequencies*).

```
F_s1 + F_s2 = 'F_s1 + 'F_s2
```

## Formula Break-down
---

```
F(ω) = ∫ f(t) * e^(-iωt) dt ; (-∞ to ∞)
```

- where:
    
    + ω is the angular frequency (often 2πf where f is frequency),
    
    + ```e^(-iωt)``` or ```(cos(ωt) - i sin(ωt))``` represents oscillations that match the frequencies in f(t).

### Converting e^(-iωt) from Euler's Form to Polar Form
---

To convert ```e^(-iωt)``` from Euler's form to polar form, we use Euler's formula:

```
    e^(iθ) = cos(θ) + i sin(θ)
```

For e^(-iωt), where θ = -ωt, we can substitute:

```
e^(-iωt) = cos(-ωt) + i sin(-ωt)
```

### Why Angular Frequency, instead of Linear Frequency?
---

##### 1. **Linear Frequency (f)**
---

- **Definition**: Linear frequency f (measured in Hertz) is the number of cycles a wave completes per second.

- **Period**:
  - The period T, the time for one cycle, is given by ```T = 1 / F```.

  - Higher frequency results in a shorter period, meaning more cycles per second.

- **Example**:
  - For ```f = 2Hz```, the period ```T = 1/2 = 0.5 seconds```.

##### 2. **Angular Frequency (ω)**
---

- **Definition**: Angular frequency ω measures the rate of phase change over time in radians per second.

- **Conversion**: Angular frequency relates to linear frequency by ```ω = 2 * PI * f```.

- **Period (in terms of ω)**:
  - Period ```T = 2 PI / ω```, so higher angular frequency leads to a shorter period, increasing oscillations.
  
#### 3. **Impact on the Sine Wave's Domain and Period**
---

- **Linear Frequency (f)**: The formula ```y = sin(2 * PI * f * x)``` uses ```(2 * PI * f)``` to convert cycles into radians.

  - Increasing f compresses the wave horizontally, adding more oscillations within the same domain.
  
- **Angular Frequency (ω)**: The formula ```y = sin(ω * f)``` directly scales the wave's period.

  - Higher ω increases oscillations per unit of x, compressing the wave horizontally.

#### 4. **Phase Shift (PHI) and Its Effects on Waves**
---

- **Definition**: Phase shift is a horizontal displacement, altering where the wave begins without changing its frequency or amplitude.

- **Formula with Phase Shift**: ```y = sin(2 * PI * f * x + PHI)```

  - **Positive** __PHI__ shifts the wave left.
  - **Negative** __PHI__ shifts the wave right.
  
- **Practical Impact**:

  - Phase shifts affect interference, e.g., constructive (in-phase) or destructive (out-of-phase).

  - Graphically, phase shift adjusts the wave's start point along the x-axis.

## Why Angular Frequency is Preferred in DSP
---

1. **Radians Compatibility**:

   - ```ω = 2 * PI * f``` aligns naturally with radians, simplifying trigonometric functions (sine/cosine) and avoiding extra conversions.
   
2. **Fourier Transform and Complex Exponentials**:

   - Fourier transform decomposes signals into components like ```e^i * ω * t```, where ω in radians per second integrates seamlessly with DSP mathematics.
   
3. **Discrete Sampling**:

   - For sampled signals, angular frequency provides a phase change per sample, fitting digital signal processing's sample-based approach.

   - Normalized angular frequency helps DSP work within a manageable range ```[0, PI]``` for periodicity and symmetry.

4. **Filter Design**:

   - In DSP, filter design relies on a consistent representation of phase response with angular frequency, allowing smoother calculations of phase and frequency responses.


## Coding the Fourier Transform
---

To program a version of the formula that can be utilized for digitally stored signals, we will use the discrete approximation called the Discrete Fourier Transform (DFT). 

- Here is the general approach:

    1. **Define Input**: Sample the input signal f(t) at discrete points f[n] to represent a time-domain signal in array form.

    2. **Loop Through Frequencies**: For each frequency bin k (usually 0 to N-1 for N samples), calculate the summation of f[n] * e^(-2πi k n / N) for each sample point n.

    3. **Return the Result**: Store these results in a complex array, representing the amplitude and phase of each frequency component.

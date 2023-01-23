## Courant Number Derivation for Heat Transfer

The courant number for heat transfer in 1D can be shown as follows:

$$Co = \frac{\alpha t_{timestep}}{L_{mesh}^2}$$

in 2D it can be [shown](https://skill-lync.com/student-projects/Analysis-of-Solution-Stability-in-a-2D-Heat-conduction-problem-08510)
as:


$$Co = \frac{\alpha t_{timestep}}{x_{mesh}^2} +
\frac{\alpha t_{timestep}}{y_{mesh}^2}$$

The maximum Courant number allowable here is not 1 but 0.25.


Now the courant number is a little harder to imagine for heat transfer,
unlike in fluid mechanics where we have:

$$Co_{fluid} = \frac{u \Delta t_{timestep}}{\Delta x}$$
or
$$Co_{fluid} = \frac{\dot{V} \Delta t_{timestep}}{V_{control\_volume}}$$

This is in the 1D case. 

Essentially, if the volumetric flowrate or mass flowrate is enough to
"flush" out all the control volume in one timestep, the Courant number
is too big.

For heat transfer, this intuition is not as obvious.

## Courant Number in releation to Fundamental Equations

Courant Number is used for transient heat transfer where in 1D:

$$m c_p \frac{\partial T}{\partial t} = 
-kA_{normal} \frac{\partial T}{\partial x}$$

This assumes that there is a control volume with one boundary of 
conduction heat transfer

$$\rho V_{control\_volume} c_p \frac{\partial T}{\partial t} = 
-kA_{normal} \frac{\partial T}{\partial x}$$

$$\frac{\partial T}{\partial t} = 
-\alpha \frac{A_{normal}}{V_{control\_volume}} \frac{\partial  T}{\partial x}$$



we note that the mesh size in x direction is:

$$x_{mesh} = \frac{V_{control\_volume}}{A_{normal}}$$

We could nondimensionalise the time with the timestep and x with
the mesh size in x direction

$$\frac{\partial T}{\partial t^*} = 
-\alpha \frac{t_{timestep}}{x_{mesh}^2} \frac{\partial  T}{\partial x^*}$$


The courant number then appears as:


$$Co = \frac{\alpha t_{timestep}}{x_{mesh}^2}$$

where:
$$x_{mesh} = \frac{V_{control\_volume}}{A_{normal}}$$

$$\frac{\partial T}{\partial t^*} = 
-Co \frac{\partial  T}{\partial x^*}$$

And it just so happens to have the same dimensions as the Fourier Number.

Now suppose we have a convection boundary condition, the Biot number comes
into play

$$m c_p \frac{\partial T}{\partial t} = 
-hA_{surface} (T_{surface} - T_{fluid})$$

$$\rho V_{control\_volume} c_p \frac{\partial T}{\partial t} = 
-hA_{surface} (T_{surface} - T_{fluid})$$


$$ \frac{\partial T}{\partial t} = 
\frac{-hA_{surface}}{\rho V_{control\_volume} c_p} (T_{surface} - T_{fluid})$$


we can define the lengthscale as volume to surface ratio:

$$L = \frac{V_{control\_volume}}{A_{surface}}$$


$$ \frac{\partial T}{\partial t} = 
\frac{-h}{\rho L c_p} (T_{surface} - T_{fluid})$$

Multiply top and bottom by conductivity of the control volume
($k_{solid}$),


$$ \frac{\partial T}{\partial t} = 
\frac{-h k}{\rho L k  c_p} (T_{surface} - T_{fluid})$$

$$ \frac{\partial T}{\partial t} = 
\frac{-h \alpha}{ L k  } (T_{surface} - T_{fluid})$$

nondimensionalising time and bringing out the fourier number,


$$ \frac{\partial T}{\partial t^*} = 
\frac{-h L \alpha t_{timescale}}{k L^2   } (T_{surface} - T_{fluid})$$

$$ \frac{\partial T}{\partial t^*} = 
-Bi Fo (T_{surface} - T_{fluid})$$

So in this case, the Courant number equivalent becomes the product
of
$$Co = Bi Fo$$

So how do we determine if something is table?

We can start with integration to get a sense of things

Let's integrate the dimensional form since it's more intuitive 
to integrate:
$$ \frac{d T}{d t} = 
\frac{-h \alpha}{ L k  } (T_{surface} - T_{fluid})$$

We assume $T_{surface} = T_{control\_volume}$ as for control volume,
lumped capacitance is always the case

$$ \frac{d (T_{cv} - T_{fluid})}{d t} = 
\frac{-h \alpha}{ L k  } (T_{cv} - T_{fluid})$$


$$ d\ \ln (T_{cv} - T_{fluid}) = 
\frac{-h \alpha}{ L k  } dt$$

we integrate from t =0  to t= $t_{elapsed}$


$$  (T_{cv}(t_{elapsed}) - T_{fluid})/(T_{cv\_initial} - T_{fluid})= 
\exp \left( \frac{-h \alpha}{ L k  } t_{elapsed} \right)$$

$$\theta = (T_{cv}(t_{elapsed}) - T_{fluid})/(T_{cv\_initial} - T_{fluid})$$
So the maximum temperature change that can happen is 

$$T_{cv\_initial} - T_{fluid}$$

So assuming that we have this change go to completion,


$$\exp \left(- \frac{h \alpha}{ L k  } t_{elapsed} \right) \approx 0$$

If we nondimensionalise 

$$t^* = \frac{t_{elapsed}}{t_{steady\_state}}$$

Where $t_{steady\_state}$ is the time taken for the heat transfer to reach 
completion, so that 

$$t^* = order\ of\ magnitude\ (1)$$

we can write,

$$ \exp \left(- Bi Fo\ t^* \right) \approx 0$$

I'll just define a threshold for steady state as:

$$\exp(- Bi\ Fo\ t^*) = 1 - 0.999 = 1e-3$$

take ln both sides,


$$- Bi\ Fo\ t^* = -6.9077$$

For a nice round easy to remember number,
$$ Bi\ Fo\ t^* \approx 7$$

This is the uppermost upper limit of the courant number defined
as (BiFo), as in the product of both cannot exceed 7, if not, the
heat transfer sort of "overshoots".

If the timestep is so big that it causes this, the simulation will surely
be unstable. If this Bi Fo becomes 7, just assume steady state happens instantly.

However, when it comes to timestepping, we know the gradient changes
at every timestep because the temperature difference becomes smaller.

So let's assume that the solver guesses the gradient of the change in 
$\theta$ with respect to time is linear, and for the timestep itself,
it assumes that the rate of change of temperature with respect to time
is constant. This would result in a linear relationship.

The gradient at t = 0 is simply -Bi Fo, 

the predicted evolution of temperature assuming a constant 
rate of change of temperature is:

$$\theta = 1 - Bi \ Fo \ t^*$$

If we were to plot the graphs of $\theta$ vs Bi Fo t*, and one graph
has exp(- Bi Fo t*) and the other has 1 - Bi Fo t*,

Then we would notice that their gradients diverge readily at Bi Fo = 0.25. 
So when Bi Fo  = 0.25, we would want the solver to re-evaluate the temperature
gradient to keep up with the changes. This is probably why we do not want the
courant number to be greater than 0.25. In fact Co = 0.1 to 0.2 is ideal so that
we can keep track of temperature changes accurately.

In the same way, Co should be 0.25 or less for the conduction heat flux case.
That's why Co should be 0.25 or less.

Again, to find Bi,

$$Bi = \frac{h_{fluid}L_{volume-to-surface-area-ratio}}{k_{solid}}$$

All other properties for $\alpha$ are solid thermophysical properties,
only the fluid heat transfer coefficient is a property of the fluid.

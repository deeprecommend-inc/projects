import numpy as np
import matplotlib.pyplot as plt
import matplotlib.animation as animation

# Set up the figure, axis, and plot element
fig, ax = plt.subplots()
ax.set_xlim(0, 2 * np.pi)
ax.set_ylim(-1.5, 1.5)
line, = ax.plot([], [], lw=2)

# Initialization function: plot the background of each frame
def init():
    line.set_data([], [])
    return line,

# Animation function: this is called sequentially
def animate(i):
    x = np.linspace(0, 2 * np.pi, 1000)
    y = np.sin(x - 0.05 * i)
    line.set_data(x, y)
    return line,

# Call the animator
anim = animation.FuncAnimation(fig, animate, init_func=init,
                               frames=200, interval=20, blit=True)

# Save the animation as an mp4 file
Writer = animation.writers['ffmpeg']
writer = Writer(fps=30, metadata=dict(artist='Me'), bitrate=1800)
anim.save('sine_wave_animation.mp4', writer=writer)

# Display the animation
plt.show()
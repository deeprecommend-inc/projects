import matplotlib.pyplot as plt
import matplotlib.animation as animation
import numpy as np

def fibonacci():
    a, b = 0, 1
    while True:
        yield a
        a, b = b, a + b

fig, ax = plt.subplots(figsize=(10, 10))
ax.set_xlim(-1, 1)
ax.set_ylim(-1, 1)
ax.axis('off')

circles = []
colors = plt.cm.rainbow(np.linspace(0, 1, 20))
fib = fibonacci()

def init():
    ax.set_title("Fibonacci Sequence as Colorful Circles")
    return []

def animate(i):
    num = next(fib)
    size = np.sqrt(num) / 50  # スケーリングして適切なサイズに調整
    circle = plt.Circle((0, 0), size, color=colors[i], alpha=0.7)
    ax.add_artist(circle)
    circles.append(circle)
    
    for j, c in enumerate(circles):
        c.center = (np.cos(j*np.pi/10)*0.7, np.sin(j*np.pi/10)*0.7)
    
    ax.set_title(f"Fibonacci Sequence as Colorful Circles\nCurrent number: {num}")
    return circles

ani = animation.FuncAnimation(fig, animate, init_func=init, frames=20, interval=500, blit=True, repeat=False)

# Save the animation as an mp4 file
Writer = animation.writers['ffmpeg']
writer = Writer(fps=30, metadata=dict(artist='Me'), bitrate=1800)
ani.save('fibonacci.mp4', writer=writer)

plt.show()

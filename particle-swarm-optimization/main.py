import math

import matplotlib.pyplot as plt
import numpy as np

from PSO import PSO
from Particle import Particle


def fitness_function_plot(x1, x2):
    return fitness_function([x1, x2])


# def fitness_function(params):
#     x1 = params[0]
#     x2 = params[1]
#
#     return (-((x2 + 47) * math.sin(math.sqrt(abs((x1 / 2) + (x2 + 47)))))) - \
#            (x1 * math.sin(math.sqrt(abs(x1 - (x2 + 47)))))

def fitness_function(params):
    x1 = params[0]
    x2 = params[1]

    return ((x1**2 + x2 - 11)**2) + ((x1 + x2**2 - 7)**2)


x = np.linspace(-5, 5, 50)
y = np.linspace(-5, 5, 50)

X, Y = np.meshgrid(x, y)
Z = np.vectorize(fitness_function_plot)(X, Y)

fig = plt.figure(dpi=400)
ax = plt.axes(projection='3d')

# surf = ax.plot_surface(X, Y, Z, rstride=1, cstride=1,
#                            cmap='viridis', edgecolor='none')
# fig.colorbar(surf, shrink=0.5, aspect=5)


def plot_swarm(swarm: list[Particle], t: int):
    # ax.plot_surface(X, Y, Z, rstride=1, cstride=1,
    #                        cmap='viridis', edgecolor='none')

    ax.set_xlabel('x')
    ax.set_ylabel('y')
    ax.set_zlabel('z')

    ax.plot_wireframe(X, Y, Z, rstride=20, cstride=20, linewidth=1)

    for particle in swarm:
        ax.scatter([particle.positions[0]], [particle.positions[1]],
                   [particle.fitness], plotnonfinite=True, zorder=10, s=3)

    fig.suptitle(f"Iteration {t}")
    fig.savefig(f"images/iteration_{t}.png", dpi=400)
    plt.cla()
    # plt.show()


if __name__ == '__main__':
    positions_bounds = [[-5.0, 5.0], [-5.0, 5.0]]
    velocities_bounds = [[-2.0, 2.0], [-2.0, 2.0]]

    pso = PSO(use_weight_decay=True, swarm_size=20,
              positions_bounds=positions_bounds,
              velocities_bounds=velocities_bounds,
              fitness_function=fitness_function)

    pso.optimize(50, maximize=True, plot_progress=plot_swarm)

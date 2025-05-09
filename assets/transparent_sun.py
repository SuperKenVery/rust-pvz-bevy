from PIL import Image
import numpy as np

sun = np.array(Image.open("Sun.png")) # (80, 80, 4)

R, G, B, A = sun[:,:,0], sun[:,:,1], sun[:,:,2], sun[:,:,3]

white = np.logical_and(R>=250, np.logical_and(G>=250, B>=250))
sun[white, 3] = 0

transparent = Image.fromarray(sun)
transparent.save("Sun_transparent_background.png")


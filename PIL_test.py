from PIL import Image

im = Image.open("kite_flyer.JPG")
print(im.size)

r, g, b = im.split()
im = Image.merge("RGB", (b, g, r))
im.show()

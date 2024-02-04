from PIL import Image, ExifTags
import os
import matplotlib.pyplot as plt

descTag = ExifTags.Base.ImageDescription.value
count = 0
for fileName in os.listdir("images"):
    count += 1
    im = Image.open("images/"+fileName)
    try:
        currentExif = im.getexif()
        description = currentExif[descTag]
    except KeyError:
        currentExif = Image.Exif()
        description = ""
    plt.imshow(im)
    plt.ion()
    plt.show()
    print(f"{count} Filename: {fileName}\t\t Current caption: {description}")
    plt.pause(1)
    currentInput = input("Enter a new caption or press enter to keep: ")
    if currentInput != "":
        currentExif[descTag] = currentInput
    fileName = fileName.replace(" ", "-")
    im.save("images/"+fileName, exif=currentExif)
    plt.clf()
    im.close()
    print("\n\n\n")

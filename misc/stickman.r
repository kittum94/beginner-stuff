library("ggplot2")
library("plotrix")

print("stickman program")

x <- c(1, 2, 3, 2, 2, 3, 1, 2, 2, 2)
y <- c(1, 2, 1, 2, 3, 3, 3, 3, 4, 5)

plot(x, y, type='b')
draw.circle(2, 5, 0.5)
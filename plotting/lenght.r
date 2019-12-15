# read file into r
data <- read.csv(file="result.csv", header=FALSE, sep=",")
# filename for result
png(file = "result.png", width=1920,height=1080)

plot(x = data$V1,y = data$V2,
	xlab = "number",
	ylab = "lenght",
	xlim = c(0,100000),
	ylim = c(0,350),
	col=rgb(255, 0, 0, maxColorValue=255),
)

	 
# Save the file.
dev.off()

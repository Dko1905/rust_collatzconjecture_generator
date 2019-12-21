# Get input filename
inputFilename <- commandArgs(trailingOnly = TRUE)[1]
# Get output filename
outputFilename <- commandArgs(trailingOnly = TRUE)[2]

# read file into r
data <- read.csv(file=inputFilename, header=FALSE, sep=",")
# filename for result
png(file = outputFilename, width=1080,height=1080)

plot(x = data$V1,y = data$V2,
	xlab = "number",
	ylab = "lenght",

	col=rgb(255, 0, 0, maxColorValue=255),
)

	 
# Save the file.
dev.off()

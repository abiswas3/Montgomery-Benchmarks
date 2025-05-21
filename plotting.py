import sys
import pandas as pd
import matplotlib.pyplot as plt

# Load your CSV
df = pd.read_csv(f"{sys.argv[1]}")  # Replace with your actual CSV path
print(df.mean())
# Create a boxplot
ax = df.boxplot(column=["G-mult", "Y-mult"])

# Customize and show
ax.set_title("G-mult vs Y-mult")
ax.set_ylabel("Time (seconds)")
ax.set_yscale("log")
plt.grid(True)
plt.show()

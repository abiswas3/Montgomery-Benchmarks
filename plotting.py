import sys
import pandas as pd
import matplotlib.pyplot as plt

# Load your CSV
df = pd.read_csv(f"{sys.argv[1]}")  # Replace with your actual CSV path
# Compute average percentage advantage
avg_g = df["G-mult"].mean()
avg_y = df["Y-mult"].mean()
advantage = ((avg_g - avg_y) / avg_g) * 100
print(f"Average Advantage: {advantage}")
print(df.mean())
# Create a boxplot
ax = df.boxplot(column=["G-mult", "Y-mult"])

# Customize and show
ax.set_title("G-mult vs Y-mult averaged over 10_000 trials with 1000 chained multiplications")
ax.set_ylabel("Time (nano-seconds)")
# ax.set_yscale("log")
plt.grid(True)
plt.tight_layout()
plt.show()

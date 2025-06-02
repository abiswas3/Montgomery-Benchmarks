import sys
import pandas as pd
import matplotlib.pyplot as plt

# Load your CSV
print(f"Scanning file: {sys.argv[1]}")
df = pd.read_csv(f"{sys.argv[1]}")  # Replace with your actual CSV path
# Compute average percentage advantage
avg_g = df["C-mult"].mean()
avg_y = df["H-mult"].mean()
advantage = ((avg_g - avg_y) / avg_g) * 100
if advantage < 0:
    color = "\033[91m"  # red
elif advantage > 0:
    color = "\033[92m"  # green
else:
    color = "\033[0m"   # default
reset = "\033[0m"

print(f"{color}Average Advantage (%): {advantage}{reset}")
print(df.mean())
# # Create a boxplot
# ax = df.boxplot(column=["G-mult", "Y-mult"])
#
# # Customize and show
# ax.set_title("G-mult vs Y-mult averaged over 10_000 trials with 1000 chained multiplications")
# ax.set_ylabel("Time (nano-seconds)")
# # ax.set_yscale("log")
# plt.grid(True)
# plt.tight_layout()
# plt.show()

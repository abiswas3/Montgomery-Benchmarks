
#!/bin/bash

FILE="$1"

if [[ ! -f "$FILE" ]]; then
  echo "Usage: $0 <assembly_file>"
  exit 1
fi

# Filter out:
#  - empty lines
#  - comment lines starting with # or ;
#  - labels ending with a colon ':'
# Then get the first word of each remaining line (the opcode)
opcodes=$(grep -vE '^\s*($|#|;|.*:)$' "$FILE" | awk '{print $1}')

# Count total instructions (non-label, non-empty, non-comment lines)
total_instructions=$(echo "$opcodes" | wc -l)

# Count opcode frequencies and sum them
echo "Opcode counts:"
opcode_counts=$(echo "$opcodes" | sort | uniq -c | sort -nr)
echo "$opcode_counts"

# Calculate sum of opcode counts
sum_of_counts=$(echo "$opcode_counts" | awk '{sum += $1} END {print sum}')

echo
echo "Total instruction lines (excluding labels/comments): $total_instructions"
echo "Unique opcodes: $(echo "$opcodes" | sort | uniq | wc -l)"

if [[ "$sum_of_counts" -eq "$total_instructions" ]]; then
  echo "✅ Sum of opcode counts matches total instruction lines."
else
  echo "❌ Mismatch! Sum of opcode counts ($sum_of_counts) does NOT match total instructions ($total_instructions)."
fi

echo "Unique opcodes: $(echo "$opcodes" | sort | uniq | wc -l)"

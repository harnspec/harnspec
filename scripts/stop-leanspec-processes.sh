#!/usr/bin/env bash
# stop-harnspec-processes.sh
# Stops all running harnspec and harnspec-mcp processes
# Useful before rebuilding or when processes are stuck

echo "🔍 Looking for harnspec processes..."

# Find all harnspec related processes (excluding this script and grep)
PIDS=$(ps aux | grep -E "harnspec|harnspec-mcp" | grep -v grep | grep -v "stop-harnspec-processes" | awk '{print $2}')

if [ -z "$PIDS" ]; then
  echo "✅ No harnspec processes found"
  exit 0
fi

echo "🛑 Found processes to stop:"
ps aux | grep -E "harnspec|harnspec-mcp" | grep -v grep | grep -v "stop-harnspec-processes"
echo ""

for PID in $PIDS; do
  echo "  Killing PID $PID..."
  kill "$PID" 2>/dev/null || true
done

# Wait a moment for processes to exit
sleep 1

# Check if any survived
REMAINING=$(ps aux | grep -E "harnspec|harnspec-mcp" | grep -v grep | grep -v "stop-harnspec-processes" | wc -l)

if [ "$REMAINING" -gt 0 ]; then
  echo "⚠️  Some processes didn't stop, force killing..."
  for PID in $PIDS; do
    kill -9 "$PID" 2>/dev/null || true
  done
fi

echo "✅ All harnspec processes stopped"

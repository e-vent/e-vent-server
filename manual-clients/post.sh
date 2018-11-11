#!/bin/bash
echo "What's the name?"
read name

echo "What's the description?"
read desc

echo "What's the background?"
read bg

BODY="{\"name\":\"$name\",\"desc\":\"$desc\",\"bg\":\"$bg\"}"
echo $BODY

curl -H "Content-Type: application/json" -d "$BODY" 127.0.0.1:8000/events

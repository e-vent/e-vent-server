#!/bin/bash
echo Which event to get?
read id
curl 127.0.0.1:8000/events/$id

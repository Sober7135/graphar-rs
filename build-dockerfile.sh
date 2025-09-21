#!/bin/bash
docker buildx build . --network host -t graphar-rs # --no-cache

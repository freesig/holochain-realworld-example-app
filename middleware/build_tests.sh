#!/bin/bash
single_source code docs/ws.md src/lib/ws/tests.rs rust test
single_source code docs/connect.md tests/tests.rs rust test
single_source md docs/ws.md docs/docs/ws.md
single_source md docs/connect.md docs/docs/connect.md

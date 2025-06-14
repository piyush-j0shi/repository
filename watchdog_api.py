import os
from os.path import basename, exists
from re import L
from typing import reveal_type 
from fastapi import FastAPI, Query, HTTPException
from fastapi.responses import PlainTextResponse
from watch_dogs import restore_file, LOG_FILE

app = FastAPI(title = "watchdogs")

@app.get("/logs", response_class = PlainTextResponse)
def get_logs():
    if not os.path.exists(LOG_FILE):
        return "no logs yet"

    with open(LOG_FILE, "r") as f:
        return f.read()

@app.get("/status")
def get_status():
    return {
            "status" : "watchdog is running (if watchdog runner is running)"    
        }

@app.post("/recover")
def recover_file(file : str = Query(..., description = "path to file to recover")):
    if not os.path.exists(f".trash_bin/{os.path.basename(file)}"):
        raise HTTPException(
                    status_code = 404,
                    detail = "no backup found"
                )

    restore_file(file)
    return {
            "message" : f"{file} restored from backup"
        }


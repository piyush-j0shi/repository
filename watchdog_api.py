from fastapi import FastAPI, Query, HTTPException, UploadFile, File, Header
from fastapi.responses import PlainTextResponse
import os
import shutil

from watch_dogs import restore_file, LOG_FILE, BACKUP_DIR, cleanup_old_backups, log

app = FastAPI(title="Watchdog Monitor API")

SECRET_TOKEN = "supersecret"  

def verify_token(token: str = Header(..., alias="X-API-Key")):
    if token != SECRET_TOKEN:
        raise HTTPException(status_code=403, detail="Invalid API Key")

@app.get("/logs", response_class=PlainTextResponse)
def get_logs():
    if not os.path.exists(LOG_FILE):
        return "No logs yet."
    with open(LOG_FILE, "r") as f:
        return f.read()

@app.get("/status")
def get_status():
    return {"status": "watchdog is running (if watchdog_runner is active)"}

@app.post("/recover")
def recover_file(file: str = Query(...), token: str = Header(..., alias="X-API-Key")):
    verify_token(token)
    if not os.path.exists(f".trash_bin/{os.path.basename(file)}"):
        raise HTTPException(status_code=404, detail="No backup found.")
    restore_file(file)
    return {"message": f"{file} restored from backup"}

@app.post("/upload")
def upload_file(file: UploadFile = File(...), token: str = Header(..., alias="X-API-Key")):
    verify_token(token)
    save_path = os.path.join(".", file.filename)
    with open(save_path, "wb") as f:
        shutil.copyfileobj(file.file, f)
    log("UPLOAD", f"{file.filename} uploaded via API")
    return {"message": f"{file.filename} uploaded"}

@app.post("/cleanup")
def trigger_cleanup(token: str = Header(..., alias="X-API-Key")):
    verify_token(token)
    cleanup_old_backups()
    return {"message": "Old backups cleaned up"}


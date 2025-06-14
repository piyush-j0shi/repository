import os
import shutil
import datetime
from watchdog.events import FileSystemEventHandler

MONITORED_EXTENSIONS = ['.txt', '.py', '.json']
BACKUP_DIR = ".trash_bin"
LOG_FILE = "watchdog.log"

def timestamp():
    return datetime.datetime.now().strftime("%Y-%m-%d %H:%M:%S")

def log(label, message):
    entry = f"[{label}] {timestamp()} — {message}"
    print(entry)
    with open(LOG_FILE, "a") as f:
        f.write(entry + "\n")

def backup_file(path):
    if not os.path.exists(BACKUP_DIR):
        os.makedirs(BACKUP_DIR)

    if os.path.isfile(path):
        base = os.path.basename(path)
        backup_path = os.path.join(BACKUP_DIR, base)

        if os.path.abspath(path) == os.path.abspath(backup_path):
            return  

        shutil.copy2(path, backup_path)
        log("BACKUP", f"{base} → {backup_path}")

# let's add a function for 7 days
# def cleanup_old_backups(days=7):
#     if not os.path.exists(BACKUP_DIR):
#         return

#     now = datetime.datetime.now()
#     cutoff = now - datetime.timedelta(days=days)

#     for file in os.listdir(BACKUP_DIR):
#         path = os.path.join(BACKUP_DIR, file)
#         if os.path.isfile(path):
#             mtime = datetime.datetime.fromtimestamp(os.path.getmtime(path))
#             if mtime < cutoff:
#                 os.remove(path)
#                 log("CLEANUP", f"Removed old backup: {file}")

def cleanup_old_backups(minutes=3):
    if not os.path.exists(BACKUP_DIR):
        return

    now = datetime.datetime.now()
    cutoff = now - datetime.timedelta(minutes=minutes)

    for file in os.listdir(BACKUP_DIR):
        path = os.path.join(BACKUP_DIR, file)
        if os.path.isfile(path):
            mtime = datetime.datetime.fromtimestamp(os.path.getmtime(path))
            if mtime < cutoff:
                os.remove(path)
                log("CLEANUP", f"Removed old backup: {file}")

def restore_file(path):
    base = os.path.basename(path)
    backup_path = os.path.join(BACKUP_DIR, base)
    if os.path.exists(backup_path):
        shutil.copy2(backup_path, path)
        log("RECOVERED", f"{path} restored from backup")
    else:
        log("FAILED", f"No backup found for {path}")

class CustomHandler(FileSystemEventHandler):

    def should_handle(self, path):
        if BACKUP_DIR in path:
            return False
        _, ext = os.path.splitext(path)
        return ext in MONITORED_EXTENSIONS

    def on_created(self, event):
        if not event.is_directory and self.should_handle(event.src_path):
            log("CREATED", event.src_path)
            backup_file(event.src_path)

    def on_modified(self, event):
        if not event.is_directory and self.should_handle(event.src_path):
            log("MODIFIED", event.src_path)
            backup_file(event.src_path)

    def on_deleted(self, event):
        if not event.is_directory and self.should_handle(event.src_path):
            log("DELETED", event.src_path)
            restore_file(event.src_path)

    def on_moved(self, event):
        if not event.is_directory and self.should_handle(event.dest_path):
            log("MOVED", f"from {event.src_path} to {event.dest_path}")



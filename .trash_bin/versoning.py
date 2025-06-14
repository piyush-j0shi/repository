import os
import shutil
import datetime
import difflib
from watchdog.events import FileSystemEventHandler

WATCH_DIR = "."
BACKUP_DIR = ".trash_bin"
VERSIONS_DIR = ".versions"
LOG_FILE = "watchdog.log"

def log(action, message):
    timestamp = datetime.datetime.now().strftime("%Y-%m-%d %H:%M:%S")
    line = f"[{action}] {timestamp} — {message}"
    print(line)
    with open(LOG_FILE, "a") as f:
        f.write(line + "\n")

def ensure_dir(path):
    if not os.path.exists(path):
        os.makedirs(path)

def sanitize_filename(path):
    """Returns safe filename for backup/version storage (e.g., subdir_test.txt)."""
    relative_path = os.path.relpath(path, WATCH_DIR)
    return relative_path.replace(os.sep, "_")

def backup_file(file_path):
    if not os.path.isfile(file_path):
        return

    ensure_dir(BACKUP_DIR)

    filename = os.path.basename(file_path)
    sanitized = sanitize_filename(file_path)
    backup_path = os.path.join(BACKUP_DIR, sanitized)

    if os.path.abspath(file_path) != os.path.abspath(backup_path):
        shutil.copy2(file_path, backup_path)
        log("BACKUP", f"{filename} → {backup_path}")
    else:
        log("SKIP", f"{file_path} already backed up")

def version_file(file_path):
    if not os.path.isfile(file_path):
        return

    filename = os.path.basename(file_path)
    sanitized = sanitize_filename(file_path)
    file_dir = os.path.join(VERSIONS_DIR, sanitized)
    ensure_dir(file_dir)

    timestamp = datetime.datetime.now().strftime("%Y%m%d_%H%M%S")
    version_name = f"{filename}_{timestamp}.txt"
    version_path = os.path.join(file_dir, version_name)
    shutil.copy2(file_path, version_path)
    log("VERSION", f"Saved version: {version_path}")

    versions = sorted([f for f in os.listdir(file_dir) if f.endswith(".txt")])
    if len(versions) >= 2:
        prev_version_path = os.path.join(file_dir, versions[-2])
        with open(prev_version_path, "r") as f1, open(file_path, "r") as f2:
            diff = difflib.unified_diff(
                f1.readlines(), f2.readlines(),
                fromfile=versions[-2], tofile=version_name,
                lineterm=""
            )
            diff_text = "\n".join(diff)
            if diff_text.strip():
                diff_file = os.path.join(file_dir, f"{filename}_{timestamp}.diff")
                with open(diff_file, "w") as df:
                    df.write(diff_text)
                log("DIFF", f"Saved diff: {diff_file}")

class CustomHandler(FileSystemEventHandler):
    def on_created(self, event):
        if not event.is_directory:
            log("CREATED", event.src_path)
            backup_file(event.src_path)

    def on_modified(self, event):
        if not event.is_directory:
            log("MODIFIED", event.src_path)
            backup_file(event.src_path)
            version_file(event.src_path)

    def on_deleted(self, event):
        if not event.is_directory:
            log("DELETED", event.src_path)

    def on_moved(self, event):
        if not event.is_directory:
            log("MOVED", f"{event.src_path} → {event.dest_path}")


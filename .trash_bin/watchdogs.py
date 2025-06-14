import time
import os
from watchdog.observers import Observer
from watch_dogs import CustomHandler, log

if __name__ == "__main__":
    path = "."  
    log("WATCHING", f"Monitoring folder: {os.path.abspath(path)}")

    event_handler = CustomHandler()
    observer = Observer()
    observer.schedule(event_handler, path=path, recursive=False)
    observer.start()

    try:
        while True:
            time.sleep(1)

    except KeyboardInterrupt:
        log("EXIT", "Stopping observer...")
        observer.stop()

    observer.join()


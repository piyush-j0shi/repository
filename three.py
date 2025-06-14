import os
import sys
import time
import shutil
import random
import threading
import py_compile
from datetime import datetime

# ANSI color codes for sick effects
class Colors:
    RED = '\033[91m'
    GREEN = '\033[92m'
    YELLOW = '\033[93m'
    BLUE = '\033[94m'
    MAGENTA = '\033[95m'
    CYAN = '\033[96m'
    WHITE = '\033[97m'
    BOLD = '\033[1m'
    DIM = '\033[2m'
    UNDERLINE = '\033[4m'
    BLINK = '\033[5m'
    REVERSE = '\033[7m'
    RESET = '\033[0m'
    BG_RED = '\033[41m'
    BG_GREEN = '\033[42m'
    BG_BLUE = '\033[44m'

def clear_screen():
    os.system('cls' if os.name == 'nt' else 'clear')

def get_width():
    try:
        return shutil.get_terminal_size((120, 30)).columns
    except:
        return 120

def typewriter(text, delay=0.03, color=Colors.GREEN):
    for char in text:
        sys.stdout.write(color + char + Colors.RESET)
        sys.stdout.flush()
        time.sleep(delay)
    print()

def matrix_rain(duration=3):
    width = get_width()
    height = 20
    matrix = [[' ' for _ in range(width)] for _ in range(height)]

    start_time = time.time()
    while time.time() - start_time < duration:
        # Add new drops
        for _ in range(random.randint(1, 3)):
            col = random.randint(0, width - 1)
            matrix[0][col] = random.choice('01ﾊﾐﾋｰｳｼﾅﾓﾆｻﾜﾂｵﾘｱﾎﾃﾏｹﾒｴｶｷﾑﾕﾗｾﾈｽﾀﾇﾍ')

        # Move drops down
        for row in range(height - 1, 0, -1):
            for col in range(width):
                matrix[row][col] = matrix[row - 1][col]

        # Clear top row
        for col in range(width):
            matrix[0][col] = ' ' if random.random() > 0.1 else matrix[0][col]

        # Print matrix
        clear_screen()
        for row in matrix:
            line = ''.join(row)
            print(Colors.GREEN + line + Colors.RESET)

        time.sleep(0.1)

def glitch_intense(text, noise=0.05):
    glitch_chars = '!@#$%^&*()_+-=[]{}|;:,.<>?~`█▓▒░'
    result = ''
    for i, c in enumerate(text):
        if random.random() < noise:
            result += Colors.RED + random.choice(glitch_chars) + Colors.RESET
        elif random.random() < noise * 0.5:
            result += Colors.YELLOW + c.upper() + Colors.RESET
        else:
            result += c
    return result

def neon_glow(text, color=Colors.CYAN):
    return f"{color}{Colors.BOLD}{text}{Colors.RESET}"

def cyber_banner():
    banner = f"""
{Colors.CYAN}{Colors.BOLD}
    ██████╗██╗   ██╗██████╗ ███████╗██████╗
    ██╔════╝╚██╗ ██╔╝██╔══██╗██╔════╝██╔══██╗
    ██║      ╚████╔╝ ██████╔╝█████╗  ██████╔╝
    ██║       ╚██╔╝  ██╔══██╗██╔══╝  ██╔══██╗
    ╚██████╗   ██║   ██████╔╝███████╗██║  ██║
     ╚═════╝   ╚═╝   ╚═════╝ ╚══════╝╚═╝  ╚═╝

    ████████╗███████╗██████╗ ███╗   ███╗██╗███╗   ██╗ █████╗ ██╗
    ╚══██╔══╝██╔════╝██╔══██╗████╗ ████║██║████╗  ██║██╔══██╗██║
       ██║   █████╗  ██████╔╝██╔████╔██║██║██╔██╗ ██║███████║██║
       ██║   ██╔══╝  ██╔══██╗██║╚██╔╝██║██║██║╚██╗██║██╔══██║██║
       ██║   ███████╗██║  ██║██║ ╚═╝ ██║██║██║ ╚████║██║  ██║███████╗
       ╚═╝   ╚══════╝╚═╝  ╚═╝╚═╝     ╚═╝╚═╝╚═╝  ╚═══╝╚═╝  ╚═╝╚══════╝
{Colors.RESET}
"""
    print(banner)
    time.sleep(2)

def animated_progress_bar(task, duration=3, style='cyber'):
    width = get_width() - 40
    print(f"\n{Colors.YELLOW}[EXECUTING]{Colors.RESET} {glitch_intense(task)}")

    if style == 'cyber':
        for i in range(0, 101, random.choice([2, 3, 4, 5])):
            filled = int((i / 100) * width)
            bar = '█' * filled + '░' * (width - filled)

            # Add glitch effect to bar
            if random.random() < 0.1:
                bar = glitch_intense(bar, 0.05)

            percentage = f"{i:3d}%"
            status = f"{Colors.CYAN}[{bar}]{Colors.RESET} {Colors.GREEN}{percentage}{Colors.RESET}"

            # Add random status messages
            if i % 20 == 0 and i > 0:
                status += f" {Colors.DIM}({random.choice(['OK', 'VERIFIED', 'SYNCED', 'LOCKED'])}){Colors.RESET}"

            sys.stdout.write(f'\r{status}')
            sys.stdout.flush()
            time.sleep(duration / 50)

        print(f" {Colors.GREEN}{Colors.BOLD}[COMPLETE]{Colors.RESET}")

    elif style == 'matrix':
        matrix_chars = '01'
        for i in range(0, 101, random.choice([3, 5, 7])):
            filled = int((i / 100) * width)
            bar = ''.join(random.choice(matrix_chars) for _ in range(filled))
            empty = '.' * (width - filled)

            sys.stdout.write(f'\r{Colors.GREEN}[{bar}{empty}] {i:3d}%{Colors.RESET}')
            sys.stdout.flush()
            time.sleep(duration / 30)

        print(f" {Colors.GREEN}[MATRIX SYNCHRONIZED]{Colors.RESET}")

def hologram_effect(text, duration=2):
    colors = [Colors.CYAN, Colors.BLUE, Colors.MAGENTA, Colors.WHITE]

    for _ in range(int(duration * 10)):
        color = random.choice(colors)
        glitched = glitch_intense(text, 0.02)
        sys.stdout.write(f'\r{color}{Colors.BOLD}{glitched}{Colors.RESET}')
        sys.stdout.flush()
        time.sleep(0.1)

    print(f'\r{Colors.CYAN}{Colors.BOLD}{text}{Colors.RESET}')

def neural_network_viz():
    width = get_width()
    height = 15

    print(f"\n{Colors.MAGENTA}{Colors.BOLD}[NEURAL NETWORK VISUALIZATION]{Colors.RESET}")

    for frame in range(30):
        clear_screen()
        print(f"{Colors.MAGENTA}Neural Activity Frame {frame + 1}/30{Colors.RESET}\n")

        for row in range(height):
            line = ""
            for col in range(width):
                if random.random() < 0.1:
                    line += Colors.GREEN + "●" + Colors.RESET
                elif random.random() < 0.05:
                    line += Colors.RED + "◆" + Colors.RESET
                elif random.random() < 0.03:
                    line += Colors.YELLOW + "▲" + Colors.RESET
                else:
                    line += Colors.DIM + "·" + Colors.RESET
            print(line)

        time.sleep(0.2)

def fake_system_breach():
    systems = [
        "MAINFRAME-ALPHA-7",
        "SECURITY-NODE-12",
        "DATABASE-CLUSTER-X",
        "FIREWALL-GAMMA-3",
        "BACKUP-SYSTEM-9",
        "CORE-PROCESSOR-5"
    ]

    for system in systems:
        status = random.choice(["BREACHED", "COMPROMISED", "ACCESSED", "INFILTRATED"])
        color = Colors.RED if "BREACH" in status else Colors.YELLOW
        print(f"{color}[{status}]{Colors.RESET} {system} - {Colors.GREEN}SUCCESS{Colors.RESET}")
        time.sleep(random.uniform(0.3, 0.8))

def spinning_loader(text, duration=3):
    spinners = ['⠋', '⠙', '⠹', '⠸', '⠼', '⠴', '⠦', '⠧', '⠇', '⠏']
    end_time = time.time() + duration

    i = 0
    while time.time() < end_time:
        spinner = spinners[i % len(spinners)]
        sys.stdout.write(f'\r{Colors.CYAN}{spinner}{Colors.RESET} {text}')
        sys.stdout.flush()
        time.sleep(0.1)
        i += 1

    print(f'\r{Colors.GREEN}✓{Colors.RESET} {text} - {Colors.GREEN}COMPLETE{Colors.RESET}')

def enhanced_fake_ops():
    print(f"\n{Colors.RED}{Colors.BOLD}[INITIALIZING CYBER OPERATIONS]{Colors.RESET}\n")
    time.sleep(1)

    # Matrix rain intro
    matrix_rain(2)
    clear_screen()

    # Cyber banner
    cyber_banner()

    # Enhanced operations
    operations = [
        ("Establishing quantum tunnel", "cyber"),
        ("Bypassing biometric scanners", "matrix"),
        ("Injecting payload into memory", "cyber"),
        ("Cracking encryption algorithms", "matrix"),
        ("Hijacking satellite uplink", "cyber"),
        ("Manipulating data streams", "matrix"),
        ("Deploying stealth modules", "cyber"),
        ("Synchronizing with darknet", "matrix")
    ]

    for op, style in operations:
        animated_progress_bar(op, random.uniform(2, 4), style)
        time.sleep(0.5)

    print(f"\n{Colors.YELLOW}[PHASE 2: SYSTEM INFILTRATION]{Colors.RESET}")
    fake_system_breach()

    print(f"\n{Colors.MAGENTA}[PHASE 3: NEURAL ANALYSIS]{Colors.RESET}")
    neural_network_viz()

    print(f"\n{Colors.CYAN}[PHASE 4: DATA EXTRACTION]{Colors.RESET}")

    extraction_tasks = [
        "Scanning classified databases",
        "Extracting user credentials",
        "Copying encrypted files",
        "Analyzing network topology",
        "Harvesting metadata"
    ]

    for task in extraction_tasks:
        spinning_loader(task, random.uniform(1.5, 3))

    # Hologram effects
    print(f"\n{Colors.RED}[WARNING: TRACE DETECTED]{Colors.RESET}")
    hologram_effect("INITIATING COUNTERMEASURES...", 3)

    print(f"\n{Colors.GREEN}[ALL OPERATIONS COMPLETE]{Colors.RESET}")
    print(f"{Colors.DIM}Timestamp: {datetime.now().strftime('%Y-%m-%d %H:%M:%S')}{Colors.RESET}")

def glitch_text_animation(text, iterations=10):
    original = text
    for i in range(iterations):
        # Gradually reduce glitch intensity
        noise_level = 0.3 - (i * 0.025)
        glitched = glitch_intense(original, max(0.02, noise_level))
        sys.stdout.write(f'\r{glitched}')
        sys.stdout.flush()
        time.sleep(0.2)

    # Final clean text
    print(f'\r{Colors.GREEN}{Colors.BOLD}{original}{Colors.RESET}')

def nuke_trace_enhanced(source):
    print(f"\n{Colors.RED}{Colors.BLINK}[CRITICAL: INITIATING SELF-DESTRUCT]{Colors.RESET}")

    # Countdown with effects
    for i in range(5, 0, -1):
        glitch_text_animation(f">>> SELF-DESTRUCT IN {i}...", 5)
        time.sleep(0.8)

    try:
        # Enhanced destruction sequence
        destruction_steps = [
            "Wiping memory sectors",
            "Fragmenting disk clusters",
            "Overwriting file headers",
            "Clearing process traces",
            "Purging system logs",
            "Destroying evidence chain"
        ]

        for step in destruction_steps:
            spinning_loader(step, 1.5)

        # Original cleanup code
        py_compile.compile(source, doraise=True)
        cache_dir = "__pycache__"
        pyc_file = None
        for file in os.listdir(cache_dir):
            if file.startswith(os.path.splitext(os.path.basename(source))[0]):
                pyc_file = os.path.join(cache_dir, file)
                break
        if pyc_file and os.path.exists(pyc_file):
            os.remove(pyc_file)
        if os.path.exists(cache_dir):
            shutil.rmtree(cache_dir)
        os.remove(source)
        open(source, 'w').close()

        print(f"\n{Colors.GREEN}{Colors.BOLD}>>> ALL TRACES ELIMINATED{Colors.RESET}")
        print(f"{Colors.DIM}System restored to clean state{Colors.RESET}")

    except Exception as e:
        print(f"{Colors.RED}[ERROR DURING CLEANUP]: {e}{Colors.RESET}")

def launch():
    clear_screen()

    # Epic intro
    typewriter("CYBER TERMINAL v2.0 - ENHANCED EDITION", 0.05, Colors.CYAN)
    typewriter("Initializing secure connection...", 0.03, Colors.GREEN)

    # Handshake with glitch effect
    glitch_text_animation(">>> HANDSHAKE ACCEPTED - TUNNEL ESTABLISHED <<<", 8)

    time.sleep(1)

    # Main operations
    enhanced_fake_ops()

    # Self-destruct sequence
    time.sleep(2)
    nuke_trace_enhanced(__file__)

if __name__ == "__main__":
    try:
        launch()
    except KeyboardInterrupt:
        print(f"\n{Colors.RED}[OPERATION TERMINATED BY USER]{Colors.RESET}")
        print(f"{Colors.YELLOW}Emergency protocols activated{Colors.RESET}")
    except Exception as e:
        print(f"\n{Colors.RED}[SYSTEM ERROR]: {e}{Colors.RESET}")
        print(f"{Colors.DIM}Check system logs for details{Colors.RESET}")

const root = document.documentElement;
const icon = document.getElementById('toggle-icon');
const label = document.getElementById('toggle-label');

// ── Theme ──
function isDark() {
  const theme = root.getAttribute('data-theme');
  if (theme) return theme === 'dark';
  return window.matchMedia('(prefers-color-scheme: dark)').matches;
}

function updateToggle() {
  icon.textContent = isDark() ? '●' : '○';
  label.textContent = isDark() ? 'light' : 'dark';
}

function toggleTheme() {
  root.setAttribute('data-theme', isDark() ? 'light' : 'dark');
  updateToggle();
}

updateToggle();

// ── Typewriter ──
const typewriterEl = document.getElementById('typewriter');
const cursorEl = document.querySelector('.cursor');
const fullName = 'Piyush Joshi';
let charIndex = 0;

function type() {
  if (charIndex < fullName.length) {
    typewriterEl.textContent += fullName[charIndex++];
    setTimeout(type, 80);
  } else {
    setTimeout(() => {
      cursorEl.style.transition = 'opacity 0.4s ease';
      cursorEl.style.opacity = '0';
    }, 2000);
  }
}

type();

// ── Progress bar + Back to top + Active section ──
const progressBar = document.getElementById('progress-bar');
const backToTop   = document.getElementById('back-to-top');
const sectionIds  = ['datamonk', 'anavcloud', 'clavrit', 'text-to-sql', 'skills'];
const navDots     = document.querySelectorAll('.nav-dot');

function updateActiveSection() {
  const mid = window.scrollY + window.innerHeight * 0.35;
  let activeId = null;
  sectionIds.forEach(id => {
    const el = document.getElementById(id);
    if (el && el.offsetTop <= mid) activeId = id;
  });
  navDots.forEach(dot => dot.classList.toggle('active', dot.dataset.target === activeId));
}

window.addEventListener('scroll', () => {
  const scrollTop = window.scrollY;
  const docHeight = document.documentElement.scrollHeight - window.innerHeight;
  progressBar.style.width = `${(scrollTop / docHeight) * 100}%`;
  backToTop.classList.toggle('visible', scrollTop > 300);
  updateActiveSection();
}, { passive: true });

updateActiveSection();

function scrollToTop() {
  window.scrollTo({ top: 0, behavior: 'smooth' });
}

// ── Scroll animations ──
const entries = document.querySelectorAll('.entry');
const scrollObserver = new IntersectionObserver((items) => {
  items.forEach(item => {
    if (item.isIntersecting) {
      item.target.classList.add('visible');
    } else if (item.boundingClientRect.top > 0) {
      item.target.classList.remove('visible');
    }
  });
}, { threshold: 0.05 });

entries.forEach(entry => {
  entry.classList.add('will-animate');
  scrollObserver.observe(entry);
});


// ── Text scramble ──
const scrambleChars = 'ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789';

function scramble(el, onDone) {
  const original = el.textContent;
  let frame = 0;
  const totalFrames = original.length * 2.5;

  const run = () => {
    el.textContent = original.split('').map((char, i) => {
      if (char === ' ' || char === '-' || char === '·') return char;
      if (i < Math.floor(frame / 2.5)) return original[i];
      return scrambleChars[Math.floor(Math.random() * scrambleChars.length)];
    }).join('');

    frame++;
    if (frame <= totalFrames) {
      requestAnimationFrame(run);
    } else {
      el.textContent = original;
      onDone?.();
    }
  };

  requestAnimationFrame(run);
}

document.querySelectorAll('.entry-title').forEach(el => {
  if (el.closest('.skills-entry')) return;
  let isScrambling = false;
  el.addEventListener('mouseenter', () => {
    if (isScrambling) return;
    isScrambling = true;
    scramble(el, () => { isScrambling = false; });
  });
});

// ── Section anchor links ──
document.querySelectorAll('.anchor-link').forEach(link => {
  link.addEventListener('click', e => {
    e.preventDefault();
    const id = link.getAttribute('href').slice(1);
    const url = `${location.origin}${location.pathname}#${id}`;
    navigator.clipboard.writeText(url).then(() => {
      link.textContent = '✓';
      setTimeout(() => { link.textContent = '#'; }, 1500);
    });
  });
});

// ── Section nav dots ──
navDots.forEach(dot => {
  dot.addEventListener('click', () => {
    document.getElementById(dot.dataset.target)?.scrollIntoView({ behavior: 'smooth' });
  });
});

// ── Skill tooltips ──
const skillUsage = {
  LangGraph:             ['Datamonk', 'AnavCloud', 'Text-to-SQL'],
  LangChain:             ['Datamonk', 'AnavCloud', 'Text-to-SQL'],
  LlamaIndex:            ['AnavCloud'],
  'Scikit-learn':        ['Datamonk'],
  OpenCV:                ['Clavrit'],
  NumPy:                 ['Datamonk'],
  Pandas:                ['Datamonk'],
  YOLO:                  ['Clavrit'],
  RAG:                   ['AnavCloud'],
  'Multi-Agent Systems': ['Datamonk'],
  'LLM Orchestration':   ['Datamonk', 'Text-to-SQL'],
  Python:                ['Datamonk', 'AnavCloud', 'Clavrit', 'Text-to-SQL'],
  FastAPI:               ['Datamonk'],
  Flask:                 ['AnavCloud'],
  Docker:                ['Datamonk'],
  'CI/CD':               ['Datamonk'],
  ChromaDB:              ['AnavCloud'],
  'Vector Databases':    ['AnavCloud'],
  Azure:                 ['AnavCloud'],
  SQL:                   ['Text-to-SQL'],
};

const tooltip = document.getElementById('skill-tooltip');

document.querySelectorAll('.skill-tag').forEach(tag => {
  const skill = tag.textContent.trim();
  const usedIn = skillUsage[skill];
  if (!usedIn || usedIn.length === 0) return;

  tag.addEventListener('mouseenter', () => {
    tooltip.textContent = 'used in: ' + usedIn.join(' · ');
    // position off-screen first so we can measure it
    tooltip.style.left = '-9999px';
    tooltip.style.top = '-9999px';
    tooltip.classList.add('visible');

    setTimeout(() => {
      const rect = tag.getBoundingClientRect();
      const tt   = tooltip.getBoundingClientRect();
      let top  = rect.top - tt.height - 8;
      let left = rect.left;
      if (left + tt.width > window.innerWidth - 8) left = window.innerWidth - tt.width - 8;
      if (top < 8) top = rect.bottom + 8;
      tooltip.style.left = left + 'px';
      tooltip.style.top  = top  + 'px';
    }, 0);
  });

  tag.addEventListener('mouseleave', () => tooltip.classList.remove('visible'));
});

// ── Copy email ──
function copyEmail() {
  const email = 'piyush.limph@gmail.com';
  const el = document.getElementById('email-display');
  navigator.clipboard.writeText(email).then(() => {
    el.textContent = 'copied!';
    el.classList.add('copied');
    setTimeout(() => { el.textContent = email; el.classList.remove('copied'); }, 2000);
  });
}

// ── Skills filter ──
const filterBtns = document.querySelectorAll('.filter-btn');
const skillTags  = document.querySelectorAll('.skill-tag');

filterBtns.forEach(btn => {
  btn.addEventListener('click', () => {
    filterBtns.forEach(b => b.classList.remove('active'));
    btn.classList.add('active');
    const filter = btn.dataset.filter;
    skillTags.forEach(tag => {
      tag.classList.toggle('hidden', filter !== 'all' && tag.dataset.cat !== filter);
    });
  });
});

// ── Command palette ──
let cmdOpen = false;
let cmdSelected = 0;
let cmdFiltered = [];

const commands = [
  { label: 'Datamonk',     hint: 'navigate', action: () => { scrollToSection('datamonk');    closeCmd(); } },
  { label: 'AnavCloud',    hint: 'navigate', action: () => { scrollToSection('anavcloud');   closeCmd(); } },
  { label: 'Clavrit',      hint: 'navigate', action: () => { scrollToSection('clavrit');     closeCmd(); } },
  { label: 'Text-to-SQL',  hint: 'navigate', action: () => { scrollToSection('text-to-sql'); closeCmd(); } },
  { label: 'Skills',       hint: 'navigate', action: () => { scrollToSection('skills');      closeCmd(); } },
  { label: 'Toggle Theme', hint: 'action',   action: () => { toggleTheme(); closeCmd(); } },
  { label: 'Copy Email',   hint: 'action',   action: () => { copyEmail();   closeCmd(); } },
];

function scrollToSection(id) {
  document.getElementById(id)?.scrollIntoView({ behavior: 'smooth' });
}

function openCmd() {
  cmdOpen = true;
  document.getElementById('cmd-backdrop').classList.add('open');
  document.getElementById('cmd-palette').classList.add('open');
  const input = document.getElementById('cmd-input');
  input.value = '';
  input.focus();
  renderCmdList('');
}

function closeCmd() {
  cmdOpen = false;
  document.getElementById('cmd-backdrop').classList.remove('open');
  document.getElementById('cmd-palette').classList.remove('open');
}

function renderCmdList(query) {
  const q = query.toLowerCase();
  cmdFiltered = commands.filter(c => c.label.toLowerCase().includes(q));
  cmdSelected = 0;
  const list = document.getElementById('cmd-list');
  list.innerHTML = cmdFiltered.map((c, i) => `
    <li class="cmd-item ${i === 0 ? 'selected' : ''}" data-index="${i}">
      <span class="cmd-label">${c.label}</span>
      <span class="cmd-hint">${c.hint}</span>
    </li>
  `).join('');
  list.querySelectorAll('.cmd-item').forEach(item => {
    item.addEventListener('click', () => cmdFiltered[+item.dataset.index].action());
  });
}

function updateCmdSelection() {
  document.querySelectorAll('.cmd-item').forEach((item, i) => {
    item.classList.toggle('selected', i === cmdSelected);
  });
  document.querySelectorAll('.cmd-item')[cmdSelected]?.scrollIntoView({ block: 'nearest' });
}

document.getElementById('cmd-input').addEventListener('input', e => renderCmdList(e.target.value));
document.getElementById('cmd-backdrop').addEventListener('click', closeCmd);

// ── Magnetic snap (elements move toward cursor) ──
document.querySelectorAll('.toggle, .back-to-top, .filter-btn, .skill-tag, .email-copy').forEach(el => {
  el.addEventListener('mousemove', e => {
    const r  = el.getBoundingClientRect();
    const dx = (e.clientX - (r.left + r.width  / 2)) * 0.55;
    const dy = (e.clientY - (r.top  + r.height / 2)) * 0.55;
    el.style.transform = `translate(${dx}px, ${dy}px)`;
  });

  el.addEventListener('mouseleave', () => {
    el.style.transition = 'transform 0.6s cubic-bezier(0.23, 1, 0.32, 1)';
    el.style.transform  = '';
    setTimeout(() => { el.style.transition = ''; }, 650);
  });
});

// ── Keyboard shortcuts ──
document.addEventListener('keydown', e => {
  if ((e.key === 'k' || e.key === 'K') && (e.metaKey || e.ctrlKey)) {
    e.preventDefault();
    cmdOpen ? closeCmd() : openCmd();
    return;
  }
  if (cmdOpen) {
    if (e.key === 'Escape')    { closeCmd(); return; }
    if (e.key === 'ArrowDown') { e.preventDefault(); cmdSelected = Math.min(cmdSelected + 1, cmdFiltered.length - 1); updateCmdSelection(); }
    if (e.key === 'ArrowUp')   { e.preventDefault(); cmdSelected = Math.max(cmdSelected - 1, 0); updateCmdSelection(); }
    if (e.key === 'Enter')     { cmdFiltered[cmdSelected]?.action(); }
    return;
  }
  if ((e.key === 'd' || e.key === 'D') && !e.metaKey && !e.ctrlKey) toggleTheme();
});

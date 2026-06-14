const root = document.documentElement;
const icon = document.getElementById('toggle-icon');
const label = document.getElementById('toggle-label');

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

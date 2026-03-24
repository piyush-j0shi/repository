const root = document.documentElement;
const icon = document.getElementById('toggle-icon');
const label = document.getElementById('toggle-label');

if (window.matchMedia('(prefers-color-scheme: dark)').matches) {
  root.setAttribute('data-theme', 'dark');
  icon.textContent = '●';
  label.textContent = 'light';
}

function toggleTheme() {
  const isDark = root.getAttribute('data-theme') === 'dark';
  root.setAttribute('data-theme', isDark ? 'light' : 'dark');
  icon.textContent = isDark ? '○' : '●';
  label.textContent = isDark ? 'dark' : 'light';
}

if (!localStorage.theme) {
    localStorage.theme = document.documentElement.getAttribute('data-theme');
} else {
    document.documentElement.setAttribute('data-theme', localStorage.theme);
}

function toggleTheme() {
    if (localStorage.theme === 'light') {
        localStorage.theme = 'dark';
    } else {
        localStorage.theme = 'light';
    }
    document.documentElement.setAttribute('data-theme', localStorage.theme);
}

export default toggleTheme
// Check for the user id, if not found, generate a new one
if (!localStorage.getItem('userId')) {
    const userId = Math.random().toString(36).substring(2, 36);
    localStorage.setItem('userId', userId);
}

//TODO: Check for user name, if not found, ask for one

// Attach the user ID to all HTMX requests
document.addEventListener('htmx:beforeRequest', (e) => {
    const userId = localStorage.getItem('userId');
    if (userId) {
        e.detail.headers['X-User-ID'] = userId;
    }
});
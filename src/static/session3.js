function getSessionId3() {
    const params = new URLSearchParams(window.location.search);
    return params.get('session_id') || 'ID_NOT_FOUND';
}

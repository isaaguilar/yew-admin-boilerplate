window.showModal = function (elementId) {
    const modal = document.getElementById(elementId);
    if (modal) {
        const bootstrapModal = new bootstrap.Modal(modal);
        bootstrapModal.show();
    }
}


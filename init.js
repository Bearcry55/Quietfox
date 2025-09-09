// === Privacy Protection ===

// Block document.cookie (read & write)
Object.defineProperty(document, 'cookie', {
    get: () => '',
    set: () => {}
});

// Clear and disable localStorage
try {
    localStorage.clear();
    Object.defineProperty(window, 'localStorage', {
        value: {
            getItem: () => null,
            setItem: () => {},
            removeItem: () => {},
            clear: () => {},
            key: () => null,
            length: 0
        },
        writable: false
    });
} catch (e) {
    console.warn("localStorage override failed:", e);
}

// Clear and disable sessionStorage
try {
    sessionStorage.clear();
    Object.defineProperty(window, 'sessionStorage', {
        value: {
            getItem: () => null,
            setItem: () => {},
            removeItem: () => {},
            clear: () => {},
            key: () => null,
            length: 0
        },
        writable: false
    });
} catch (e) {
    console.warn("sessionStorage override failed:", e);
}

// Attempt to clear cookies (for engines that allow it)
try {
    document.cookie.split(";").forEach(cookie => {
        const eqPos = cookie.indexOf("=");
        const name = eqPos > -1 ? cookie.substring(0, eqPos) : cookie;
        document.cookie = name + "=;expires=Thu, 01 Jan 1970 00:00:00 GMT;path=/";
    });
} catch (e) {
    console.warn("cookie clearing failed:", e);
}

// === URL Cleaner ===

function cleanURL(url) {
    try {
        const urlObj = new URL(url);
        const trackingParams = [
            'utm_source', 'utm_medium', 'utm_campaign', 'utm_term', 'utm_content',
            'fbclid', 'gclid', 'msclkid', 'dclid', 'twclid', 'ref', 'source',
            'campaign_id', 'ad_id', 'affiliate_id', 'click_id', '_ga', 'mc_cid',
            'mc_eid', 'pk_campaign', 'pk_kwd', 'pk_source', 'sk', 'trk'
        ];
        trackingParams.forEach(param => {
            urlObj.searchParams.delete(param);
        });
        return urlObj.toString();
    } catch (e) {
        return url;
    }
}

// Clean the current URL on load
if (window.location.search) {
    const cleanedURL = cleanURL(window.location.href);
    if (cleanedURL !== window.location.href) {
        window.history.replaceState({}, document.title, cleanedURL);
    }
}

// Intercept link clicks and clean URLs
document.addEventListener('click', function (e) {
    const link = e.target.closest('a');
    if (link && link.href) {
        const cleanedURL = cleanURL(link.href);
        if (cleanedURL !== link.href) {
            e.preventDefault();
            window.location.href = cleanedURL;
        }
    }
}, true);

// === Keyboard Shortcuts ===

document.addEventListener("keydown", (e) => {
    // Navigation
    if ((e.altKey && e.key === "ArrowLeft") ) {
        window.history.back();
    }

    // Reload
    if (e.key === "F5") {
        e.preventDefault();
        window.location.reload();
    }

    // Zoom controls
    const zoom = parseFloat(document.body.style.zoom || "1");
    if (e.ctrlKey && e.key === "=") {
        e.preventDefault();
        document.body.style.zoom = (zoom + 0.1).toString();
    }
    if (e.ctrlKey && e.key === "-") {
        e.preventDefault();
        document.body.style.zoom = Math.max(0.5, zoom - 0.1).toString();
    }
    if (e.ctrlKey && e.key === "0") {
        e.preventDefault();
        document.body.style.zoom = "1";
    }
});

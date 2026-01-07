Για το θέμα WordPress Networker, πρέπει να προσθέσουμε προσαρμοσμένο κώδικα στην εγκατάσταση WordPress για να ανιχνεύσουμε αυτόματα τη σκοτεινή λειτουργία και να ενημερώσουμε το widget σχολίων.

Ο κώδικας πρέπει να εισαχθεί στο υποσέλιδο του ιστότοπού σας. Υπάρχουν αρκετά plugins που μπορούν να το κάνουν αυτό, οπότε δεν θα τα αναφέρουμε εδώ. Ωστόσο, εδώ είναι ο κώδικας για προσθήκη:

[inline-code-attrs-start title = 'Networker Theme Dark Mode Support Script'; type = 'javascript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(function () {
    let isDarkMode = false;

    function setIsDarkMode(newValue) {
        isDarkMode = newValue;
        for (const instance of window.fcUIInstances) {
            if (instance.targetElement) {
                const config = instance.config;
                config.hasDarkBackground = isDarkMode;
                instance.instance.update(config)
            }
        }
    }

    function getDarkModeSetting() {
        return document.body.attributes['data-scheme'].value === 'dark';
    }
    let initialValue = getDarkModeSetting();
    if (isDarkMode !== initialValue) {
        setIsDarkMode(initialValue);
    }
    const observer = new MutationObserver(function (mutations) {
        mutations.forEach(function (mutation) {
            if (mutation.type === "attributes") {
                const newValue = getDarkModeSetting();
                if (isDarkMode !== newValue) {
                    setIsDarkMode(newValue);
                }
                return false;
            }
        });
    });

    observer.observe(document.body, {
        attributes: true
    });
})();
[inline-code-end]

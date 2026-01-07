Per il tema WordPress Networker, dobbiamo aggiungere codice personalizzato alla nostra installazione WordPress per rilevare automaticamente la modalità scura e aggiornare il widget dei commenti.

Il codice deve essere inserito nel footer del tuo sito. Ci sono diversi plugin che possono farlo, quindi non li elencheremo qui. Tuttavia, ecco il codice da aggiungere:

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

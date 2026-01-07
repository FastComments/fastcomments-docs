For WordPress Networker-temaet skal vi tilføje tilpasset kode til vores WordPress-installation for automatisk at registrere mørk tilstand og opdatere kommentar-widgetten.

Koden skal indsættes i footeren på dit websted. Der er mange plugins, der kan gøre dette, så vi vil ikke liste dem her. Her er dog koden, der skal tilføjes:

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

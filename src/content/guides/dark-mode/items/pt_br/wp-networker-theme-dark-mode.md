Para o tema WordPress Networker, precisamos adicionar código personalizado à nossa instalação WordPress para detectar automaticamente o modo escuro e atualizar o widget de comentários.

O código deve ser inserido no rodapé do seu site. Existem vários plugins que podem fazer isso, então não vamos listá-los aqui. No entanto, aqui está o código para adicionar:

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

עבור ערכת הנושא WordPress Networker, עלינו להוסיף קוד מותאם אישית להתקנת WordPress שלנו כדי לזהות אוטומטית מצב כהה ולעדכן את וידג'ט התגובות.

הקוד חייב להיות מוכנס לתחתית האתר שלכם. יש לא מעט תוספים שיכולים לעשות זאת, כך שלא נפרט אותם כאן. עם זאת, הנה הקוד להוספה:

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

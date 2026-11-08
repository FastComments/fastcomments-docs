WordPress Networker थीम के लिए, हमें अपनी WordPress इंस्टॉलेशन में कस्टम कोड जोड़ना होगा ताकि डार्क मोड का स्वतः पता लगाया जा सके और कमेंट विजेट को अपडेट किया जा सके।

यह कोड आपकी साइट के फुटर में डाला जाना चाहिए। ऐसा करने के लिए काफी प्लगइन्स उपलब्ध हैं, इसलिए हम उन्हें यहाँ सूचीबद्ध नहीं करेंगे। हालाँकि, यहाँ जोड़ने के लिए कोड दिया गया है:

[inline-code-attrs-start title = 'Networker थीम डार्क मोड सपोर्ट स्क्रिप्ट'; type = 'javascript'; isFunctional = false; inline-code-attrs-end]
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
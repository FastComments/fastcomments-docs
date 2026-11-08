### डेवलपर्स के लिए - डार्क मोड को बंद करने के लिए बाध्य करना

डार्क मोड को बंद करने के लिए बाध्य किया जा सकता है, इसके लिए विजेट कॉन्फ़िगरेशन में `hasDarkBackground` को `false` के रूप में पास करें। यह VanillaJS, Angular, React, Vue, और React Native लाइब्रेरीज़ के लिए काम करता है।

प्रत्येक लाइब्रेरी के [GitHub](https://github.com/fastComments/) पर एक `examples` फ़ोल्डर होता है, जिसमें डार्क मोड का उपयोग करने के तरीके के उदाहरण होते हैं।

### डार्क मोड को चालू करने के लिए बाध्य करना

हम `hasDarkBackground` को `true` सेट करके डार्क मोड को हमेशा चालू रहने के लिए बाध्य कर सकते हैं।

हम इसे विजेट कस्टमाइज़ेशन UI के माध्यम से भी कर सकते हैं [यहाँ](https://fastcomments.com/auth/my-account/customize-widget)।

`Base Theme` के अंतर्गत बस `Force Dark Mode` चुनें।

### VanillaJS विजेट - डार्क मोड अपडेट करना

डार्क मोड को अपडेट करने का सबसे आसान तरीका यह है कि पेज पर विजेट के सभी इंस्टेंस को देखा जाए और उनकी कॉन्फ़िगरेशन को अपडेट किया जाए:

[inline-code-attrs-start title = 'VanillaJS - डार्क मोड टॉगल'; type = 'javascript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
    let isDarkMode = somehowDetermineIfDarkModeEnabled();
    for (const instanceWrapped of window.fcUIInstances) {
        if (instanceWrapped.targetElement) {
            const config = instanceWrapped.config;
            config.hasDarkBackground = isDarkMode;
            instanceWrapped.instance.update(config)
        }
    }
[inline-code-end]
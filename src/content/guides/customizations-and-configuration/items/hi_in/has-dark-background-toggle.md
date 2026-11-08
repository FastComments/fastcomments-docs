[related-parameter-start name = 'hasDarkBackground'; type = 'boolean'; related-parameter-end]

उन साइटों के लिए जो प्रारंभिक पेज लोड के बाद डार्क मोड टॉगल करने की अनुमति देती हैं, यह थोड़ा अधिक जटिल है।

सबसे पहले, कमेंट विजेट लाइब्रेरी (React, Vue) के सभी वर्तमान संस्करणों में अपने-अपने रिपॉजिटरीज़ में डार्क मोड टॉगल करने के उदाहरण मौजूद हैं।

VanillaJS विजेट के लिए, हमें कुछ अतिरिक्त काम करना होगा। सबसे पहले, FastCommentsUI एक ऑब्जेक्ट लौटाता है जिसमें "destroy" और "update" फ़ंक्शन होते हैं।

हम जब भी कमेंट विजेट कॉन्फ़िगरेशन को अपडेट करना चाहें, बस update फ़ंक्शन को कॉल कर सकते हैं, जैसा कि नीचे दिखाया गया है। यहाँ VanillaJS विजेट के साथ डार्क मोड टॉगल करने का एक पूर्ण कार्यशील उदाहरण दिया गया है।

[inline-code-attrs-start title = 'डार्क मोड टॉगल करने का पूर्ण उदाहरण'; inline-code-attrs-end]
[inline-code-start]
<script src="https://fastcomments.com/js/embed-v2.min.js"></script>
<button id="toggle-dark-mode">Toggle Dark Mode</button>
<div id="fastcomments-widget"></div>
<script>
    (function() {
        const button = document.getElementById('toggle-dark-mode');
        const config = {
            tenantId: 'demo',
            hasDarkBackground: false
        };
        const instance = window.FastCommentsUI(document.getElementById('fastcomments-widget'), config);
        button.addEventListener('click', function() {
            config.hasDarkBackground = !config.hasDarkBackground;
            if (config.hasDarkBackground) {
                document.body.classList.add('dark');
            } else {
                document.body.classList.remove('dark');
            }
            instance.update(config);
        });
    })();
</script>
<style>
    body.dark {
        background: #000;
        color: #fff;
    }
</style>
[inline-code-end]
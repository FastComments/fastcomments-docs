अब हम FastComments विजेट कोड को आपकी साइट में जोड़ेंगे। यह कोड उन सभी फ़ॉर्म को खोजेगा जिनका शीर्षक `FastComments Goes Here` है और उन्हें FastComments से बदल देगा।

तो चलिए साइट एडिटर के निचले बाएँ कोने में `Settings` पर जाते हैं:

<div class="screenshot white-bg">
    <div class="title">सेटिंग्स खोलें</div>
    <img class="screenshot-image" src="/images/installation-guides/ionos-step-2-open-settings.png" alt="सेटिंग्स खोलें" />
</div>

`Custom Head Code` सेक्शन खोलें:

<div class="screenshot white-bg">
    <div class="title">Custom Head Code खोलें</div>
    <img class="screenshot-image" src="/images/installation-guides/ionos-step-2-open-custom-head-code.png" alt="Custom Head Code खोलें" />
</div>

Ionos के लिए हमें FastComments विजेट कोड के एक **विशेष संस्करण** की आवश्यकता है। **अन्य ट्यूटोरियल्स के कोड स्निपेट काम नहीं करेंगे।**

अब निम्नलिखित कोड कॉपी करें:

[inline-code-attrs-start title = 'Ionos FastComments स्निपेट'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
<script>
    (function () {
        let loaded = false;
        let interval = null;

        function attemptLoad() {
            const nodes = document.querySelectorAll('h2');

            nodes.forEach(function (node) {
                if (node.innerText && node.innerText.trim() === 'FastComments Goes Here') {
                    // हम वह एलिमेंट लेते हैं जो पूरी चौड़ाई का नहीं है
                    const target = node.parentNode.parentNode.parentNode.parentNode.parentNode;
                    target.innerHTML = '';
                    FastCommentsUI(target, {
                        tenantId: "demo"
                    });
                    interval && clearInterval(interval);
                    loaded = true;
                }
            });
        }

        attemptLoad();
        if (!loaded) {
            interval = setInterval(attemptLoad, 300);
        }
    })();
</script>
[inline-code-end]

...और इसे दिखाए अनुसार पेस्ट करें:

<div class="screenshot white-bg">
    <div class="title">पेस्ट करें और सेव करें</div>
    <img class="screenshot-image" src="/images/installation-guides/ionos-step-2-paste-and-save.png" alt="पेस्ट करें और सेव करें" />
</div>
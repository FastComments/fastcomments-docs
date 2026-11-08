On-Prem के लिए फ्रंट एंड कोड स्निपेट्स और लाइब्रेरीज़ SaaS प्रोडक्ट जैसे ही हैं। हालाँकि, आपको `apiHost` और सही स्क्रिप्ट पाथ निर्दिष्ट करना होगा:

[inline-code-attrs-start title = 'On Prem के लिए कमेंट्स कोड'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script async src="https://my.host.com/js/embed-v2-async.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
    window.fcConfigs = [{
        target: '#fastcomments-widget',
        tenantId: "demo",
        apiHost: "https://my.host.com"
        // ... can also pass SSO payload etc.
    }];
</script>
[inline-code-end]

ऊपर दिया गया एक बहुत सरल उदाहरण है। हम 1st-party React, Angular, Vue, Svelte, आदि लाइब्रेरीज़ का भी उपयोग कर सकते हैं।
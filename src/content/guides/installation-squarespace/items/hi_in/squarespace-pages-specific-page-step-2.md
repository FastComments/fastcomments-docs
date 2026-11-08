अब हम निम्नलिखित कोड स्निपेट को कॉपी कर सकते हैं। स्निपेट के ऊपरी दाएँ कोने में दिखने वाले कॉपी बटन का उपयोग करें।

कोड में कुछ चीज़ें आप कॉन्फ़िगर कर सकते हैं, लाइन 4 से 7 देखें।

[inline-code-attrs-start title = 'Squarespace Single Page Code'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script async src="https://cdn.fastcomments.com/js/embed-v2-async.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
    window.fcConfigs = [{
        target: '#fastcomments-widget',
        tenantId: 'demo' // आपकी अकाउंट आईडी
    }];
</script>
[inline-code-end]

यह इस तरह दिखना चाहिए:

<div class="screenshot white-bg">
    <div class="title">पेस्ट करें और सेव करें</div>
    <img class="screenshot-image" src="/images/installation-guides/squarespace-pages-specific-page-step-2-1-add-code-and-save.png" alt="पेस्ट करें और सेव करें" />
</div>

अब ऊपरी दाएँ कोने में सेव पर क्लिक करें।

ध्यान दें कि `Preview in Safe Mode` विकल्प काम नहीं करेगा, लेकिन जब आप अपनी साइट पर जाएँगे तो विजेट दिखाई देगा।

यदि आपको समस्या आ रही है, तो सुनिश्चित करें कि नीचे के पास `"tenantId": "demo"` न लिखा हो। यदि आप लॉग इन हैं तो यह आपकी टेनेंट आईडी दिखानी चाहिए। यदि नहीं, तो सपोर्ट से संपर्क करें।
अब हम निम्नलिखित कोड स्निपेट को कॉपी कर सकते हैं। स्निपेट के ऊपरी दाएँ कोने में दिखने वाले कॉपी बटन का उपयोग करें।

कोड में कुछ चीज़ें आप कॉन्फ़िगर कर सकते हैं, लाइन 4 से 7 देखें।

[inline-code-attrs-start title = 'Squarespace सभी पेजों के लिए कमेंट्स कोड'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
    (function () {
        const tenantId = 'demo'; // आपकी अकाउंट आईडी

        function tryLoad() {
            // अलग-अलग लेआउट के लिए लोड करने की कोशिश करें
            let targetDiv = document.querySelector('.blog-item-comments-content');
            if (!targetDiv) {
                targetDiv = document.getElementById('fastcomments-widget');
            }
            window.FastCommentsUI(targetDiv, {
                tenantId
            });
        }

        tryLoad();
    })();
</script>

[inline-code-end]

...फिर कोड एरिया में पेस्ट करें और सेव पर क्लिक करें। यह इस तरह दिखना चाहिए, जिसमें कोड `FOOTER` ब्लॉक में हो:

<div class="screenshot white-bg">
    <div class="title">पेस्ट करें और सेव करें</div>
    <img class="screenshot-image" src="/images/installation-guides/squarespace-pages-all-pages-step-2-1-add-code-and-save.png" alt="पेस्ट करें और सेव करें" />
</div>

यदि आपको समस्या आ रही है, तो सुनिश्चित करें कि नीचे के पास `"tenantId": "demo"` न लिखा हो। यदि आप लॉग इन हैं तो यह आपकी टेनेंट आईडी दिखानी चाहिए। यदि नहीं, तो सपोर्ट से संपर्क करें।
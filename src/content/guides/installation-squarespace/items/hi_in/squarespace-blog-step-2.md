अब हम निम्नलिखित कोड स्निपेट कॉपी कर सकते हैं (स्निपेट के ऊपरी दाएँ कोने में कॉपी बटन का उपयोग करें):

[inline-code-attrs-start title = 'Squarespace ब्लॉग कमेंट्स कोड'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
    (function () {
        const tenantId = 'demo'; // आपकी अकाउंट आईडी

        function tryLoad() {
            // विभिन्न लेआउट के लिए लोड करने का प्रयास करें
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

...फिर कोड एरिया में पेस्ट करें और सेव पर क्लिक करें:

<div class="screenshot white-bg">
    <div class="title">पेस्ट करें और सेव करें</div>
    <img class="screenshot-image" src="/images/installation-guides/squarespace-step-2-1-add-code-and-save.png" alt="पेस्ट करें और सेव करें" />
</div>
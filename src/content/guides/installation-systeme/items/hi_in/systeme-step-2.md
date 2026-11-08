अब हम अपना कोड कॉपी करने जा रहे हैं। यदि कोड स्निपेट में लाइन 6 पर `tenantId: "demo"` दिखता है, तो आपको अपने FastComments खाते में लॉग इन करना चाहिए
और फिर इस पेज को रिफ्रेश करना चाहिए ताकि कॉपी किया गया कोड स्निपेट आपकी खाता आईडी रखता हो।

[inline-code-attrs-start title = 'Systeme.io स्निपेट'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script async src="https://cdn.fastcomments.com/js/embed-v2-async.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
    window.fcConfigs = [{
        target: '#fastcomments-widget',
        tenantId: "demo"
    }];
</script>
[inline-code-end]

अब इसे एडिटर में पेस्ट करें और सेव पर क्लिक करें:

<div class="screenshot white-bg">
    <div class="title">FastComments कोड जोड़ें</div>
    <img class="screenshot-image" src="/images/installation-guides/systeme-add-code.png" alt="FastComments कोड जोड़ें" />
</div>

... फिर अपनी साइट सेव करें। बस इतना ही!
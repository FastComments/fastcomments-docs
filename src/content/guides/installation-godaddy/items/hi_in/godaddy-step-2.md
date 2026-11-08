अब जब आपने एक कस्टम HTML ब्लॉक जोड़ लिया है, हम FastComments विजेट कोड जोड़ सकते हैं।

**Godaddy के लिए निम्नलिखित कोड का उपयोग करें, अन्य ट्यूटोरियल्स का कोड नहीं। यह कोड Godaddy-विशिष्ट है।**

निम्नलिखित कोड कॉपी करें:

[inline-code-attrs-start title = 'Godaddy कमेंट्स कोड स्निपेट'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
    (function () {
        if (top.location.pathname && top.location.pathname.includes('/f')) {
            FastCommentsUI(document.getElementById('fastcomments-widget'), {
                tenantId: "demo",
                pageTitle: top.document.title,
                url: top.location.href,
                urlId: top.location.pathname
            });
        }
    })();
</script>
[inline-code-end]

यह विशेष कोड स्निपेट Godaddy के साथ संगत होने के लिए डिज़ाइन किया गया है, और यह केवल आपकी ब्लॉग पोस्ट पर दिखाई देगा - होमपेज पर नहीं।

अब इस कोड को `Step One` में बताए गए `Custom Code` क्षेत्र में पेस्ट करें।

<div class="screenshot white-bg">
    <div class="title">कोड कॉपी और पेस्ट करें</div>
    <img class="screenshot-image" src="/images/installation-guides/godaddy-step-2-code-added.png" alt="कोड कॉपी और पेस्ट करें" />
</div>

ऊपर दाईं ओर Done पर क्लिक करें:

<div class="screenshot white-bg">
    <div class="title">Done पर क्लिक करें</div>
    <img class="screenshot-image" src="/images/installation-guides/godaddy-step-2-done.png" alt="Done पर क्लिक करें" />
</div>

चरण दो के लिए बस इतना ही!
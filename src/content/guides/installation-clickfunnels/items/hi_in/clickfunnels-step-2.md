अब जब हम टेम्पलेट एडिटर में हैं, हमें यह तय करना होगा कि हम कमेंट्स या लाइव चैट कहाँ दिखाना चाहते हैं।

इस उदाहरण में हम इसे सीधे वीडियो के नीचे जोड़ेंगे। उस एलिमेंट पर होवर करें जिसके अंत में विजेट जोड़ना है, और `ADD ELEMENT` पर क्लिक करें:

<div class="screenshot white-bg">
    <div class="title">Add Element</div>
    <img class="screenshot-image" src="/images/installation-guides/clickfunnels-step-4-add-element.png" alt="Add Element" />
</div>

`CUSTOM JS/HTML` चुनें:

<div class="screenshot white-bg">
    <div class="title">Select CUSTOM JS/HTML</div>
    <img class="screenshot-image" src="/images/installation-guides/clickfunnels-step-5-custom-js-html.png" alt="Select CUSTOM JS/HTML" />
</div>

अब आइए कोड एडिटर खोलें जहाँ हम अपना कोड पेस्ट करेंगे।

अगले चरण में ClickFunnels थोड़ा भ्रमित करने वाला है।

यह महत्वपूर्ण है कि नए एलिमेंट पर होवर करते समय आप `Code` को *चुनें ही नहीं*। इसके बजाय, `SETTINGS` चुनें:

<div class="screenshot white-bg">
    <div class="title">Select SETTINGS</div>
    <img class="screenshot-image" src="/images/installation-guides/clickfunnels-step-6-settings.png" alt="Select SETTINGS" />
</div>

अब दाईं ओर हम `Open Code Editor` पर क्लिक कर सकते हैं:

<div class="screenshot white-bg">
    <div class="title">Click Open Code Editor</div>
    <img class="screenshot-image" src="/images/installation-guides/clickfunnels-step-7-open-code-editor.png" alt="Click Open Code Editor" />
</div>

आपको एक बड़ा वर्गाकार बॉक्स खुलता दिखेगा। यहीं पर हम अपना कोड पेस्ट कर सकते हैं। निम्नलिखित स्निपेट कॉपी करें (ऊपर दाईं ओर के कॉपी बटन का उपयोग करें):

[inline-code-attrs-start title = 'ClickFunnels Streaming Chat Code Snippet'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-live-chat.min.js"></script>
<div id="fastcomments-live-chat-widget" style="width: 500px;min-height: 780px;"></div>
<style>
    #fastcomments-live-chat-widget iframe {
        min-height: 780px;
    }
</style>
<script>
    (function fcLoad() {
        function tryLoad() {
            // some providers change the code snippet to be async
            const container = document.getElementById('fastcomments-live-chat-widget');
            if (!container) {
                return waitRetry();
            }
            if (!window.FastCommentsLiveChat) {
                return waitRetry();
            }
            window.FastCommentsLiveChat(container, {
                tenantId: 'demo'
            });
        }
        function waitRetry() {
            setTimeout(tryLoad, 500);
        }
        tryLoad();
    })();
</script>
[inline-code-end]

यह कोड स्निपेट हमारे Streaming Chat प्रोडक्ट के लिए है, जो वीडियो के साथ बहुत अच्छा लगता है। यदि आप इसके बजाय Live Commenting विजेट का कोड स्निपेट चाहते हैं, जो सामान्य पेजों या ब्लॉग पोस्टों के लिए सबसे उपयुक्त है, तो वह इस ट्यूटोरियल के अंत में है।

जब हम कोड स्निपेट को विंडो में पेस्ट करते हैं, तो यह इस तरह दिखना चाहिए:

<div class="screenshot white-bg">
    <div class="title">Paste Code</div>
    <img class="screenshot-image" src="/images/installation-guides/clickfunnels-step-8-paste.png" alt="Paste Code" />
</div>

अब हमें बस बॉक्स को बंद करना है:

<div class="screenshot white-bg">
    <div class="title">Close</div>
    <img class="screenshot-image" src="/images/installation-guides/clickfunnels-step-9-close.png" alt="Close" />
</div>

अब आप अपने बदलावों का पूर्वावलोकन कर सकते हैं! विजेट को इधर-उधर घुमाएँ और देखें कि आपको यह कहाँ सबसे अच्छा लगता है।

<div class="screenshot white-bg">
    <div class="title">Preview</div>
    <img class="screenshot-image" src="/images/installation-guides/clickfunnels-step-10-preview.png" alt="Preview" />
</div>

सफलता! मोबाइल पर टेस्ट करना न भूलें!

<div class="screenshot white-bg">
    <div class="title">Success!</div>
    <img class="screenshot-image" src="/images/installation-guides/clickfunnels-step-11-success.png" alt="Success!" />
</div>
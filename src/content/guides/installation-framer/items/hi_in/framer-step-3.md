Framer Live Comments FastComments स्निपेट नीचे दिया गया है।

[inline-code-attrs-start title = 'FastComments Framer-विशिष्ट कमेंट्स स्निपेट'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
<div id="fastcomments-widget" style="width: 100%;height: 100%;"></div>
<script>
    (function fcLoad() {
        function tryLoad() {
            // some providers change the code snippet to be async
            const container = document.getElementById('fastcomments-widget');
            if (!container) {
                return waitRetry();
            }
            if (!window.FastCommentsUI) {
                return waitRetry();
            }
            if (container.fastCommentsSetup) {
                return;
            }
            window.FastCommentsUI(container, {
                tenantId: 'demo',
                pageTitle: top.document.title,
                url: top.location.href,
                urlId: top.location.pathname
            });
            container.fastCommentsSetup = true;
        }
        function waitRetry() {
            setTimeout(tryLoad, 500);
        }
        tryLoad();
    })();
</script>
[inline-code-end]

या, वैकल्पिक रूप से, आप Streaming Chat विजेट का उपयोग कर सकते हैं। Framer Streaming Chat FastComments स्निपेट इस प्रकार है:

[inline-code-attrs-start title = 'FastComments Framer-विशिष्ट Streaming Chat स्निपेट'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-live-chat.min.js"></script>
<div id="fastcomments-live-chat-widget" style="width: 100%;height: 100%;"></div>
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
            if (container.fastCommentsSetup) {
                return;
            }
            window.FastCommentsLiveChat(container, {
                tenantId: 'demo',
                pageTitle: top.document.title,
                url: top.location.href,
                urlId: top.location.pathname
            });
            container.fastCommentsSetup = true;
        }
        function waitRetry() {
            setTimeout(tryLoad, 500);
        }
        tryLoad();
    })();
</script>
[inline-code-end]

FastComments Framer एडिटर को सपोर्ट करता है, इसलिए कोड पेस्ट करने के बाद आपको कुछ ऐसा दिखाई देगा (आपको `Publish` पर क्लिक करना पड़ सकता है):

<div class="screenshot white-bg">
    <div class="title">Comment Widget Preview</div>
    <img class="screenshot-image" src="/images/installation-guides/framer-step-3-paste.png" alt="Comment Widget Preview" />
</div>

अब जब आप अपनी साइट देखेंगे तो आपको कमेंट एरिया दिखाई देगा! Framer के साइडबार में आप चाहें तो विजेट को फुल विड्थ भी सेट कर सकते हैं।

ध्यान दें कि Framer विजेट की ऊँचाई सीमित करता है और ऑटो-रिसाइज़िंग को सपोर्ट नहीं करता, इसलिए हमने यहाँ Live Chat
विजेट चुना है क्योंकि यह निश्चित ऊँचाई का होता है।
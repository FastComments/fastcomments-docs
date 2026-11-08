आपने जो HTML एलिमेंट अभी जोड़ा है, उस पर क्लिक करें। दिखाई देने वाले प्रॉपर्टी एडिटर में, निम्नलिखित कोड को HTML फ़ील्ड में पेस्ट करें:

[inline-code-attrs-start title = 'Bubble.io लाइव कमेंटिंग कोड स्निपेट'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
<div id="fastcomments-widget" style="width: 100%;height: 100%;"></div>
<script>
    (function fcLoad() {
        function tryLoad() {
            // bubble tends to change the code snippet to be async
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

<div class="screenshot white-bg">
    <div class="title">FastComments कोड डालें</div>
    <img class="screenshot-image" src="/images/installation-guides/bubble-io-step-2-insert-code.png" alt="HTML एलिमेंट में FastComments कोड डालना" />
</div>

नोट: इस कोड में एक रिट्राई मैकेनिज्म शामिल है ताकि Bubble के डायनामिक वातावरण में FastComments ठीक से लोड हो सके।
अन्य कोड स्निपेट काम नहीं करेंगे।

साइन अप करने के बाद `'demo'` को अपनी वास्तविक FastComments टेनेंट ID से बदलना न भूलें। यदि आप FastComments.com में लॉग इन हैं, तो यह पहले से बदला हुआ होना चाहिए।
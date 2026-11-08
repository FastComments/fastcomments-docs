अगले चरण में आपको नीचे दिया गया पहले से तैयार विजेट कोड कॉपी करना होगा।

जब तक आप FastComments.com में लॉग इन हैं, नीचे दिया गया कोड स्निपेट पहले से ही आपकी खाता जानकारी रखेगा। आइए इसे कॉपी करें:

[inline-code-attrs-start title = 'Super.so FastComments Collab Chat कोड'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-collab-chat.min.js"></script>
<script>
    (function () {
        let currentPathname = window.location.pathname;
        let currentWidget = null;
        let currentTopBar = null;

        function load() {
            if (!window.FastCommentsCollabChat) {
                console.log('...no script, trying again...');
                return setTimeout(load, 100);
            }

            const target = document.querySelector('.super-content');
            if (!target || !target.innerHTML || target.innerHTML.length < 100) {
                console.log('...no content, trying again...');
                return setTimeout(load, 100);
            }

            // मौजूदा इंस्टेंस को साफ़ करें
            if (target.fastCommentsInstance) {
                target.fastCommentsInstance.destroy();
            }

            // यदि मौजूदा टॉप बार है तो उसे साफ़ करें
            if (currentTopBar && currentTopBar.parentNode) {
                currentTopBar.parentNode.removeChild(currentTopBar);
            }

            // नया टॉप बार बनाएं
            const topBarTarget = document.createElement('div');
            target.parentNode.insertBefore(topBarTarget, target);
            topBarTarget.style.maxWidth = 'var(--layout-max-width)';
            topBarTarget.style.margin = '0 auto';
            currentTopBar = topBarTarget;
            currentWidget = target;

            // FastComments Collab Chat इनिशियलाइज़ करें
            target.fastCommentsInstance = FastCommentsCollabChat(target, {
                tenantId: "demo",
                topBarTarget: topBarTarget
            });

            // वर्तमान pathname अपडेट करें
            currentPathname = window.location.pathname;
        }

        // प्रारंभिक लोड
        load();

        // हर 500ms में बदलावों की जांच करें
        setInterval(() => {
            // यदि pathname बदला हो तो रीलोड करें
            if (window.location.pathname !== currentPathname) {
                console.log('Pathname changed, reloading...');
                load();
                return;
            }

            // यदि विजेट हटा दिया गया हो तो रीलोड करें
            if (currentWidget && !currentWidget.parentNode) {
                console.log('Widget removed, reloading...');
                load();
                return;
            }

            // यदि कंटेनर खाली कर दिया गया हो तो रीलोड करें
            const target = document.querySelector('.super-content');
            if (target && target.innerHTML.length < 100) {
                console.log('Container emptied, reloading...');
                load();
            }
        }, 500);
    })();
</script>
[inline-code-end]

अब `Body` क्षेत्र में पेस्ट करें:

<div class="screenshot white-bg">
    <div class="title">पेस्ट किया गया कोड</div>
    <img class="screenshot-image" src="/images/installation-guides/super-so-step-3-pasted-code.png" alt="पेस्ट किया गया कोड" />
</div>

यदि कोड पेस्ट करने के बाद आपको "this is a demo message" दिखाई दे:

- सुनिश्चित करें कि आप अपने fastcomments.com खाते में लॉग इन हैं।
- सुनिश्चित करें कि आपके पास 3rd party cookies सक्षम हैं।
- फिर इस पेज को रिफ्रेश करें और कोड स्निपेट को फिर से कॉपी करें। इसमें `tenantId` आपके टेनेंट के पहचानकर्ता से भरा होना चाहिए।
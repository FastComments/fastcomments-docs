Weebly और FastComments इंटीग्रेशन को ठीक से काम करने के लिए, हमें **दो** छोटे कोड स्निपेट जोड़ने होंगे।

पहला स्निपेट Weebly के "Comments are Closed" संदेश को छिपाने के लिए है, और दूसरा वास्तव में FastComments को लोड करने के लिए है।

सबसे पहले, यह छोटा कोड स्निपेट कॉपी करें:

[inline-code-attrs-start title = 'FastComments हेडर कोड स्निपेट'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<style>
    #comments {
        display: none;
    }
    #commentArea:not(.loaded) {
        display: none;
    }
    #commentArea.loaded {
        display: block !important;
    }
</style>
[inline-code-end]

फिर, `Step One` वाली उसी सेटिंग्स पेज पर, `Post header code` के आगे वाले `+` पर क्लिक करें।

<div class="screenshot white-bg">
    <div class="title">Post Header Code खोलें</div>
    <img class="screenshot-image" src="/images/installation-guides/weebly-step-2-1-open-post-header-code.png" alt="Post Header Code खोलें" />
</div>

आपको इस तरह एक टेक्स्ट बॉक्स खुलता दिखाई देगा:

<div class="screenshot white-bg">
    <div class="title">Post Header Code खुला हुआ</div>
    <img class="screenshot-image" src="/images/installation-guides/weebly-step-2-2-post-header-code-open.png" alt="Post Header Code खुला हुआ" />
</div>

अब अपना कोड स्निपेट पेस्ट करें:

<div class="screenshot white-bg">
    <div class="title">हेडर कोड स्निपेट पेस्ट किया गया</div>
    <img class="screenshot-image" src="/images/installation-guides/weebly-step-2-3-post-header-code-added.png" alt="हेडर कोड स्निपेट पेस्ट किया गया" />
</div>

अगला कदम FastComments को सक्रिय करने के लिए फुटर कोड है। `Post footer code` के आगे वाले प्लस चिह्न पर क्लिक करें:

<div class="screenshot white-bg">
    <div class="title">Post Footer Code खोलें</div>
    <img class="screenshot-image" src="/images/installation-guides/weebly-step-2-4-open-post-footer-code.png" alt="Post Footer Code खोलें" />
</div>

यह कोड स्निपेट कॉपी करें, जो **विशेष रूप से Weebly के लिए** डिज़ाइन किया गया है:

[inline-code-attrs-start title = 'FastComments फुटर कोड स्निपेट'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
    (function () {
        let loaded = false;
        let interval = null;
        function attemptLoad() {
            if (loaded) {
                clearInterval(interval);
                return;
            }
            const comments = document.getElementById('comments');
            if (comments) { // show comments बटन हटाएँ
                comments.remove();
            }
            const commentArea = document.getElementById('commentArea');
            if (!commentArea) {
                return;
            }
            commentArea.innerHTML = '';
            commentArea.classList.add('loaded');
            FastCommentsUI(commentArea, {
                tenantId: "demo",
                urlId: window.location.pathname
            });
            loaded = true;
            clearInterval(interval);
        }
        attemptLoad();
        interval = setInterval(attemptLoad, 300);
    })();
</script>
[inline-code-end]

अब अपना फुटर कोड पेस्ट करें:

<div class="screenshot white-bg">
    <div class="title">Post Footer Code जोड़ा गया</div>
    <img class="screenshot-image" src="/images/installation-guides/weebly-step-2-5-post-footer-code-added.png" alt="Post Footer Code जोड़ा गया" />
</div>

बस इतना ही!
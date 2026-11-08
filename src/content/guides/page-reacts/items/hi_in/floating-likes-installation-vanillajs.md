इंस्टॉलेशन सरल है:

[inline-code-attrs-start title = 'Floating Likes कोड उदाहरण'; type = 'html'; isFunctional = false; type = 'html';  inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-page-likes-floating.min.js"></script>
<div id="fastcomments-page-likes-floating"></div>
<script>
    window.FastCommentsEmbedPageLikesFloating(document.getElementById('fastcomments-page-likes-floating'), {
        tenantId: 'demo'
    });
</script>
[inline-code-end]

कंस्ट्रक्टर का टाइप सिग्नेचर इस प्रकार है:

[inline-code-attrs-start title = 'कॉन्फ़िगरेशन'; useDemoTenant = true; isFunctional = false; type = 'javascript';  inline-code-attrs-end]
[inline-code-start]
/**
 *
 * @param {HTMLElement} targetElement
 * @param config
 * @property {string} tenantId
 * @property {string} urlId - पेज/आर्टिकल आईडी सेट करने के लिए इसे बदलें। डिफ़ॉल्ट रूप से, यह पेज URL होता है।
 * @property {() => void} [onOpenComments]
 * @property {object} [sso]
 * @constructor
 */
[inline-code-end]

यह sso को सपोर्ट करता है ताकि रिएक्शन को प्रमाणीकरण के लिए यूज़र के अकाउंट से जोड़ा जा सके।

वर्तमान में, केवल VanillaJS सपोर्टेड है। यदि आप चाहते हैं कि यह विजेट हमारी किसी क्लाइंट लाइब्रेरी में जोड़ा जाए, तो हमें बताएं!

### Async वर्शन

[inline-code-attrs-start title = 'Floating Likes कोड Async उदाहरण'; isFunctional = false; type = 'html';  inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-page-likes-floating.min.js?v=2" async></script>
<div id="fastcomments-page-likes-floating"></div>
<script>
    (function () {
        function tryLoad() {
            if (window.FastCommentsEmbedPageLikesFloating) {
                window.FastCommentsEmbedPageLikesFloating(document.getElementById('fastcomments-page-likes-floating'), {
                    tenantId: 'demo'
                });
            } else {
                setTimeout(tryLoad, 50);
            }
        }

        tryLoad();
    })();
</script>
[inline-code-end]

### SSO के साथ

हम Secure SSO या Simple SSO पेलोड भी पास कर सकते हैं:

[inline-code-attrs-start title = 'Floating Likes Secure SSO कोड उदाहरण'; type = 'html'; isFunctional = false; type = 'html';  inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-page-likes-floating.min.js"></script>
<div id="fastcomments-page-likes-floating"></div>
<script>
    window.FastCommentsEmbedPageLikesFloating(document.getElementById('fastcomments-page-likes-floating'), {
        tenantId: 'demo',
        sso // sso ऑब्जेक्ट पास करें
    });
</script>
[inline-code-end]

[inline-code-attrs-start title = 'Floating Likes Simple SSO कोड उदाहरण'; type = 'html'; isFunctional = false; type = 'html';  inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-page-likes-floating.min.js"></script>
<div id="fastcomments-page-likes-floating"></div>
<script>
    window.FastCommentsEmbedPageLikesFloating(document.getElementById('fastcomments-page-likes-floating'), {
        tenantId: 'demo',
        simpleSSO: {
            id: 'some-user-id',
            username: 'some username',
            email: 'some@email.com',
        }
    });
</script>
[inline-code-end]
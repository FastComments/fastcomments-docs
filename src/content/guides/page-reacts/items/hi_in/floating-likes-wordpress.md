WordPress के लिए, यह सुविधा WPCode जैसे प्लगइन इंस्टॉल करके और ब्लॉग फुटर में निम्नलिखित HTML स्निपेट जोड़कर सक्षम की जा सकती है:

[inline-code-attrs-start title = 'WordPress के लिए फ्लोटिंग लाइक्स कोड'; type = 'bash'; useDemoTenant = true; isFunctional = false; type = 'html';  inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-page-likes-floating.min.js" async></script>
<div id="fastcomments-page-likes-floating"></div>
<script>
    (function () {
        function tryLoad() {
            if (window.FastCommentsEmbedPageLikesFloating) {
                const articles = document.getElementsByTagName('article');
                if (!articles.length) {
                    return console.warn('Article not found to show fastcomments likes.');
                }
                window.FastCommentsEmbedPageLikesFloating(document.getElementById('fastcomments-page-likes-floating'), {
                    tenantId: '-VuPDR12d-v_',
                    urlId: articles[0].id.replace('post-', '')
                });
            } else {
                setTimeout(tryLoad, 50);
            }
        }

        tryLoad();
    })();
</script>
[inline-code-end]
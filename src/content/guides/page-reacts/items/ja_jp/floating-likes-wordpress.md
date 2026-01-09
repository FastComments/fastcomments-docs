WordPressでは、この機能はWPCodeのようなプラグインをインストールし、次のHTMLスニペットをブログのフッターに追加することで有効にできます:

[inline-code-attrs-start title = 'WordPress用フローティングいいねコード'; type = 'bash'; useDemoTenant = true; isFunctional = false; type = 'html';  inline-code-attrs-end]
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

---
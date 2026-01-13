Για το WordPress, αυτή η δυνατότητα μπορεί να ενεργοποιηθεί εγκαθιστώντας ένα πρόσθετο όπως το WPCode και προσθέτοντας το ακόλουθο απόσπασμα HTML στο υποσέλιδο του ιστολογίου:

[inline-code-attrs-start title = 'Κώδικας Επιπλέουσων Likes για WordPress'; type = 'bash'; useDemoTenant = true; isFunctional = false; type = 'html';  inline-code-attrs-end]
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
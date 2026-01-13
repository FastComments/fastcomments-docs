FastComments תומכת גם בווידג'ט Page Reacts (המכונה גם "לחצן לייק צף") עבור Zyro.

ניתן לראות אותו בפעולה בפינה הימנית התחתונה של דף זה!

1. ראשית, העתק את הקוד:

[inline-code-attrs-start title = 'קוד לייקים צפים של Zyro'; type = 'bash'; useDemoTenant = false; isFunctional = false; type = 'html';  inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-page-likes-floating.min.js?v=2" async></script>
<div id="fastcomments-page-likes-floating"></div>
<script>
    (function () {
        function tryLoad() {
            if (typeof window.FastCommentsEmbedPageLikesFloating === 'function') {
                window.FastCommentsEmbedPageLikesFloating(document.getElementById('fastcomments-page-likes-floating'), {
                    tenantId: "demo"
                });
            } else {
                setTimeout(tryLoad, 50);
            }
        }

        tryLoad();
    })();
</script>
[inline-code-end]

2. לאחר מכן, ב-Zyro, פתח את בונה האתר.
3. עבור אל הגדרות האתר בפינה השמאלית התחתונה.
4. בחר באינטגרציות.
5. הוסף את הקוד החדש ל-*סוף* של השדה `Custom code`, ופרסם את האתר.
6. לא תראה את הווידג'ט במצב תצוגה מקדימה, אך הוא יופיע בגרסה המתפרסמת של האתר.

---
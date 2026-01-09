FastComments תומך גם בווידג'ט Page Reacts (המכונה גם ככפתור לייק צף) עבור Hostinger.

ניתן לראותו בפעולה בפינה הימנית התחתונה של דף זה!

### הערה!

הוראות אלה מיועדות ל-Hostinger Site Builder. אם אתה משתמש ב-Hostinger *WordPress*, פשוט העתק את הקוד שלהלן והוסף אותו לאתר ה-WordPress שלך באמצעות [WPCode](https://wordpress.org/plugins/insert-headers-and-footers/), שהוא תוסף חינמי ונוח להוספת קטעי קוד קצרים לאתר שלך.

1. ראשית, העתק את הקוד:

[inline-code-attrs-start title = 'קוד הלייק הצף של Hostinger'; type = 'bash'; useDemoTenant = false; isFunctional = false; type = 'html';  inline-code-attrs-end]
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

2. לאחר מכן, ב-Hostinger, פתח את בנאי האתר.
3. עבור אל Website Settings בפינה השמאלית התחתונה.
4. בחר ב-Integrations.
5. הוסף את הקוד החדש ל*סוף* השדה `Custom code`, ופרסם את האתר שלך.
6. לא תראה את הווידג'ט במצב תצוגה מקדימה, אבל הוא יופיע בגרסה המתפרסמת של האתר.
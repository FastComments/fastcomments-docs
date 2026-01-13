ההתקנה פשוטה:

[inline-code-attrs-start title = 'דוגמת קוד - לייקים צפים'; type = 'html'; isFunctional = false; type = 'html';  inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-page-likes-floating.min.js"></script>
<div id="fastcomments-page-likes-floating"></div>
<script>
    window.FastCommentsEmbedPageLikesFloating(document.getElementById('fastcomments-page-likes-floating'), {
        tenantId: 'demo'
    });
</script>
[inline-code-end]

חתימת הסוג של הבנאי היא:

[inline-code-attrs-start title = 'תצורה'; useDemoTenant = true; isFunctional = false; type = 'javascript';  inline-code-attrs-end]
[inline-code-start]
/**
 *
 * @param {HTMLElement} targetElement
 * @param config
 * @property {string} tenantId
 * @property {string} urlId - שנה זאת כדי להגדיר את מזהה הדף/המאמר. כברירת מחדל, זהו כתובת ה-URL של הדף.
 * @property {() => void} [onOpenComments]
 * @property {object} [sso]
 * @constructor
 */
[inline-code-end]

זה תומך ב sso כדי לקשר את ה-reacts לחשבון המשתמש לצרכי אימות.

כעת נתמך רק VanillaJS. אם ברצונך שהווידג'ט יתווסף לאחת מספריות הלקוח שלנו, אנא הודע לנו! 

### גרסה אסינכרונית

[inline-code-attrs-start title = 'דוגמת קוד אסינכרוני - לייקים צפים'; isFunctional = false; type = 'html';  inline-code-attrs-end]
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

### עם SSO

ניתן גם להעביר מטעני SSO מאובטח או SSO פשוט:

[inline-code-attrs-start title = 'דוגמת קוד SSO מאובטח - לייקים צפים'; type = 'html'; isFunctional = false; type = 'html';  inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-page-likes-floating.min.js"></script>
<div id="fastcomments-page-likes-floating"></div>
<script>
    window.FastCommentsEmbedPageLikesFloating(document.getElementById('fastcomments-page-likes-floating'), {
        tenantId: 'demo',
        sso // העבר אובייקט sso
    });
</script>
[inline-code-end]

[inline-code-attrs-start title = 'דוגמת קוד SSO פשוט - לייקים צפים'; type = 'html'; isFunctional = false; type = 'html';  inline-code-attrs-end]
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
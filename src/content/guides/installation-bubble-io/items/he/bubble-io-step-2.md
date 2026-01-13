לחץ על רכיב ה-HTML שהוספת זה עתה. בעורך התכונות שמופיע, הדבק את קטע הקוד הבא לתוך שדה ה-HTML:

[inline-code-attrs-start title = 'קטע קוד להערות חיות ב-Bubble.io'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
<div id="fastcomments-widget" style="width: 100%;height: 100%;"></div>
<script>
    (function fcLoad() {
        function tryLoad() {
            // ל-Bubble יש נטייה לשנות את קטע הקוד כך שיהיה אסינכרוני
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
    <div class="title">הכנס את קוד FastComments</div>
    <img class="screenshot-image" src="/images/installation-guides/bubble-io-step-2-insert-code.png" alt="הכנסת קוד FastComments לרכיב ה-HTML" />
</div>

הערה: קטע קוד זה כולל מנגנון ניסיונות חוזרים כדי להבטיח ש-FastComments יטען כראוי בסביבת ה-Bubble הדינמית.
קטעי קוד אחרים לא יעבדו.

זכור להחליף את 'demo' במזהה ה-tenant של FastComments שלך לאחר ההרשמה. אם אתה מחובר ל-FastComments.com, זה אמור להיות מוחלף כבר.
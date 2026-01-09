סקיפט FastComments לתגובות בזמן אמת של Framer נמצא להלן.

[inline-code-attrs-start title = 'קטע FastComments ייעודי ל‑Framer — תגובות'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
<div id="fastcomments-widget" style="width: 100%;height: 100%;"></div>
<script>
    (function fcLoad() {
        function tryLoad() {
            // חלק מהספקים משנים את קטע הקוד כדי שיהיה אסינכרוני
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

או, לחלופין, ניתן להשתמש בוידג'ט שיחת סטרימינג. סקיפט FastComments לשיחת סטרימינג של Framer הוא:

[inline-code-attrs-start title = 'קטע FastComments ייעודי ל‑Framer — שיחת סטרימינג'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-live-chat.min.js"></script>
<div id="fastcomments-live-chat-widget" style="width: 100%;height: 100%;"></div>
<script>
    (function fcLoad() {

        function tryLoad() {
            // חלק מהספקים משנים את קטע הקוד כדי שיהיה אסינכרוני
            const container = document.getElementById('fastcomments-live-chat-widget');
            if (!container) {
                return waitRetry();
            }
            if (!window.FastCommentsLiveChat) {
                return waitRetry();
            }
            if (container.fastCommentsSetup) {
                return;
            }
            window.FastCommentsLiveChat(container, {
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

FastComments תומכת בעורך Framer, כך שעליך לראות משהו כזה לאחר הדבקת הקוד (ייתכן שתצטרך ללחוץ על `Publish`):

<div class="screenshot white-bg">
    <div class="title">תצוגה מקדימה של וידג'ט תגובות</div>
    <img class="screenshot-image" src="/images/installation-guides/framer-step-3-paste.png" alt="תצוגה מקדימה של וידג'ט תגובות" />
</div>

כעת כשאתה צופה באתר שלך עליך לראות את אזור התגובות! בסרגל הצדדי של Framer ניתן להגדיר את הוידג'ט לרוחב מלא גם כן, אם רצוי.

שימו לב ש‑Framer מגביל את גובה הווידג'טים ואינו תומך בהתאמה אוטומטית של גובה, לכן בחרנו כאן בוידג'ט ה‑Live Chat
מכיוון שהוא בעל גובה קבוע.
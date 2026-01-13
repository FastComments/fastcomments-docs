עכשיו אנחנו יכולים להעתיק את קטע הקוד הבא. השתמש בכפתור ההעתקה שמופיע בפינה הימנית העליונה של הקטע.

יש כמה דברים שניתן להגדיר בקוד; ראה שורות 4 עד 7.

[inline-code-attrs-start title = 'קוד לדף יחיד של Squarespace'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
    (function () {
        const tenantId = 'demo'; // מזהה החשבון שלך

        function tryLoad() {
            window.FastCommentsUI(document.getElementById('fastcomments-widget'), {
                tenantId
            });
        }

        tryLoad();
    })();
</script>

[inline-code-end]

זה צריך להיראות כך:

<div class="screenshot white-bg">
    <div class="title">הדבק ושמור</div>
    <img class="screenshot-image" src="/images/installation-guides/squarespace-pages-specific-page-step-2-1-add-code-and-save.png" alt="הדבק ושמור" />
</div>

עכשיו לחץ על שמור בפינה הימנית העליונה.

שים לב שאפשרות `Preview in Safe Mode` לא תפעל, אבל הווידג'ט יופיע כשאתה מבקר באתר שלך.

אם יש לך בעיות, וודא שבחלק התחתון לא כתוב `"tenantId": "demo"`. זה אמור להציג את מזהה ה-tenant שלך אם אתה מחובר. אם לא, פנה לתמיכה.
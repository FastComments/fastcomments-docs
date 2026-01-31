עתה נוכל להעתיק את קטע הקוד הבא. השתמש בכפתור ההעתקה שמופיע בפינה הימנית העליונה של הקטע.

יש כמה דברים שאפשר להגדיר בקוד, ראה שורות 4 עד 7.

[inline-code-attrs-start title = 'קוד לדף יחיד ב-Squarespace'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script async src="https://cdn.fastcomments.com/js/embed-v2-async.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
    window.fcConfigs = [{
        target: '#fastcomments-widget',
        tenantId: 'demo' // מזהה החשבון שלך
    }];
</script>
[inline-code-end]

זה אמור להיראות כך:

<div class="screenshot white-bg">
    <div class="title">הדבק ושמור</div>
    <img class="screenshot-image" src="/images/installation-guides/squarespace-pages-specific-page-step-2-1-add-code-and-save.png" alt="הדבק ושמור" />
</div>

כעת לחץ על שמור בפינה הימנית העליונה.

שים לב שאפשרות `Preview in Safe Mode` לא תעבוד, אבל הווידג'ט יופיע כשאתה מבקר באתר שלך.

אם יש לך בעיות, ודא שבחלק התחתון זה לא אומר `"tenantId": "demo"`. הוא צריך להציג את ה-tenant id שלך אם אתה מחובר. אם לא, פנה לתמיכה.
כעת, אחרי שהוספת בלוק HTML מותאם אישית, נוכל להוסיף את קוד הווידג'ט של FastComments.

**השתמש בקוד הבא ל-Godaddy, לא בקוד ממדריכים אחרים. קוד זה ספציפי ל-Godaddy.**

העתק את הקוד הבא:

[inline-code-attrs-start title = 'קטע קוד תגובות של Godaddy'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
    (function () {
        if (top.location.pathname && top.location.pathname.includes('/f')) {
            FastCommentsUI(document.getElementById('fastcomments-widget'), {
                tenantId: "demo",
                pageTitle: top.document.title,
                url: top.location.href,
                urlId: top.location.pathname
            });
        }
    })();
</script>
[inline-code-end]

קטע קוד זה תוכנן להיות תואם ל-Godaddy, והוא גם יופיע רק בפוסטים בבלוג שלך — לא בדף הבית.

עתה הדבק את הקוד באזור ה-`Custom Code` המוזכר ב-`Step One`.

<div class="screenshot white-bg">
    <div class="title">העתק והדבק את הקוד</div>
    <img class="screenshot-image" src="/images/installation-guides/godaddy-step-2-code-added.png" alt="העתק והדבק את הקוד" />
</div>

לחץ על Done בפינה הימנית העליונה:

<div class="screenshot white-bg">
    <div class="title">לחץ על Done</div>
    <img class="screenshot-image" src="/images/installation-guides/godaddy-step-2-done.png" alt="לחץ על Done" />
</div>

זהו עבור שלב שני!

---
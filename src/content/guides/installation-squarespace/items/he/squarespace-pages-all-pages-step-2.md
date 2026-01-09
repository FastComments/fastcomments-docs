עכשיו נוכל להעתיק את קטע הקוד הבא. השתמש בכפתור ההעתקה שמופיע בפינה הימנית העליונה של הקטע.

יש מספר דברים שניתן להגדיר בקוד — ראה שורות 4 עד 7.

[inline-code-attrs-start title = 'קוד תגובות לכל העמודים של Squarespace'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
    (function () {
        const tenantId = 'demo'; // מזהה החשבון שלך

        function tryLoad() {
            // נסה לטעון עבור פריסות שונות
            let targetDiv = document.querySelector('.blog-item-comments-content');
            if (!targetDiv) {
                targetDiv = document.getElementById('fastcomments-widget');
            }
            window.FastCommentsUI(targetDiv, {
                tenantId
            });
        }

        tryLoad();
    })();
</script>

[inline-code-end]

...לאחר מכן הדבק באזור הקוד ולחץ על שמור. זה אמור להיראות כך, כשהקוד נמצא בבלוק `FOOTER`:

<div class="screenshot white-bg">
    <div class="title">הדבק ושמור</div>
    <img class="screenshot-image" src="/images/installation-guides/squarespace-pages-all-pages-step-2-1-add-code-and-save.png" alt="הדבק ושמור" />
</div>

אם יש לך בעיות, ודא שבקרבת התחתית זה לא אומר `"tenantId": "demo"`. זה אמור להציג את מזהה ה-tenant שלך אם אתה מחובר. אם לא, פנה לתמיכה.
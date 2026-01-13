כעת נוכל להעתיק את קטע הקוד הבא (השתמש בכפתור ההעתקה בפינה הימנית העליונה של הקטע):

[inline-code-attrs-start title = 'קוד תגובות לבלוג Squarespace'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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

...לאחר מכן הדבק באזור הקוד ולחץ על שמור:

<div class="screenshot white-bg">
    <div class="title">הדבק ושמור</div>
    <img class="screenshot-image" src="/images/installation-guides/squarespace-step-2-1-add-code-and-save.png" alt="הדבק ושמור" />
</div>

---
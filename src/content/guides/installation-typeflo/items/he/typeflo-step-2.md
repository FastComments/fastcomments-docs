במדור **כותרת תחתונה** בלשונית **קוד מותאם אישית**, הדבק את הקוד הבא:

[inline-code-attrs-start title = 'קטע קוד לתגובות חיות של Typeflo.io'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js" async></script>
<script>
    (function () {
        function load() {
            let target = null;
            let lastInstance;
            if (document.querySelector('.fastcomments-widget')) {
                setTimeout(load, 1000);
                return;
            }
            if (lastInstance) {
                lastInstance.destroy();
            }
            if (window.FastCommentsUI) {
                const newElement = document.createElement('div');
                newElement.classList.add('fastcomments-widget');
                const subscribeSection = document.querySelector('.nc-SectionSubscribe2');
                if (subscribeSection) {
                    subscribeSection.parentNode.insertBefore(newElement, subscribeSection);
                    target = newElement;
                } else {
                    const fullWidthSection = document.querySelector('.container.w-full');
                    if (fullWidthSection) {
                        fullWidthSection.prepend(newElement);
                        target = newElement;
                    }
                }
            }
            if (target) {
                lastInstance = FastCommentsUI(target, {
                    "tenantId": "demo"
                });
            }
            setTimeout(load, 1000);
        }

        load();
    })();
</script>
[inline-code-end]

<div class="screenshot white-bg">
    <div class="title">הדבק את הקוד במדור הכותרת התחתונה</div>
    <img class="screenshot-image" src="/images/installation-guides/typeflo-step-3-paste-code-in-footer-section.png" alt="הדבק קוד FastComments במדור הכותרת התחתונה" />
</div>

לאחר הדבקת הקוד, לחץ על כפתור **שמור** כדי להחיל את השינויים.

הערה: קוד זה כולל לוגיקה שממקמת בצורה דינמית את הווידג'ט של FastComments במיקום האופטימלי בפוסטים בבלוג של Typeflo.io. קטעי קוד אחרים לא יעבדו כראוי עם פריסת Typeflo.io.

זכור להחליף את 'demo' ב־tenant ID האמיתי של FastComments שלך לאחר ההרשמה. אם אתה מחובר ל-FastComments.com, הוא אמור להיות כבר מוחלף.
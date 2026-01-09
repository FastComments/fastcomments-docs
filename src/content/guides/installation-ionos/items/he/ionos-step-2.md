Next אנחנו עומדים להוסיף את קוד הווידג'ט של FastComments לאתר שלכם. קוד זה יחפש את כל הטפסים עם הכותרת `FastComments Goes Here` ויחליף אותם ב-FastComments.

אז נלך ל-`Settings` בפינה השמאלית התחתונה של עורך האתר:

<div class="screenshot white-bg">
    <div class="title">פתח הגדרות</div>
    <img class="screenshot-image" src="/images/installation-guides/ionos-step-2-open-settings.png" alt="פתח הגדרות" />
</div>

פתח את הסעיף `Custom Head Code`:

<div class="screenshot white-bg">
    <div class="title">פתח את Custom Head Code</div>
    <img class="screenshot-image" src="/images/installation-guides/ionos-step-2-open-custom-head-code.png" alt="פתח את Custom Head Code" />
</div>

בעבור Ionos אנחנו זקוקים לגרסה **מיוחדת** של קוד הווידג'ט של FastComments. קטעי קוד מתוך **מדריכים אחרים לא יעבדו.**

כעת העתק את הקוד הבא:

[inline-code-attrs-start title = 'קטע FastComments עבור Ionos'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
<script>
    (function () {
        let loaded = false;
        let interval = null;

        function attemptLoad() {
            const nodes = document.querySelectorAll('h2');

            nodes.forEach(function (node) {
                if (node.innerText && node.innerText.trim() === 'FastComments Goes Here') {
                    // נבחר את האלמנט שאינו ברוחב מלא
                    const target = node.parentNode.parentNode.parentNode.parentNode.parentNode;
                    target.innerHTML = '';
                    FastCommentsUI(target, {
                        tenantId: "demo"
                    });
                    interval && clearInterval(interval);
                    loaded = true;
                }
            });
        }

        attemptLoad();
        if (!loaded) {
            interval = setInterval(attemptLoad, 300);
        }
    })();
</script>
[inline-code-end]

...והדבק אותו כפי שמוצג:

<div class="screenshot white-bg">
    <div class="title">הדבק ושמור</div>
    <img class="screenshot-image" src="/images/installation-guides/ionos-step-2-paste-and-save.png" alt="הדבק ושמור" />
</div>

---
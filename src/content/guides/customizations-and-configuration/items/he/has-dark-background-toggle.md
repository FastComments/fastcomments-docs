[related-parameter-start name = 'hasDarkBackground'; type = 'boolean'; related-parameter-end]

לאתרים שמאפשרים להחליף למצב כהה לאחר הטעינה הראשונית של הדף, זה קצת יותר מורכב.

ראשית, בכל הגרסאות הנוכחיות של ספריית ווידג'ט ההערות (React, Vue) יש דוגמאות להחלפת מצב כהה במאגרי הקוד המתאימים.

עבור הווידג'ט של VanillaJS, נצטרך לבצע עוד עבודה. קודם כל, FastCommentsUI מחזירה אובייקט עם הפונקציות "destroy" ו-"update".

נוכל פשוט לקרוא לפונקציית update בכל פעם שנרצה לעדכן את תצורת הווידג'ט של ההערות, כך. הנה דוגמה עובדת מלאה להחלפת
מצב כהה עם הווידג'ט של VanillaJS.

[inline-code-attrs-start title = 'דוגמה מלאה להחלפת מצב כהה'; inline-code-attrs-end]
[inline-code-start]
<script src="https://fastcomments.com/js/embed-v2.min.js"></script>
<button id="toggle-dark-mode">Toggle Dark Mode</button>
<div id="fastcomments-widget"></div>
<script>
    (function() {
        const button = document.getElementById('toggle-dark-mode');
        const config = {
            tenantId: 'demo',
            hasDarkBackground: false
        };
        const instance = window.FastCommentsUI(document.getElementById('fastcomments-widget'), config);
        button.addEventListener('click', function() {
            config.hasDarkBackground = !config.hasDarkBackground;
            if (config.hasDarkBackground) {
                document.body.classList.add('dark');
            } else {
                document.body.classList.remove('dark');
            }
            instance.update(config);
        });
    })();
</script>
<style>
    body.dark {
        background: #000;
        color: #fff;
    }
</style>
[inline-code-end]

---
## הוספת ווידגט תגובות חיות למאמרי Notion שלך ב‑Super.so

בנוסף ל‑Collab Chat, באפשרותך להוסיף ווידגט תגובות מסורתי לתחתית המאמרים שלך ב‑Notion. זה מאפשר לקוראים להשאיר תגובות ולנהל דיונים על כל המאמר.

### שלבי התקנה

העתק את הקוד הבא והדבק אותו בקטע ה‑`Body` בהגדרות האתר שלך ב‑Super.so:

[inline-code-attrs-start title = 'ווידגט תגובות חיות של FastComments ל‑Super.so'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
<script>
    (function () {
        let currentPathname = window.location.pathname;
        let currentWidget = null;

        function load() {
            if (!window.FastCommentsUI) {
                console.log('...no script, trying again...');
                return setTimeout(load, 100);
            }

            const contentArea = document.querySelector('.notion-root');
            if (!contentArea || !contentArea.innerHTML || contentArea.innerHTML.length < 100) {
                console.log('...no content, trying again...');
                return setTimeout(load, 100);
            }

            // נקה מופע קיים
            if (contentArea.fastCommentsInstance) {
                contentArea.fastCommentsInstance.destroy();
            }

            // צור יעד חדש
            const target = document.createElement('div');
            contentArea.append(target);
            currentWidget = target;

            // אתחל את FastComments
            contentArea.fastCommentsInstance = FastCommentsUI(target, {
                tenantId: "demo",
                urlId: window.location.pathname
            });

            // עדכן את currentPathname
            currentPathname = window.location.pathname;
        }

        // טעינה ראשונית
        load();

        // בדוק כל 500ms עבור שינויים
        setInterval(() => {
            // טען מחדש אם ה‑pathname השתנה
            if (window.location.pathname !== currentPathname) {
                console.log('Pathname changed, reloading...');
                load();
                return;
            }

            // טען מחדש אם הווידגט הוסר
            if (currentWidget && !currentWidget.parentNode) {
                console.log('Widget removed, reloading...');
                load();
                return;
            }

            // טען מחדש אם המכולה רוקנה
            const contentArea = document.querySelector('.notion-root');
            if (contentArea && contentArea.innerHTML.length < 100) {
                console.log('Container emptied, reloading...');
                load();
            }
        }, 500);
    })();
</script>
[inline-code-end]

### הערות חשובות

- הווידגט התגובות יופיע בתחתית המאמרים שלך ב‑Notion
- לכל עמוד יש שרשור תגובות ייחודי מבוסס על נתיב ה‑URL
- הקפד להחליף את `"demo"` ב‑tenant ID האמיתי שלך מחשבון FastComments שלך
- הווידגט מטפל אוטומטית בטעינת הדפים הדינמית של Super.so
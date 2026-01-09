[related-parameter-start name = 'urlId'; type = 'string'; related-parameter-end]

כבר כיסינו כיצד `urlId` הוא ה-id של הדף או המאמר שאליהם קשורים התגובות.

גם, לסיכום, אם לא הוגדר `urlId` הוא יוגדר כברירת מחדל ל-URL של הדף הנוכחי.

מה לגבי SPA-ים, או Single-Page-Applications, שבהם הדף או התוכן שאליהם קשורות התגובות משתנים
דינמית ללא רענון מלא של הדף?

#### Angular, React, Vue, וכו'

עם הספריות שלנו כגון Angular ו-React, עדכון פשוט של מאפיין `urlId` שמועבר ל-widget
יגרום ל-widget של התגובות להתעדכן. ניתן לראות זאת בפעולה עבור אפליקציית ה-React, למשל, <a href="https://github.com/FastComments/fastcomments-react/blob/master/example/src/PaginatedApp.tsx#L39" target="_blank">כאן</a>.

#### VanillaJS

אם אתם משתמשים בספריית VanillaJS זה קצת יותר מורכב מאחר ואין מסגרת כמו Angular או React
לטיפול ב-data binding או בהפצת ה-state.

כאשר אתם מאתחלים את ה-widget של VanillaJS, הוא מחזיר מספר פונקציות שניתן לקרוא להן כדי לעדכן אותו.

הנה דוגמה פונקציונלית שבה אנו משנים את ה-hash של הדף ומעדכנים את ה-widget של התגובות:

[inline-code-attrs-start title = 'דוגמה לשינוי ה-hash של הדף'; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
<button id="change-page"></button>
<div id="fastcomments-widget"></div>
<script>
    (function fastCommentsMain() {
        let config = {
            tenantId: 'demo'
        };
        let instance = window.FastCommentsUI(document.getElementById('fastcomments-widget'), config);

        let page = '#page-1';
        function getNextPage() {
            if (page === '#page-1') {
                return '#page-2';
            } else {
                return '#page-1';
            }
        }

        let button = document.getElementById('change-page');
        function nextPage() {
            page = getNextPage();
            button.innerText = 'Go to ' + getNextPage();
            window.location.hash = page;
            let locationString = window.location.toString();

            config.url = locationString; // We update url, too, so notifications can link back to the right page
            config.urlId = locationString;

            instance.update(config);
        }
        nextPage();
        button.addEventListener('click', nextPage);
    })();
</script>
[inline-code-end]

---
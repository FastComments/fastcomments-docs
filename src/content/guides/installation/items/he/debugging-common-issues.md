להלן כמה תסמינים שאנו נתקלים בהם לעתים קרובות ופתרונות נפוצים. 

### הודעה "This is a demo"

זה מוצג כאשר העתקת את קוד הווידג'ט מהדף הראשי שלנו, שמשתמש ב-tenant הדמו שלנו. כדי להשתמש ב-tenant שלך, העתק את קוד הווידג'ט מ[כאן](https://fastcomments.com/auth/my-account/get-acct-code).

### שגיאה "FastComments cannot load on this domain"

FastComments צריך לדעת אילו דומיינים שייכים לך כדי לאמת בקשות המשויכות לחשבונך. [עיינו בתיעוד שלנו](/guide-multiple-sites.html#add-domains-to-account) כדי לראות כיצד לפתור שגיאה זו (פשוט הוסיפו את תת-הדומיין + הדומיין המדויק לחשבונכם).

שימו לב שזה אמור להתרחש רק לאחר תום תקופת הניסיון. בתקופת הניסיון, כל הבקשות מדומיינים חדשים יתווספו אוטומטית לחשבונכם.

### תגובות שהועתקו לא מוצגות בהתקנות מותאמות

בדרך כלל זה קורה כאשר ההערות המיוצאות קשורות ל-`Page ID`, ואתה מעביר כתובת URL (או אין ערך, במקרה זה היא מייצרת כברירת מחדל את ה-URL של הדף).

תוכל לאבחן זאת על ידי [ייצוא ההערות שלך](https://fastcomments.com/auth/my-account/manage-data/export) וצפייה בעמודת `URL ID` (כעת עמודה `B`).

ודא שהערכים שאתה רואה בעמודת `URL ID` הם אותם הערכים שאתה מעביר לקונפיגורציית הווידג'ט כפרמטר `urlId`.

להסבר נוסף, נסה לקרוא את [תיעוד: כיצד תגובות מקושרות לדפים ומאמרים](/guide-customizations-and-configuration.html#url-id).

אם כל השאר נכשל, [פנו אלינו](https://fastcomments.com/auth/my-account/help).

### הווידג'ט של ההערות לא מוצג

אם ווידג'ט ההערות לא מוצג, בדוק את קונסול המפתחים של Chrome לשגיאות.

ברוב מקרי התצורה השגויה, הווידג'ט של ההערות לפחות יציג שגיאה בעמוד אם הוא מצליח להיטען. אי־הצגה כללית בדרך כלל מעידה על שגיאת סקריפט.

### התצורה הרצויה לא פועלת כמצופה

נסו את [ההרחבה של Chrome](https://chrome.google.com/webstore/detail/fastcomments-debugger/cadggdemhfkjjghkdbfhonoccnplffjj?hl=en-US) כדי לראות איזו תצורה הווידג'ט מקבל. אם כלום לא עוזר, צלמו צילום מסך של מה שההרחבה מציגה ו[פנו אלינו](https://fastcomments.com/auth/my-account/help).

### חסרות תגובות באותו URL עם Hash Bang שונה

ברירת המחדל, FastComments ישתמש ב-URL של הדף כ"פתיחה" (bucket) לאחסון התגובות. אם ה-URLים שלך כוללים `#hashbangs`, ואלה לא אמורים להיות חלק מהמזהה שמזהה את שרשור התגובות, נוכל פשוט להתעלם מערך ה-hash bang, לדוגמה:

[inline-code-attrs-start title = 'התעלמות מ־Hash Bangs — דוגמה'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script async src="https://cdn.fastcomments.com/js/embed-v2-async.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
window.fcConfigs = [{
    target: '#fastcomments-widget',
    tenantId: "demo",
    url: location.href.replace(location.hash, ''),
    urlId: location.href.replace(location.hash, '')
}];
</script>
[inline-code-end]

שימו לב כי לאחר ביצוע שינוי זה, יידרש לבצע מיגרציה עבור ההערות הקיימות. [לשם כך, פנו אלינו.](https://fastcomments.com/auth/my-account/help)

### פרמטרי שאילתה ב-URL שמשפיעים על הווידג'ט

ברירת המחדל, FastComments ישתמש ב-URL של הדף כ"פתיחה" (bucket) לאחסון התגובות. אם ה-URLים שלך כוללים פרמטרי שאילתה שאינם אמורים להיות חלק מהמזהה שמזהה את שרשור התגובות, נוכל פשוט להתעלם מהם, לדוגמה:

[inline-code-attrs-start title = 'התעלמות מפרמטרי שאילתה'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script async src="https://cdn.fastcomments.com/js/embed-v2-async.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
window.fcConfigs = [{
    target: '#fastcomments-widget',
    tenantId: "demo",
    url: location.protocol + '//' + location.host + location.pathname,
    urlId: location.pathname
}];
</script>
[inline-code-end]

שימו לב כי לאחר ביצוע שינוי זה, יידרש לבצע מיגרציה עבור ההערות הקיימות. [לשם כך, פנו אלינו.](https://fastcomments.com/auth/my-account/help)

### לא מקבלים אימיילים

ב-FastComments אנו משקיעים מאמצים רבים כדי להבטיח שהשליחה של האימיילים תהיה אמינה ככל האפשר. עם זאת, ישנם ספקי אימייל שקשה מאוד למסור אליהם בצורה אמינה. בדקו את תיקיית הספאם עבור הודעות מ-fastcomments.com.

אם [פניתם אלינו](https://fastcomments.com/auth/my-account/help) אנו בדרך כלל יכולים לספק תובנות נוספות מדוע ייתכן שאינכם רואים אימיילים מאיתנו.

---
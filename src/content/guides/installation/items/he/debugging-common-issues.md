הנה כמה תסמינים שאנחנו פוגשים לעתים קרובות, ופתרונות נפוצים.

### הודעת "This is a demo"

זה מוצג כאשר העתקת את קוד הווידג'ט מדף הבית שלנו, שמשתמש בדייר הדמו
שלנו. כדי להשתמש בדייר שלך, העתק את קוד הווידג'ט [מכאן](https://fastcomments.com/auth/my-account/get-acct-code).

### שגיאת "FastComments cannot load on this domain"

FastComments צריך לדעת אילו דומיינים שייכים לך כדי לאמת בקשות המשויכות
לחשבון שלך. [עיין בתיעוד שלנו](/guide-multiple-sites.html#add-domains-to-account) כדי לראות איך
לפתור שגיאה זו (פשוט הוסף את התת-דומיין + דומיין המדויקים לחשבון שלך).

שים לב שזה אמור לקרות רק לאחר שתקופת הניסיון הסתיימה. במהלך תקופת הניסיון, כל בקשות מדומיינים חדשים
יתווספו אוטומטית לחשבון שלך.

### תגובות שהועברו לא מוצגות להתקנות מותאמות

בדרך כלל זה קורה כאשר התגובות המיובאות מקושרות ל-`Page ID`, ואתה מעביר URL
(או אין ערך, ובמקרה זה הוא משתמש ב-URL של הדף).

אתה יכול לבצע דיבוג על ידי [ייצוא התגובות שלך](https://fastcomments.com/auth/my-account/manage-data/export) וצפייה בעמודת `URL ID` (כרגע עמודה `B`).

ודא שהערכים שאתה רואה בעמודת `URL ID` הם אותם ערכים שאתה מעביר להגדרות הווידג'ט
כפרמטר `urlId`.

להסבר נוסף, נסה לקרוא את [התיעוד שלנו על איך תגובות מקושרות לדפים ומאמרים](/guide-customizations-and-configuration.html#url-id).

אם כל השאר נכשל, [צור איתנו קשר](https://fastcomments.com/auth/my-account/help).

### ווידג'ט התגובות לא מוצג

אם ווידג'ט התגובות לא מוצג, בדוק את קונסולת המפתחים של Chrome לשגיאות.

עבור רוב התצורות השגויות, ווידג'ט התגובות יציג לפחות שגיאה בדף אם הוא
יכול לטעון. לראות כלום הוא בדרך כלל אינדיקציה לשגיאת סקריפט.

### הגדרות רצויות לא עובדות כצפוי

נסה את [תוסף Chrome](https://chrome.google.com/webstore/detail/fastcomments-debugger/cadggdemhfkjjghkdbfhonoccnplffjj?hl=en-US) שלנו כדי לראות אילו
הגדרות מועברות לווידג'ט התגובות. אם כל השאר נכשל, צלם צילום מסך של מה שתוסף Chrome אומר
ו[צור איתנו קשר](https://fastcomments.com/auth/my-account/help).

### תגובות חסרות באותו URL עם hash bang שונה

כברירת מחדל, FastComments ישתמש ב-URL של הדף עבור ה-"bucket" בו התגובות מאוחסנות. אם ה-URLs שלך כוללים `#hashbangs`, ו-`#hashbangs` אלו
לא אמורים להיות חלק מהמזהה שמזהה שרשור תגובות, אנחנו יכולים פשוט להתעלם מערך ה-hash bang, לדוגמה:

[inline-code-attrs-start title = 'דוגמה להתעלמות מ-hash bangs'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
window.FastCommentsUI(document.getElementById('fastcomments-widget'), {
    tenantId: "demo",
    url: location.href.replace(location.hash, ''),
    urlId: location.href.replace(location.hash, '')
});
</script>
[inline-code-end]

שים לב שלאחר שינוי זה, יהיה צורך לבצע מיגרציה לתגובות קיימות. [לשם כך, צור איתנו קשר.](https://fastcomments.com/auth/my-account/help)

### פרמטרים של URL query משפיעים על הווידג'ט

כברירת מחדל, FastComments ישתמש ב-URL של הדף עבור ה-"bucket" בו התגובות מאוחסנות. אם ה-URLs שלך כוללים פרמטרי query
שלא אמורים להיות חלק מהמזהה שמזהה שרשור תגובות, אנחנו יכולים פשוט להתעלם מהם, לדוגמה:

[inline-code-attrs-start title = 'התעלמות מפרמטרי query'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
window.FastCommentsUI(document.getElementById('fastcomments-widget'), {
    tenantId: "demo",
    url: location.protocol + '//' + location.host + location.pathname,
    urlId: location.pathname
});
</script>
[inline-code-end]

שים לב שלאחר שינוי זה, יהיה צורך לבצע מיגרציה לתגובות קיימות. [לשם כך, צור איתנו קשר.](https://fastcomments.com/auth/my-account/help)

### לא מקבל מיילים

ב-FastComments, אנו משקיעים מאמץ רב כדי להבטיח שמשלוח המיילים שלנו יהיה אמין ככל האפשר.
עם זאת, חלק מספקי המייל ידועים כקשים למשלוח אמין. בדוק את תיקיית הספאם שלך
להודעות מ-fastcomments.com.

אם [תיצור איתנו קשר](https://fastcomments.com/auth/my-account/help) אנחנו בדרך כלל יכולים לספק
יותר תובנות למה אתה אולי לא רואה מיילים מאיתנו.

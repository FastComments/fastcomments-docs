### ה-API של FastComments

FastComments מספקת API לאינטראקציה עם משאבים רבים. בנו אינטגרציות עם הפלטפורמה שלנו, או אפילו בנו לקוחות משלכם!

בתיעוד זה תמצאו את כל המשאבים הנתמכים על־ידי ה-API המתועדים עם סוגי הבקשות והתשובות שלהם.

עבור לקוחות ארגוניים, כל הגישה ל-API מתועדת ביומן הביקורת.

### SDKs שנוצרו

כעת FastComments מייצרת [מפרט API](https://fastcomments.com/js/swagger.json) מקוד המקור שלנו (זה עדיין לא שלם, אך כולל רבות מה-APIs).

כמו כן, יש לנו כעת SDKs לשפות פופולריות:

- [fastcomments-cpp](./guide-sdk-cpp.html)
- [fastcomments-go](./guide-sdk-go.html)
- [fastcomments-java](./guide-sdk-java.html)
- [fastcomments-sdk-js](./guide-sdk-javascript.html)
- [fastcomments-nim](./guide-sdk-nim.html)
- [fastcomments-php](guide-sdk-php.html)
- [fastcomments-php-sso](./guide-sdk-php-sso.html)
- [fastcomments-python](./guide-sdk-python.html)
- [fastcomments-ruby](./guide-sdk-ruby.html)
- [fastcomments-rust](./guide-sdk-rust.html)
- [fastcomments-swift](./guide-sdk-swift.html)

### אימות

ה-API מאומת על ידי העברת [מפתח ה-API](https://fastcomments.com/auth/my-account/api-secret) כאחת מאלה: כותרת `X-API-KEY` או פרמטר שאילתה `API_KEY`. בנוסף תזדקקו ל-`tenantId` כדי לבצע קריאות API. ניתן להשיג זאת מאותה עמוד שבו נמצא מפתח ה-API שלכם.

### הערת אבטחה

נתיבים אלו מיועדים להיקרא מ**שרת**. __אל תקראו__ אותם מדפדפן. פעולה זו תחשוף את מפתח ה-API שלכם - זה יספק גישה מלאה לחשבון שלכם לכל מי שיכול לצפות בקוד המקור של דף!

#### Authentication Option One - Headers

- כותרת: `X-API-KEY`
- כותרת: `X-TENANT-ID`

#### Authentication Option Two - Query Parameters

- פרמטר שאילתה: `API_KEY`
- פרמטר שאילתה: `tenantId`

---
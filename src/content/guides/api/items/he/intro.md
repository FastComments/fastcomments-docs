### ממשק ה-API של FastComments

FastComments מספקת ממשק API לאינטראקציה עם משאבים רבים. צרו אינטגרציות עם הפלטפורמה שלנו, או אפילו צרו לקוחות משלכם!

בתיעוד זה תמצאו את כל המשאבים הנתמכים על ידי ה-API שתועדו עם סוגי הבקשה והתשובה שלהם.

ללקוחות Enterprise, כל הגישה ל-API נרשמת ביומן ביקורת.

### SDKs שנוצרו

FastComments מייצרת כעת מפרט [API](https://fastcomments.com/js/swagger.json) מתוך הקוד שלנו (זה עדיין לא שלם, אך כולל ממשקי API רבים).

יש לנו גם SDKs לשפות נפוצות:

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

ה-API מאומת על ידי העברת [מפתח ה-API](https://fastcomments.com/auth/my-account/api-secret) שלך ככותרת `X-API-KEY` או כפרמטר שאילתה `API_KEY`. תזדקקו גם ל-`tenantId` כדי לבצע קריאות ל-API. ניתן לקבל אותו מאותה עמוד שבו נמצא מפתח ה-API שלכם.

### הערת אבטחה

ניתובים אלה מיועדים להיקרא מ**שרת**. __אסור__ לקרוא להם מדפדפן. פעולה כזו תחשוף את מפתח ה-API שלכם — וזה ייתן גישה מלאה לחשבון שלכם לכל מי שיכול לצפות בקוד המקור של דף!

#### אפשרות אימות אחת - כותרות

- כותרת: `X-API-KEY`
- כותרת: `X-TENANT-ID`

#### אפשרות אימות שנייה - פרמטרי שאילתה

- פרמטר שאילתה: `API_KEY`
- פרמטר שאילתה: `tenantId`

### קריאה של הכתיבות שלך

FastComments מספקת זמינות Active-Active. בקשות ממרכז הנתונים שלכם מנותבות אל [נקודת הנוכחות הקרובה](https://sophon.fastcomments.com/) אליכם. זה אוטומטי, ובדרך כלל תוכלו להבחין בסמנטיקה של קריאה אחרי כתיבה. אם ברצונכם להיות בטוחים שנקרא את הכתיבות שלכם, תוכלו לקבע את הבקשות לאזור מסוים על ידי שימוש באזור זה כ-host של ה-API (עם זאת, בדרך כלל זה לא נדרש עבור רוב האינטגרציות):

- gdc-oregon.fastcomments.com
- gdc-virginia.fastcomments.com
- gdc-singapore.fastcomments.com
- gdc-falkenstein2.fastcomments.com
- gdc-sao-paulo.fastcomments.com
- eudc-helsinki2.fastcomments.com
- eudc-limburg.fastcomments.com
- eudc-france.fastcomments.com

שימו לב שאם תעשו זאת ייתכן שתרצו להגדיר נקודת גיבוי, שכן בעבר הסרנו צמתים של נקודות כניסה והשתמשנו בשמות חדשים בעת המעבר.
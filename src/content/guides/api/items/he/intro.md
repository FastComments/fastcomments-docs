### ממשק ה-API של FastComments

FastComments מספקת API לתקשר עם משאבים רבים. בנו אינטגרציות עם הפלטפורמה שלנו, או אפילו בנו לקוחות משלכם!

בתיעוד זה, תמצאו את כל המשאבים הנתמכים על ידי ה-API מתועדים עם סוגי הבקשות והתגובות שלהם.

ללקוחות Enterprise, כל הגישה ל-API נרשמת ביומן הביקורת.

### SDKs שנוצרו

FastComments מייצרת כעת [API Spec](https://fastcomments.com/js/swagger.json) מהקוד שלנו (זה עדיין לא מושלם, אך כולל הרבה APIs).

אנו גם מציעים כעת SDKs לשפות פופולריות:

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

ה-API מאומת על ידי העברת [מפתח ה-API](https://fastcomments.com/auth/my-account/api-secret) שלך כאחת מהאפשרויות: ככותרת `X-API-KEY` או כפרמטר שאילתה `API_KEY`. תצטרכו גם את `tenantId` שלכם לביצוע קריאות API. ניתן לקבל זאת מאותה העמוד שבו נמצא מפתח ה-API שלכם.

### הערת אבטחה

נתיבים אלה מיועדים לקריאה מ**שרת**. __אל תקריאו__ אותם מדפדפן. פעולה כזו תחשוף את מפתח ה-API שלכם - זה ייתן גישה מלאה לחשבון שלכם לכל מי שיכול לראות את קוד המקור של דף!

#### אפשרות אימות ראשונה - כותרות

- כותרת: `X-API-KEY`
- כותרת: `X-TENANT-ID`

#### אפשרות אימות שנייה - פרמטרי שאילתה

- פרמטר שאילתה: `API_KEY`
- פרמטר שאילתה: `tenantId`

#### אפשרות אימות שלישית - אסימון נושא OAuth

- כותרת: `Authorization: Bearer fcat_...`

אפליקציות שמתחברות דרך [שרת MCP](https://docs.fastcomments.com/guide-llm-kit.html) מקבלות אסימון דרך OAuth במקום מפתח API. אסימון זה פועל על כל נקודת קצה כאן. ה‑tenant נרמז מהאסימון, ולכן `tenantId` הוא אופציונלי, אך אם נמסר חייב להתאים לאסימון. בקשות `GET` דורשות את ההקשר `read` וכל שיטה אחרת דורשת את ההקשר `write`. הגילוי מתחיל ב-`https://fastcomments.com/.well-known/oauth-authorization-server`.

### קריאת הכתיבות שלך

FastComments מספקת זמינות Active-Active. בקשות ממרכז הנתונים שלכם מנותבות ל[נקודת הנוכחות הקרובה ביותר](https://sophon.fastcomments.com/) אליכם. זה מתבצע אוטומטית, ובדרך כלל ניתן לצפות לתחביר קריאה-כתיבה (read-your-write). אם ברצונכם לוודא קריאת הכתיבות שלכם, תוכלו להצמיד את הבקשות לאזור מסוים על ידי שימוש באותו אזור כמארח ה-API (עם זאת, זה בדרך כלל אינו נדרש ברוב האינטגרציות):

- gdc-oregon.fastcomments.com
- gdc-virginia.fastcomments.com
- gdc-singapore.fastcomments.com
- gdc-falkenstein2.fastcomments.com
- gdc-sao-paulo.fastcomments.com
- eudc-helsinki2.fastcomments.com
- eudc-limburg.fastcomments.com
- eudc-france.fastcomments.com

שימו לב שאם תעשו זאת, ייתכן שתרצו להגדיר fallback, מכיוון שהשמטנו בעבר נקודות כניסה והשתמשנו בשמות חדשים למעבר.
### ה-API של FastComments

FastComments מספקת API לתקשר עם משאבים רבים. בנו אינטגרציות עם הפלטפורמה שלנו, או אפילו בנו לקוחות משלכם!

בתיעוד זה, תמצאו את כל המשאבים הנתמכים על‑ידי ה‑API מתועדים עם סוגי הבקשות והתגובות שלהם.

ללקוחות Enterprise, כל הגישה ל‑API נרשמת ביומן הבדיקה.

### SDKs שנוצרו אוטומטית

FastComments מייצרת כעת [API Spec](https://fastcomments.com/js/swagger.json) מהקוד שלנו (זה עדיין לא מושלם, אך כולל הרבה APIs).

כמו כן, יש לנו עכשיו SDKs לשפות פופולריות:

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

ה‑API מאומת על‑ידי העברת [api key](https://fastcomments.com/auth/my-account/api-secret) שלך כאחת מהאפשרויות: ככותרת `X-API-KEY` או כפרמטר שאילתה `API_KEY`. תצטרך גם את `tenantId` שלך לביצוע קריאות API. ניתן לקבל אותו מאותה העמוד שבו נמצא מפתח ה‑API שלך.

### הערת אבטחה

נתיבים אלה נועדו להיקרא מ**שרת**. __אל תקראו אותם מדפדפן__. קריאה מהדפדפן תחשוף את מפתח ה‑API שלך – זה ייתן גישה מלאה לחשבון שלך לכל מי שיכול לראות את קוד המקור של העמוד!

#### אפשרות אימות ראשונה – כותרות

- Header: `X-API-KEY`
- Header: `X-TENANT-ID`

#### אפשרות אימות שנייה – פרמטרי שאילתה

- Query Param: `API_KEY`
- Query Param: `tenantId`

#### אפשרות אימות שלישית – OAuth Bearer Token

- Header: `Authorization: Bearer fcat_...`

יישומים של צד שלישי כגון Zapier ולקוחות של [MCP server](https://docs.fastcomments.com/guide-llm-kit.html) מקבלים אסימון דרך OAuth במקום מפתח API. אסימון זה פועל על כל קצה כאן. ה‑tenant נכלל באסימון, ולכן `tenantId` הוא אופציונלי, אך אם נמסר הוא חייב להתאים לאסימון. בקשות `GET` דורשות את ההרשאה `read` וכל שיטה אחרת דורשת את ההרשאה `write`. הזרימה המלאה, כולל רישום לקוח, PKCE, רענון וביטול, מתועדת תחת [OAuth Authorization](#oauth). הגילוי מתחיל ב‑`https://fastcomments.com/.well-known/oauth-authorization-server`.

### קריאת הכתבות שלך

FastComments מספקת זמינות Active‑Active. בקשות ממרכז הנתונים שלך מנותבות ל[נקודת הנוכחות הקרובה ביותר](https://sophon.fastcomments.com/) אליך. זה מתבצע אוטומטית, ובדרך כלל ניתן לצפות להתנהגות read‑your‑write. אם ברצונך לוודא שאתה קורא את הכתבות שלך, אתה יכול להצמיד את הבקשות לאזור מסוים על‑ידי שימוש באותו אזור כמארח ה‑API (עם זאת, זה בדרך כלל אינו נדרש ברוב האינטגרציות):

- gdc-oregon.fastcomments.com
- gdc-virginia.fastcomments.com
- gdc-singapore.fastcomments.com
- gdc-falkenstein2.fastcomments.com
- gdc-sao-paulo.fastcomments.com
- eudc-helsinki2.fastcomments.com
- eudc-limburg.fastcomments.com
- eudc-france.fastcomments.com

שימו לב שאם תעשו זאת, ייתכן שתרצו להגדיר fallback, שכן בעבר הפסקנו להשתמש בנקודות כניסה והשתמשנו בשמות חדשים למעבר.
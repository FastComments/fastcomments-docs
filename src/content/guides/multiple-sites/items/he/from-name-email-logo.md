לפעמים FastComments צריך לשלוח אימיילים למשתמשים שלך, במיוחד אם אינך משתמש ב- Secure SSO.

דוגמאות לכך כוללות אימות החשבון שלהם או אימות פעילות כאשר הם מגיבים בפעם הראשונה. FastComments גם ישלח להם התראות על תגובות להערותיהם.

כש-FastComments שולח למשתמשים שלך אימיילים, נשתמש בשם ושורת ה-From ברירת מחדל של `FastComments Robot` ו-`noreply@fastcomments.com`.

אנחנו גם נשתמש בלוגו שלנו בכותרת התחתונה של האימיילים האלה.

אם יש לך FastComments Flex או Pro, ניתן להתאים זאת לכל דומיין בנפרד דרך דף "My Domains page":

[app-screenshot-start url='/auth/my-account/configure-domains'; selector = '.content form'; title='Customizing From Name, Email, and Logo' app-screenshot-end]

כאשר אתה מתאים את הלוגו המוצג באימיילים, ודא שהגודל שאתה מעלה הוא אותו הגודל שברצונך להציג בכותרת התחתונה של האימייל.

### When Customizing The `From Domain`

אם אתה מתאים את ה- `From Domain`, ספקי דוא"ל ולקוחות צריכים לדעת ש-FastComments מורשה לשלוח אימיילים בשם הדומיין שלך. אחרת,
הגדרת ה- `From Domain` ולא ביצוע השלבים למטה עשויה לגרום לכך שהאימיילים יגיעו לתיקיית הספאם.

#### 1. Setup SPF

כדי לאפשר ל-FastComments לשלוח אימייל באופן מאובטח בשם הדומיין שלך, ודא שאתה מוסיף רשומת SPF שמאפשרת לנו לעשות זאת.

ודא שקיימות רשומות SPF שמאפשרות ל-`mail.fastcomments.com` ו-`sib.fastcomments.com` לשלוח דוא"ל בשם הדומיין שלך.

מידע נוסף על איך לעשות זאת נמצא כאן: https://mailtrap.io/blog/multiple-spf-records/

#### 2. Setup DKIM

בנוסף ל-SPF, כדאי להגדיר גם DKIM. לאחר שהקונפיגורציה של ה-DNS מוכנה, תוכל ללחוץ על "Show Advanced" בעמוד קונפיגורציות הדומיין
כדי להציג את הגדרות ה-DKIM לכל דומיין.

אתה יכול גם [invoke the API](/guide-api.html#domain-config-structure) כדי להגדיר את קונפיגורציית ה-DKIM.

### Unsubscribe Links

כאשר משתמשים ב-SSO, תכונות הסירוב לרישום שמשתמשים באימיילים ובהתראות ניתנות להתאמה [via the DomainConfigs API](/guide-api.html#domain-config-structure).

### Email Link Obfuscation

אם המוניטין של דומיין האתר שלך גורם לכך שאימיילי התראות נופלים לספאם, אתה יכול לנתב את כפתורי "view comment" דרך `fastcomments.com` במקום לקשר ישירות לדף שלך. ספקי תיבות דואר מדרגים כל קישור בגוף האימייל ביחס למוניטין היעד, כך שכאשר הדומיין שלך מסומן, הקישורים הגלויים תורמים לציון הספאם ללא קשר לניקיון הגדרות השילוח שלך.

אפשר להפעיל זאת תחת "Show Advanced" בדף My Domains, בסעיף "Email Link Obfuscation". ההגדרה היא לכל דומיין בנפרד.

כאשר מופעלת, קישורים באימיילים מסוג mention, reply, new-comment, subscribed-page, profile-comment ו-digest נכתבים מחדש לטוקנים קצרים שמפנים לדף המקורי בעת לחיצה. היעד קשור ל-tenant שלך: ההפניה רק מעבירה ל-URLs שב-host שלהם תואם לאחד הדומיינים שהגדרת, והטוקנים פוקעים אוטומטית לאחר 30 ימים.

חוויית הלחיצה לא משתנה. הקוראים עדיין נוחתים על הדף שלך עם התגובה מגוללת לתצוגה.
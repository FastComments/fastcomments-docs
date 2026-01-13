לפעמים FastComments צריך לשלוח אימיילים למשתמשים שלך, במיוחד אם אינך משתמש ב‑Secure SSO.

דוגמאות לכך כוללות אימות חשבון או פעילות כאשר הם מגיבים לראשונה. FastComments גם יישלח להם התראות על תגובות לתגובותיהם.

כש‑FastComments ישלח אימיילים למשתמשים שלך, נשמש בשם ובאימייל ברירת מחדל של `FastComments Robot` ו־`noreply@fastcomments.com`.

נשתמש גם בלוגו שלנו בתחתית האימיילים האלה.

אם יש לך FastComments Flex או Pro, כל זה ניתן להתאמה על בסיס דומיין באמצעות דף "My Domains":

[app-screenshot-start url='/auth/my-account/configure-domains'; selector = '.content form'; title='Customizing From Name, Email, and Logo' app-screenshot-end]

כשאתה מתאים את הלוגו שמוצג באימיילים, ודא שהגודל שאתה מעלה הוא אותו הגודל שברצונך להציג בתחתית האימייל.

### בעת התאמה של ה`From Domain`

אם תשנה את ה`From Domain`, ספקי ויישומי דוא"ל צריכים לדעת ש‑FastComments מורשה לשלוח אימיילים מטעם הדומיין שלך. אחרת, הגדרת ה`From Domain` ללא ביצוע הצעדים הבאים כנראה תגרום שהאימיילים יגיעו לספאם.

#### 1. הגדר SPF

כדי לאפשר ל‑FastComments לשלוח אימיילים באופן מאובטח בשם הדומיין שלך, ודא שאתה מוסיף רשומת SPF שמאפשרת לנו לעשות זאת.

וודא שיש רשומות SPF שמאפשרות ל־`mail.fastcomments.com` ו־`sib.fastcomments.com` לשלוח דואר בשם הדומיין שלך.

מידע נוסף על איך לעשות זאת זמין כאן: https://mailtrap.io/blog/multiple-spf-records/

#### 2. הגדר DKIM

בנוסף ל‑SPF, עליך להגדיר DKIM. ברגע שהקונפיגורציית ה‑DNS מוכנה, תוכל ללחוץ על 'Show Advanced' בדף הגדרות הדומיין כדי להציג את הגדרות DKIM לכל דומיין.

ניתן גם [להפעיל את ה‑API](/guide-api.html#domain-config-structure) כדי להגדיר את קונפיגורציית DKIM.

### קישורי הסרה

כאשר משתמשים ב‑SSO, תכונות ההסרה מהרשמה שמשמשות באימיילים ובהודעות ניתנות להתאמה [באמצעות ה‑DomainConfigs API](/guide-api.html#domain-config-structure).

---
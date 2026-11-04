לאחר ש-FastComments נרשם בפלטפורמה, מדריכים מוסיפים אותו לתוכן הקורס באמצעות זרימות כלי חיצוני סטנדרטיות של הפלטפורמה. עמוד זה מכסה את Sakai 23.x ו-Schoology Enterprise.

#### נעילת גישה ציבורית (מומלץ)

כברירת מחדל, נתוני התגובות של FastComments קריאים לציבור בשתי הפלטפורמות. כל מי שיכול לנחש את ה-URL של שרשור או נקודת קצה של API יכול לצפות בתגובותיו, גם מחוץ ל-Sakai או Schoology. עבור דיוני קורס כמעט בוודאי תרצו להגביל את הצפייה לסטודנטים רשומים בלבד.

פתחו את <a href="https://fastcomments.com/auth/my-account/customize-widget" target="_blank">עמוד התאמה אישית של הווידג'ט</a> וצרו כלל עם **Require SSO To View Comments** מופעל, ואז הגדירו את רמת האבטחה ל-**Secure SSO** כך ששרשורים ייטענו רק דרך השקת ה-LTI החתומה.

ראו [הגנה על שרשורי תגובות באמצעות Single-Sign-On](/guide-customizations-and-configuration.html#sso-require-to-view-comments) להדרכה המלאה, כולל כיצד להגביל את הכלל לדומיין או עמוד בודדים.

#### Sakai

**1. הוספת FastComments לאתר**

מנהל האתר מפעיל את הכלי על בסיס אתר-אתר:

1. פתחו את האתר ולחצו על **Site Info** בניווט השמאלי.
2. לחצו על **Manage Tools**.
3. גללו לרשימת **External Tools** והפעילו את **FastComments**.
4. לחצו על **Continue**, סקרו את רשימת הכלים, ואז לחצו על **Finish**.

FastComments מופיע כעת כפריט ניווט שמאלי באתר.

**2. סידור מחדש של פריט הניווט השמאלי**

עברו אל **Site Info** > **Tool Order**. גררו את **FastComments** למיקום הרצוי ולחצו על **Save**. ניתן גם לשנות את שם התווית בניווט ולהסתיר אותה מסטודנטים ממסך זה.

**3. הטמעה בתוך עמוד Lessons**

כדי למקם את FastComments בתוך עמוד Lessons ישירות, במקום ככלי ניווט שמאלי עצמאי:

1. פתחו את כלי ה-**Lessons** באתר.
2. לחצו על **Add Content** > **Add External Tool**.
3. בחרו **FastComments** מהרשימה.
4. אם FastComments פרסם Deep Linking במהלך הרישום, Sakai פותח את בורר התוכן של הכלי כדי שתוכלו לבחור או לתייג את השרשור. אם Deep Linking לא פורסם, Sakai מכניס קישור השקה ברירת מחדל.
5. שמרו את פריט ה-Lessons.

לכל מופע מוטמע יש שרשור משלו, המוגבל לקישור המשאב ההוא.

**4. התאמות הרשאות לגישת סטודנטים**

Sakai מווסת השקות של כלים חיצוניים באמצעות Realms. כדי לוודא שסטודנטים יכולים להפעיל את FastComments:

1. התחברו כמנהל Sakai ופתחו **Administration Workspace** > **Realms**.
2. פתחו את ה-realm הרלוונטי (למשל, `!site.template.course` או ה-realm של האתר הספציפי).
3. ודאו שלתפקיד `access` יש `lti.launch` מופעל ושהרשאות התפקיד בקבוצת **external.tools** הוענקו.
4. שמרו את ה-realm.

לעקיפות ברמת האתר, מנהל האתר יכול להתאים את נראות הכלי לפי תפקיד מ-**Site Info** > **Tool Order** על ידי הסתרה או הצגה של FastComments לכל תפקיד.

**5. מה סטודנטים רואים**

סטודנטים לוחצים על פריט הניווט השמאלי של FastComments (או גוללים לבלוק ה-Lessons המוטמע) ומגיעים ישירות לתצוגת התגובות המשורשרות. ה-SSO אוטומטי: Sakai שולח את זהות המשתמש בהשקת ה-LTI ו-FastComments מחבר אותם תחת חשבון ה-Sakai שלהם.

מיפוי תפקידים:

- Sakai `Instructor` -> FastComments מנחה (moderator)
- Sakai `Admin` (מנהל ב-Administration Workspace) -> FastComments מנהל
- Sakai `Student` / `access` -> FastComments מגיב

**6. מלכודות ב-Sakai**

- **הכלי לא מופיע ב-Manage Tools.** אם FastComments לא מופיע ברשימת External Tools, מנהל ה-Sakai צריך לפתוח את רישום הכלים (**Administration Workspace** > **External Tools** > **FastComments**) ולהגדיר **Stealthed** ל-`false`. כלים Stealthed מוסתרים מבורר Manage Tools ברמת האתר.
- **השקות שנכשלות בדפדפנים עם סשן משותף.** אסימון ה-CSRF של פורטל Sakai קשור לסשן הדפדפן. אם סטודנט מחובר לשני אתרי Sakai בטאבים שונים או שיש לו סשן מיושן, ההשקה מחזירה 403. פתרון: סגרו טאבים אחרים של Sakai, התנתקו, התחברו מחדש, והפעילו שוב. מנהלים יכולים גם להעלות את `sakai.csrf.token.cache.ttl` אם זה קורה ברמת הקלאסטר.
- **הטמעה במסגרת.** ודאו ש-`lti.frameheight` ב-`sakai.properties` גדול מספיק (600 ומעלה) כדי ששרשור התגובות לא ייחתך בתוך עמוד Lessons.

#### Schoology

ל-Schoology Enterprise יש שני תרחישי התקנה. ודאו איזה מהם חל לפני הוספת הכלי לקורס.

**1. שני תרחישי התקנה**

- **(a) התקנה ברמת Enterprise.** מנהל המערכת של Schoology התקין את FastComments ברמת הארגון והקצה אותו לכל הקורסים או לתבניות קורס ספציפיות. מדריכים מדלגים על ההתקנה ועוברים ישירות ל-"Add Materials".
- **(b) התקנה עצמית של המדריך.** המדריך מתקין את הכלי לקורס בודד מ-**Course Options** > **External Tools** > **Install LTI Apps**. התקנה עצמית דורשת שמנהל המערכת אישר קודם את אפליקציית FastComments ברמת הארגון.

**2. הוספת FastComments כחומר קורס**

בתוך הקורס:

1. פתחו את הקורס ועברו אל **Materials**.
2. לחצו על **Add Materials** > **Add File/Link/External Tool**.
3. בחרו **External Tool**.
4. בחרו **FastComments** מרשימת הכלים הרשומים.
5. הגדירו **Name** (זה מה שסטודנטים רואים ברשימת החומרים) ו-**Description** אופציונלי.
6. השאירו את **Enable Grading** (העברת ציונים) **כבוי**. FastComments אינו מדווח ציונים חזרה ל-Schoology, ולכן הפעלת העברת ציונים יוצרת עמודה ריקה במחברת הציונים.
7. לחצו על **Submit**.

החומר מופיע כעת ברשימת חומרי הקורס ופותח את שרשור FastComments בלחיצה.

**3. הטמעה בתוך השורה באמצעות עורך ה-Rich Text**

אם מנהל המערכת הפעיל מיקום Deep Linking עבור FastComments במהלך הרישום, מדריכים יכולים להטמיע את שרשור התגובות בתוך כל שדה Rich Text (הוראות מטלה, גופי עמודים, פרומפטים לדיון):

1. פתחו את עורך ה-Rich Text בעמוד היעד.
2. לחצו על סמל **External Tool** (חתיכת פאזל) בסרגל הכלים.
3. בחרו **FastComments**.
4. הגדירו את ההטמעה בדיאלוג ה-deep-linking ולחצו על **Insert**.
5. שמרו את העמוד.

אם כפתור External Tool לא מופיע בעורך ה-Rich Text, Deep Linking מושבת עבור הכלי הזה בטננט הזה. ראו את המלכודות למטה.

**4. נראות והקצאות סקשנים**

Schoology מגביל זמינות כלים לפי סקשן באמצעות Course Options:

1. מהקורס, לחצו על **Course Options** > **External Tools**.
2. עבור כל אפליקציית LTI מותקנת, אתם שולטים האם היא זמינה לכל הסקשנים בקורס או לסקשנים ספציפיים.
3. כדי להגביל את FastComments לסקשנים מסוימים, בטלו את הסימון של הסקשנים שלא אמורים לראות את הכלי.
4. גישה ברמת סקשן גם קובעת אילו סקשנים רואים את הפריט **Add Materials** > **External Tool** עבור FastComments.

**5. מה סטודנטים רואים**

סטודנטים לוחצים על החומר של FastComments (או גוללים להטמעה בתוך השורה) ומגיעים לדיון המשורשר. ה-SSO אוטומטי דרך השקת ה-LTI של Schoology תחת חשבון ה-Schoology שלהם.

מיפוי תפקידים:

- Schoology `Administrator` -> FastComments מנהל
- Schoology `Instructor` -> FastComments מנחה (moderator)
- Schoology `Student` -> FastComments מגיב

**6. מלכודות ב-Schoology**

- **Enterprise בלבד.** חשבונות Schoology אישיים וחינמיים אינם יכולים להתקין כלי LTI 1.3. אם הטננט שלכם ברמה החינמית, האפשרות **External Tools** חסרה ב-Course Options. שדרגו ל-Schoology Enterprise כדי להשתמש ב-FastComments.
- **Deep Linking מושבת כברירת מחדל בטננט.** חלק מטננטים של Schoology מגבילים מיקום Deep Linking ברמת הארגון. כשזה המצב, מדריכים רואים רק את זרימת **Add Materials** > **External Tool** ולא את כפתור External Tool בעורך ה-Rich Text. כדי לאפשר הטמעה בתוך השורה, מנהל המערכת עובר אל **System Settings** > **Integration** > **LTI 1.3** > **FastComments** ומפעיל את מיקום **Content Item / Deep Linking**, ואז שומר.
- **עקיפת הקצאה ברמת סקשן.** אם FastComments מוקצה ברמת הארגון אך המדריך אינו רואה אותו ב-**Add Materials**, הסקשן של הקורס נכלל בשלילה בהקצאה ברמת הארגון. בקשו ממנהל המערכת להוסיף את הסקשן להקצאת אפליקציית FastComments.
- **שם החומר מול זהות השרשור.** שינוי שם החומר ב-Schoology אינו מזיז את שרשור התגובות. שרשורים ממופתחים לפי מזהה קישור המשאב של ה-LTI, ולכן שינוי שם שומר על אותו שרשור; מחיקה ויצירה מחדש של החומר יוצרת שרשור חדש וריק.
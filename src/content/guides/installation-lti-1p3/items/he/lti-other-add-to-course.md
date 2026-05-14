ברגע ש-FastComments נרשם בפלטפורמה, המנחים מוסיפים אותו לתוכן הקורס באמצעות זרימות הכלים החיצוניים הסטנדרטיות של הפלטפורמה. עמוד זה מכסה את Sakai 23.x ואת Schoology Enterprise.

#### Sakai

**1. הוספת FastComments לאתר**

האחראי על האתר מאפשר את הכלי על בסיס לכל אתר:

1. פתח את האתר ולחץ על **Site Info** בניווט השמאלי.
2. לחץ על **Manage Tools**.
3. גלול לרשימת **External Tools** והחלף את **FastComments** למצב מופעל.
4. לחץ על **Continue**, בדוק את רשימת הכלים, ואז לחץ על **Finish**.

כעת FastComments מופיע כפריט בתפריט השמאלי באתר.

**2. סידור מחדש של ערך הניווט השמאלי**

גש ל- **Site Info** > **Tool Order**. גרור את **FastComments** למיקום הרצוי ולחץ **Save**. מסך זה מאפשר גם לשנות את תווית הניווט ולהסתיר אותה מתלמידים.

**3. הטמעה בתוך עמוד Lessons**

כדי למקם את FastComments ישירות בתוך עמוד Lessons במקום ככלי עצמאי בניווט השמאלי:

1. פתח את כלי **Lessons** באתר.
2. לחץ **Add Content** > **Add External Tool**.
3. בחר **FastComments** מהרשימה.
4. אם FastComments פרסם Deep Linking במהלך הרישום, Sakai פותח את בורר התוכן של הכלי כדי שתוכל לבחור או לתייג את השרשור. אם Deep Linking לא פורסם, Sakai מכניס קישור השקה ברירת-מחדל.
5. שמור את פריט ה-Lessons.

לכל מופע מוטמע נוצר שרשור עצמאי, המוגדר לפי קישור המשאב הזה.

**4. כוונוני הרשאות לגישה של תלמידים**

Sakai שולט בהשקות כלים חיצוניים דרך Realms. כדי לוודא שתלמידים יכולים להפעיל את FastComments:

1. היכנס כמנהל Sakai ופתח **Administration Workspace** > **Realms**.
2. פתח את ה-realm הרלוונטי (לדוגמה, `!site.template.course` או ה-realm של האתר הספציפי).
3. אמת של-role `access` יש את האפשרות `lti.launch` מופעלת ושההרשאות של התפקיד בקבוצת **external.tools** ניתנו.
4. שמור את ה-realm.

לרמות אתריות, המתחזק יכול להתאים את נראות הכלי לפי תפקידים מ- **Site Info** > **Tool Order** על ידי הסתרה או הצגה של FastComments לכל תפקיד.

**5. מה תלמידים רואים**

תלמידים לוחצים על פריט הניווט השמאלי של FastComments (או גוללים לחסום המוטמע ב-Lessons) ונכנסים ישירות לתצוגת השרשור של התגובות. SSO אוטומטית: Sakai שולח את זהות המשתמש בהשקת ה-LTI ו-FastComments מכניס אותם תחת חשבון ה-Sakai שלהם.

מיפוי תפקידים:

- Sakai `Instructor` -> FastComments moderator
- Sakai `Admin` (admin ב-Administration Workspace) -> FastComments admin
- Sakai `Student` / `access` -> FastComments commenter

**6. נקודות בעייתיות ב-Sakai**

- **הכלי לא נראה ב-Manage Tools.** אם FastComments לא מופיע ברשימת External Tools, מנהל ה-Sakai צריך לפתוח את רישום הכלים (**Administration Workspace** > **External Tools** > **FastComments**) ולהגדיר את **Stealthed** כ-`false`. כלים Stealthed מוסתרים מבורר ה-Manage Tools לכל אתר.
- **השקות שנכשלות בדפדפני מושב משותף.** טוקן ה-CSRF של הפורטל של Sakai קשור למושב הדפדפן. אם תלמיד מחובר לשני אתרי Sakai בכרטיסיות שונות או שיש לו מושב ישן, ההשקה תחזיר 403. פתרון: סגור כרטיסיות אחרות של Sakai, התנתק, התחבר שוב והפעל מחדש את ההשקה. מנהלים גם יכולים להגדיל את הערך של `sakai.csrf.token.cache.ttl` אם זה קורה ברמת הקלאסטר.
- **הטמעת מסגרות.** אמת את הערך של `lti.frameheight` בקובץ `sakai.properties` שהוא גדול מספיק (600 או יותר) כדי שהשרשור לא ייחתך בתוך עמוד Lessons.

#### Schoology

Schoology Enterprise כוללת שני תרחישי התקנה. אמת איזה מהם חלים לפני הוספת הכלי לקורס.

**1. שני תרחישי התקנה**

- **(a) התקנה ברמת הארגון.** מנהל המערכת של Schoology התקין את FastComments ברמת הארגון והקציב אותו לכל הקורסים או לתבניות קורס ספציפיות. המנחים מדלגים על ההתקנה ועוברים ישר ל-"Add Materials".
- **(b) התקנה עצמית על ידי המנחה.** המנחה מתקין את הכלי לקורס בודד דרך **Course Options** > **External Tools** > **Install LTI Apps**. התקנה עצמית מחייבת שמנהל המערכת יאשר את אפליקציית FastComments ברמת הארגון קודם לכן.

**2. הוספת FastComments כחומר קורס**

בתוך הקורס:

1. פתח את הקורס ועבור ל- **Materials**.
2. לחץ **Add Materials** > **Add File/Link/External Tool**.
3. בחר **External Tool**.
4. בחר **FastComments** מתוך רשימת הכלים הרשומים.
5. הגדר **Name** (זוהי התצוגה שהתלמידים רואים ברשימת החומרים) ותיאור אופציונלי (**Description**).
6. השאר את **Enable Grading** (העברת ציונים חזרה) כבוי. FastComments לא מדווח ציונים ל-Schoology, ולכן הפעלת העברת ציונים יוצרת טור ריק בספר הציונים.
7. לחץ **Submit**.

החומר מופיע כעת ברשימת החומרים של הקורס ונפתח את השרשור של FastComments בלחיצה.

**3. הטמעה פנימית דרך עורך הטקסט העשיר**

אם מנהל המערכת אפשר Deep Linking של FastComments במהלך הרישום, מנחים יכולים להטמיע את שרשור התגובות בתוך כל שדה Rich Text (הוראות משימות, גופי עמודים, פתיחי דיון):

1. פתח את עורך ה-Rich Text בעמוד היעד.
2. לחץ על סמל **External Tool** (חלק פאזל) בסרגל הכלים.
3. בחר **FastComments**.
4. הגדר את ההטמעה בדיאלוג ה-deep-linking ולחץ **Insert**.
5. שמור את העמוד.

אם כפתור External Tool לא מופיע בעורך ה-Rich Text, Deep Linking נעול עבור הכלי בשוכר הזה. ראו את הבעיות המפורטות למטה.

**4. נראות והקצאות לפי קטעים**

Schoology מטילה את זמינות הכלי לפי קטעים בקורס דרך Course Options:

1. מתוך הקורס, לחץ **Course Options** > **External Tools**.
2. עבור כל אפליקציית LTI מותקנת, אתה שולט האם היא זמינה לכל הקטעים בקורס או לקטעים ספציפיים.
3. כדי להגביל את FastComments לקטעים מסוימים, הסר את הסימון מהקטעים שלא צריכים לראות את הכלי.
4. גישה ברמת קטע גם קובעת אילו קטעים רואים את הרשומה **Add Materials** > **External Tool** עבור FastComments.

**5. מה תלמידים רואים**

תלמידים לוחצים על חומר ה-FastComments (או גוללים להטמעה הפנימית) ונכנסים לדיון השרשורי. SSO אוטומטית דרך השקת ה-LTI של Schoology תחת חשבון ה-Schoology שלהם.

מיפוי תפקידים:

- Schoology `Administrator` -> FastComments admin
- Schoology `Instructor` -> FastComments moderator
- Schoology `Student` -> FastComments commenter

**6. נקודות בעייתיות ב-Schoology**

- **רק ב-Enterprise.** חשבונות אישיים וחינמיים של Schoology אינם יכולים להתקין כלים ב-LTI 1.3. אם השוכר שלך ברמת החינם, אפשרות **External Tools** אינה קיימת ב-Course Options. שדרגו ל-Schoology Enterprise כדי להשתמש ב-FastComments.
- **Deep Linking כבוי כברירת מחדל עבור השוכר.** חלק מהשוכרים ב-Schoology מגבילים את מיקום Deep Linking ברמת הארגון. כשזה המצב, מנחים רואים רק את הזרימה **Add Materials** > **External Tool** ולא את כפתור ה-External Tool בעורך ה-Rich Text. כדי לאפשר הטמעה פנימית, מנהל המערכת ילך ל- **System Settings** > **Integration** > **LTI 1.3** > **FastComments** ויפעיל את המיקום **Content Item / Deep Linking**, ואז ישמור.
- **הגדרת הקצאה לפי קטע מבוטלת.** אם FastComments הוקצה ברמת הארגון אך המנחה לא רואה אותו ב- **Add Materials**, הקורס או הקטע אינם כלולים בהקצאת האפליקציה ברמת הארגון. בקש מהמנהל להוסיף את הקטע להקצאת אפליקציית FastComments.
- **שם החומר מול זהות השרשור.** שינוי שם החומר ב-Schoology לא מעביר את שרשור התגובות. השרשורים ממופים לפי מזהה קישור המשאב של ה-LTI, ולכן שינוי שם ישאיר את אותו השרשור; מחיקה ויצירה מחדש של החומר יוצרת שרשור חדש וריק.
המדריך הזה מכסה הוספת FastComments לקורס Moodle 4.x לאחר שמנהל האתר רשם את הכלי והגדיר אותו להצגה בבוחר הפעילות. אם FastComments עדיין לא רשום, עיין תחילה במדריך הרשמת Moodle.

#### פתח את הקורס במצב עריכה

1. היכנס ל-Moodle כמורה בעריכה (Editing Teacher) או ברמה גבוהה יותר עבור הקורס.
2. פתח את הקורס.
3. הפעל את מצב העריכה (**Edit mode**) באמצעות המתג בפינה הימנית-עליונה של כותרת הקורס.

Moodle 4.x החליף את תפריט ה"Add an activity or resource" הישן מ-3.x בדיאלוג בוחר פעילויות במסך מלא. ב-Moodle 4.5 נשאר אותו בורר אך נוסף שורת מועדפים/כוכביות בראש, כך שסימון של FastComments פעם אחת הופך אותו לזריז יותר להגיע אליו בחלקים הבאים.

#### הוסף את פעילות FastComments

1. גלול אל המקטע בקורס (נושא או שבוע) שבו השיחה שייכת.
2. לחץ על **Add an activity or resource** בתחתית אותו מקטע.
3. בדיאלוג הבורר, בחר **FastComments**. אם אינך רואה אותו, קפוץ לסעיף הבעיות הנפוצות למטה.

טופס הגדרות הפעילות נפתח. השדות החשובים:

- **Activity name** (נדרש). מוצג בדף הקורס ובספר הציונים. לדוגמה: `Week 3 Discussion`.
- **Activity description**. טקסט מבוא אופציונלי שמוצג מעל שרשור התגובות.
- **Show description on course page**. סמן אם ברצונך שהתיאור יהיה גלוי ללא צורך בלחיצה לתוך הפעילות.
- **Preconfigured tool**. מוגדר ל-`FastComments` (נבחר אוטומטית כשיופעל מהבורר). אל תשנה.
- **Launch container**. הגדר ל- **New window**. ראה את סעיף הבעיות הנפוצות להסבר מדוע "Same window" יכול לשבור במטלות Moodle מסוימות.
- **Tool URL**, **Public key**, **Shared secret**, **Custom parameters**. השאר ריקים. Dynamic Registration טיפל בזה ברמת האתר.

גלול לתחתית ולחץ **Save and return to course** (או **Save and display** כדי לפתוח את הפעילות מיד).

הפעילות מופיעה כשורה במקטע עם סמל FastComments. תלמידים ילחצו על השורה כדי לפתוח את שרשור התגובות.

#### הטמע את FastComments באופן מקומי בתוך העורך

לשרשור בתוך דף (Page), פרק בספר (Book chapter), שיעור (Lesson), או כל משאב אחר שמשתמש בעורך Atto או TinyMCE:

1. פתח את המשאב במצב עריכה.
2. מקם את הסמן במקום שבו השרשור אמור להופיע.
3. בסרגל הכלים של העורך, לחץ על הכפתור **LTI** / **External tool**. ב-Atto הוא מסומן "Insert LTI Advantage content". ב-TinyMCE (ברירת המחדל ב-Moodle 4.3+) הוא נמצא תחת תפריט **More** כ- **External tools**.
4. בחר **FastComments** מרשימת הכלים.
5. FastComments פותח בורר קישורים עמוקים (deep-linking). אשר את כותרת השרשור ולחץ **הטמע**.
6. העורך מכניס בלוק מציין מיקום של LTI. שמור את המשאב.

כל מופע מושתל הוא שרשור נפרד שממופה על פי מזהה פריט הקישור העמוק (deep-link content item ID), כך שדף עם שלושה הטמעות FastComments יקבל שלושה שרשורים עצמאיים.

#### הגבלת גישה והגדרות קבוצות

הגדרות הפעילות הסטנדרטיות של Moodle חלות על פעילויות FastComments:

- **Common module settings** > **Group mode**. קביעה זו ל- **Separate groups** או **Visible groups** לא מפרקת את FastComments לשרשורים נפרדים לכל קבוצה מעצמה. מצב הקבוצות של Moodle רק מסנן את ספר הציונים ורשימת החברים. כדי להפעיל שרשור נפרד לכל קבוצה, הוסף פעילות FastComments נפרדת לכל קבוצה והשתמש ב- **Restrict access** כדי להגביל כל אחת מהן.
- **Restrict access** > **Add restriction**. תומך בתנאי Moodle הסטנדרטיים: **Date**, **Grade**, **Group**, **Grouping**, **User profile**, וסטים מקוננים של הגבלות. השתמש ב- **Group** כדי לנעול פעילות FastComments לקבוצה יחידה.
- **Activity completion**. קבע ל- **Students must view this activity to complete it** אם ברצונך במעקב השלמה. FastComments כרגע לא מדווח מחדש אירוע השלמה ל-Moodle מעבר להשקה.

#### מיפוי תפקידים

FastComments קורא את הערעור LTI `roles` ש-Moodle שולח בכל השקה וממפה אותו כדלהלן:

- Moodle **Manager** or **Site administrator** -> FastComments **admin**
- Moodle **Editing teacher** or **Non-editing teacher** -> FastComments **moderator**
- Moodle **Student** -> FastComments **commenter**
- Moodle **Guest** -> קריאה בלבד

מנהלים יכולים למחוק כל תגובה, לחסום משתמשים ולערוך הגדרות שרשור. Moderators יכולים למחוק ולמאשר תגובות בתוך השרשור שהם השיקו לתוכו. תפקידי Moodle מותאמים אישית יורשים את המיפוי של ארכיטיפ התפקיד ממנו הם נשוכפצו.

#### מה הסטודנטים רואים

תלמידים לוחצים על פעילות FastComments (או גוללים אל הבלוק המושתל בתוך דף או ספר). Moodle שולח את הזהות שלהם ל-FastComments דרך השקת LTI:

- אין מסך התחברות. FastComments מחבר אותם באמצעות חשבון ה-Moodle.
- שם התצוגה שלהם, הדואר האלקטרוני והאבאטאר מגיעים מ-Moodle.
- השרשור מוקצה ל-(Moodle site, course, resource link ID), כך שהעתקת אותה פעילות לקורס אחר יוצרת שרשור חדש.
- תגובות מקוננות, הצבעות והתראות עובדים כפי שבשרשור עצמאי של FastComments.

#### נעל את הגישה הציבורית (מומלץ)

ברירת המחדל היא שנתוני התגובות של FastComments ניתנים לקריאה ציבורית. כל מי שיכול לנחש את כתובת ה-URL של שרשור או את ה-API endpoint שלו יכול לצפות בתגובות, אפילו מחוץ ל-Moodle. עבור שיחות קורס סביר להניח שתרצה להגביל צפייה רק לסטודנטים הרשומים.

פתח את <a href="https://fastcomments.com/auth/my-account/customize-widget" target="_blank">דף התאמת הווידג'ט</a> וצור כלל עם **Require SSO To View Comments** מופעל, ואז קבע את רמת האבטחה ל- **Secure SSO** כך שניתן יהיה לטעון שרשורים רק דרך השקת LTI חתומה.

ראה [Protecting Comment Threads With Single-Sign-On](/guide-customizations-and-configuration.html#sso-require-to-view-comments) למדריך מלא, כולל איך להגביל את הכלל לדומיין או לדף יחיד.

#### בעיות נפוצות ב-Moodle

**FastComments חסר מבוחר הפעילויות.** מנהל האתר רשם את הכלי אך לא קבע את **Tool configuration usage** ל- **Show in activity chooser and as a preconfigured tool**. תיקון זה תחת **Site administration** > **Plugins** > **Activity modules** > **External tool** > **Manage tools** > סמל הגלגל על האריח של FastComments.

**ההשקה נכשלת או מציגה מסגרת ריקה כשהמיכל מוגדר ל-"Same window".** עוגיות הסשן של Moodle משתמשות ב-`SameSite=Lax` כברירת מחדל, וחלק מהדפדפנים מסירים אותן ב-POST החוצה לאתר ש-LTI 1.3 משתמש בה כדי לחזור מ-FastComments. הגדר את **Launch container** ל- **New window** בפעילות. זהו דרישה קשה עבור הטמעות FastComments בתוך דף או ספר, מכיוון שדרך ההשקה המושתלת של העורך תמיד פותחת חלון חדש.

**טענת ה-`iss` היא ה-URL של אתר ה-Moodle, לא מזהה שוכר (tenant ID).** FastComments משתמש בכתובת האתר של Moodle (ערך התצורה `wwwroot`) כמנפיק (issuer) של LTI. אם ה-instance של Moodle שלך עובר לדומיין חדש או תשנה את `wwwroot`, השרשורים הקיימים של FastComments יישארו קשורים למנפיק הישן ולא יתאימו להשקות החדשות. רשם מחדש את הכלי מול ה-URL החדש והעבר שרשורים דרך ניהול FastComments במידת הצורך.

**גיבוי ושחזור של פעילות.** גיבוי של קורס ושחזור שלו לקורס חדש יוצר מזהי resource link חדשים, כך שהפעילויות המשוחזרות של FastComments יתחילו עם שרשורים ריקים. הקורס המקורי שומר על השרשורים המקוריים. זה התנהגות מכוונת, לא באג.

**Moodle 4.5 TinyMCE כברירת מחדל.** Moodle 4.5 מגיע עם TinyMCE כברירת מחדל בהתקנות חדשות. מיקום כפתור ה-External tool נמצא תחת תפריט **More** (`...`) במקום בסרגל הכלים הראשי. אתרים ישנים ששודרגו מ-4.1 שומרים על Atto אלא אם מנהל שינה את ברירת המחדל.
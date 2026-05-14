מדריך זה מכסה הוספת FastComments לקורס Moodle 4.x לאחר שמנהל האתר רשם את הכלי והגדיר אותו להופיע בבוחן הפעילויות. אם FastComments עוד לא נרשם, עיין קודם במדריך הרישום ל-Moodle.

#### פתח את הקורס במצב עריכה

1. היכנס ל-Moodle כמורה עם הרשאת עריכה (או גבוה יותר) עבור הקורס.
2. פתח את הקורס.
3. הפעל את **מצב עריכה** בעזרת המתג בפינה הימנית-על של כותרת הקורס.

Moodle 4.x החליפה את תפריט הנפתח הישן "Add an activity or resource" שהיה ב-3.x בדיאלוג בוחן פעילויות במסך מלא. ב-Moodle 4.5 נשאר אותו בוחן אך נוסף שורת מועדפים/כוכביות בראש, כך שכיור FastComments פעם אחת מקל על הגישה אליו בסעיפים הבאים.

#### הוספת פעילות FastComments

1. גלול אל החלק של הקורס (נושא או שבוע) שבו השיחה שייכת.
2. לחץ על **Add an activity or resource** בתחתית אותו חלק.
3. בדיאלוג הבוחן, בחר **FastComments**. אם אינך רואה אותו, דלג למדור הבעיות מטה.

טופס הגדרות הפעילות נפתח. השדות החשובים:

- **Activity name** (נדרש). מוצג בדף הקורס ובספר הציונים. דוגמה: `Week 3 Discussion`.
- **Activity description**. טקסט הקדמה אופציונלי המוצג מעל אשכול התגובות.
- **Show description on course page**. סמן אם ברצונך שהתיאור יהיה גלוי בלי צורך להיכנס לפעילות.
- **Preconfigured tool**. מוגדר ל-`FastComments` (נבחר אוטומטית כאשר נפתח מהבוחן). אל תשנה.
- **Launch container**. הגדר ל-**New window**. ראה את מדור הבעיות למידע מדוע "Same window" עלול להישבר בפריסות Moodle מסוימות.
- **Tool URL**, **Public key**, **Shared secret**, **Custom parameters**. השאר ריקים. Dynamic Registration טיפל בכך ברמת האתר.

גלול לתחתית ולחץ על **Save and return to course** (או **Save and display** לפתיחת הפעילות מיידית).

הפעילות מופיעה כשורה בחלק עם סמל FastComments. סטודנטים ילחצו על השורה כדי לפתוח את אשכול התגובות.

#### הטמעת FastComments בתוך העורך

לצורך אשכול בתוך Page, פרק של Book, Lesson, או כל משאב אחר שמשתמש בעורך Atto או TinyMCE:

1. פתח את המשאב במצב עריכה.
2. מקם את הסמן במקום שבו האשכול אמור להופיע.
3. בסרגל הכלים של העורך, לחץ על כפתור ה-**LTI** / **External tool**. ב-Atto הוא מתויג "Insert LTI Advantage content". ב-TinyMCE (ברירת מחדל ב-Moodle 4.3+) הוא נמצא תחת תפריט **More** כ-**External tools**.
4. בחר **FastComments** מרשימת הכלים.
5. FastComments פותח בורר deep-linking. אשר את כותרת האשכול ולחץ **Embed**.
6. העורך יכניס בלוק מייצג LTI. שמור את המשאב.

כל מופע מוטמע הוא אשכול נפרד שמקודד לפי מזהה פריט התוכן של ה-deep-link, כך שעמוד עם שלוש הטמעות של FastComments יקבל שלושה אשכולות עצמאיים.

#### הגבלת גישה והגדרות קבוצות

הגדרות הפעילות הסטנדרטיות של Moodle חלות על פעילויות FastComments:

- **Common module settings** > **Group mode**. הגדרה זו ל-**Separate groups** או **Visible groups** לא מפצלת את FastComments לאשכולות לכול קבוצה מתוך הקופסה. מצב הקבוצה של Moodle מסנן רק את ספר הציונים ורשימת החברים. כדי להפעיל אשכול נפרד לכל קבוצה, הוסף פעילות FastComments נפרדת לכל קבוצה והשתמש ב-**Restrict access** לתחום כל אחת.
- **Restrict access** > **Add restriction**. תומך בתנאים הסטנדרטיים של Moodle: **Date**, **Grade**, **Group**, **Grouping**, **User profile**, וסטים מקוננים של הגבלות. השתמש ב-**Group** כדי לנעול פעילות FastComments לקבוצה אחת.
- **Activity completion**. הגדר ל-**Students must view this activity to complete it** אם ברצונך במעקב השלמה. FastComments נכון לעכשיו לא מדווח חזרה ל-Moodle על אירוע השלמה מעבר להשקה.

#### מיפוי תפקידים

FastComments קורא את הטענה `roles` של LTI ש-Moodle שולח בכל השקה וממפה אותה כך:

- Moodle **Manager** או **Site administrator** -> FastComments **admin**
- Moodle **Editing teacher** או **Non-editing teacher** -> FastComments **moderator**
- Moodle **Student** -> FastComments **commenter**
- Moodle **Guest** -> לקריאה בלבד

מנהלים יכולים למחוק כל תגובה, לחסום משתמשים, ולערוך הגדרות אשכול. מתווכים יכולים למחוק ולאשר תגובות בתוך האשכול שהם השיקו אליו. תפקידים מותאמים אישית ב-Moodle יורשים את המיפוי של הארכיטיפ שממנו הם שוכפלו.

#### מה הסטודנטים רואים

סטודנטים לוחצים על פעילות FastComments (או גוללים לבלוק המוטמע בתוך Page או Book). Moodle שולח את זהותם ל-FastComments דרך השקת LTI:

- אין מסך התחברות. FastComments מחבר אותם באמצעות חשבון ה-Moodle שלהם.
- שם התצוגה שלהם, אימייל ואווטאר מגיעים מ-Moodle.
- האשכול מקובע ל-(Moodle site, course, resource link ID), כך שהעתקה של אותה פעילות לקורס אחר מקבלת אשכול חדש.
- תגובות מקוננות, הצבעות והתראות עובדות כמו באשכול FastComments עצמאי.

#### תקלות נפוצות ב-Moodle

**FastComments חסר בבוחן הפעילויות.** מנהל האתר רשם את הכלי אך לא קבע את **Tool configuration usage** ל-**Show in activity chooser and as a preconfigured tool**. תקן זאת תחת **Site administration** > **Plugins** > **Activity modules** > **External tool** > **Manage tools** > סמל גלגל השיניים באריח של FastComments.

**ההשקה נכשלה או מציגה מסגרת ריקה כאשר מוגדר ל-"Same window".** קובצי העוגיות של ה-session ב-Moodle משתמשים ב-`SameSite=Lax` כברירת מחדל, וחלק מהדפדפנים מסירים אותם בפוסט חוצה-אתרים שה-LTI 1.3 משתמש בו כדי לחזור מ-FastComments. הגדר את **Launch container** ל-**New window** בפעילות. זה דרישה קשוחה להטמעות FastComments בתוך Page או Book, שכן מסלול ההשקה המוטמע בעורך תמיד פותח חלון חדש.

**הטענה `iss` היא כתובת האתר של Moodle, לא מזהה שוכר.** FastComments משתמש בכתובת האתר של Moodle (ערך הקונפיגורציה `wwwroot`) כמנפיק LTI. אם המופע של Moodle שלך עובר לדומיין חדש או תשנה את `wwwroot`, אשכולות FastComments קיימים יישארו קשורים למנפיק הישן ולא יתאימו להשקות חדשות. רשום מחדש את הכלי נגד ה-URL החדש והעבר אשכולות דרך מנהל FastComments אם נדרש.

**גיבוי ושחזור של פעילות.** גיבוי קורס ושחזורו לקורס חדש יוצר מזהי resource link חדשים, כך שפעילויות FastComments משוחזרות מתחילות באשכולות ריקים. הקורס המקורי שומר על האשכולות המקוריים. זהו התנהגות מכוונת, לא באג.

**Moodle 4.5 TinyMCE כברירת מחדל.** Moodle 4.5 מגיע עם TinyMCE כברירת מחדל להתקנות חדשות. מיקום הכפתור External tool נמצא תחת תפריט **More** (`...`) ולא בסרגל הראשי. אתרים ישנים שעודכנו מ-4.1 שומרים על Atto אלא אם מנהל שינה את ברירת המחדל.

---
דף זה מכסה את הוספת FastComments לקורס ב-Brightspace לאחר שמנהל המערכת רשם את הכלי ויצר פריסה (deployment). אם הכלי עדיין לא נרשם, עיין תחילה במדריך הרישום ל-D2L.

Brightspace מספק שתי חוויות יצירת תוכן: **Classic Content** ו-**New Content Experience** (נקראת גם **Lessons**). שתיהן מציגות את FastComments, אך מסלולי התפריטים שונים. בכל מקטע למטה מכוסים שני המסלולים כאשר הם נבדלים.

#### איתור כלי FastComments

כלי FastComments מופיע בשני מקומות בעורך התוכן של הקורס:

1. ב-picker של פעילויות, שמושג מתוך כפתור **Add Existing** של מודול/יחידה (מסומן כ-**Add Existing Activities** בגרסאות ישנות של Brightspace). FastComments מופיע ישירות ב-picker בגרסאות העדכניות של Brightspace; בגרסאות ישנות הוא מקונן תחת תפריט משנה **External Learning Tools**. כל נתיב מוסיף את FastComments כנושא עצמאי.
2. בתיבת הדו-שיח **Insert Stuff** בתוך עורך ה-HTML, תחת **LTI Advantage**. זה משביץ את FastComments באופן מקומי בתוך נושא HTML דרך זרימת deep linking של LTI.

אם FastComments אינו מופיע באף אחד מה-pickers, הפריסה אינה מופעלת עבור יחידת הארגון (org unit) שמכילה את הקורס. בקש ממנהל Brightspace לפתוח **Admin Tools** > **Manage Extensibility** > **LTI Advantage** > כלי FastComments > **View Deployments**, לפתוח את הפריסה, ולהוסיף את יחידת הארגון של הקורס (או יחידת על) תחת **Org Units**.

#### הוספת FastComments כנושא במודול

Classic Content:

1. פתח את הקורס ולחץ על **Content** בסרגל הניווט.
2. בחר את המודול שבו אמור להימצא הדיון (או צור אחד דרך **Add a module**).
3. לחץ על **Add Existing** (Brightspace ישן: **Add Existing Activities** > **External Learning Tools**).
4. ב-picker, לחץ על **FastComments**. Brightspace יוצר נושא במודול ומחזיר אותך לתצוגת התוכן.
5. לחץ על הנושא החדש. שנה לו את השם למשהו תיאורי כמו `FastComments Discussion` באמצעות עורך הכותרת במקום.

New Content Experience (Lessons):

1. פתח את הקורס ולחץ על **Content**.
2. פתח את היחידה והלקח (lesson) שבו אמור להימצא הדיון.
3. לחץ **Add** > **Existing Activity** ובחר **FastComments** (Brightspace ישן: מקונן תחת **External Learning Tools**).
4. הפעילות תתווסף ללקח.
5. לחץ על כותרת הפעילות כדי לשנות את שמה.

בפעם הראשונה שכל משתמש (מדריך או סטודנט) פותח את הנושא, FastComments מאתחל את השרשור עבור קישור המשאב הזה. השרשור קשור ל-resource link ID, כך ששינוי שם או העברת הנושא לא ישנה את השרשור שנבחר.

#### הטמעת FastComments בקו בתוך נושא HTML

השתמש בזרימה זו כאשר ברצונך שהתגובות יופיעו מתחת לקריאה, וידאו, או תוכן אחר בתוך אותו דף נושא במקום כנושא נפרד.

1. פתח או צור נושא HTML במודול/לקח.
2. לחץ **Edit HTML** כדי לפתוח את עורך ה-HTML של Brightspace.
3. מקם את הסמן במקום שבו השרשור של התגובות אמור להופיע.
4. לחץ על כפתור **Insert Stuff** (אייקון פאזל בסרגל העורך).
5. בתיבת Insert Stuff, גלול ל-**LTI Advantage** ולחץ על **FastComments**.
6. FastComments פותח picker של deep linking. אשר את המיקום (האפשרויות המוגדרות כברירת מחדל מתאימות לדיוני תוכן); לחץ **Insert** או **Continue**.
7. Brightspace חוזרת לעורך ה-HTML עם בלוק מציין מקומו שמייצג את ה-LTI launch. לחץ **Save and Close** על הנושא.

כאשר הנושא נטען, Brightspace מחליפה את המציין ב-iframe שמבצע auto-launch של FastComments דרך LTI. הסטודנטים רואים את שרשור הדיון במקום.

נושא HTML יחיד יכול להכיל מספר הטמעות FastComments באמצעות deep-link. כל הטמעה מקבלת שרשור משלה כי כל deep link מייצר resource link ID שונה.

#### נושא מודול מול קישור מהיר מקומי (Inline Quicklink)

בחר בגישת ה-נושא במודול כאשר:

- הדיון הוא הפעילות המרכזית בשלב זה במודול.
- אתה רוצה שהנושא יופיע בתוכן העניינים של Brightspace, במעקב השלמת משימות, וב-Class Progress.

בחר בגישת ההטמעה המקובצת (inline embed) כאשר:

- התגובות צריכות לשבת מתחת לתוכן אחר באותו דף.
- אינך רוצה פריט נפרד שניתן לעקוב אחר השלמתו בתוכן העניינים.

#### נראות, מצב טיוטה ותנאי שחרור

נושא FastComments חדש גלוי לסטודנטים כברירת מחדל. כדי להסתירו בזמן שאתה מכין אותו:

1. בעורך התוכן, לחץ על כותרת הנושא (Classic) או על תפריט שלוש הנקודות על הפעילות (New Content Experience).
2. קבע את הסטטוס ל-**Draft** (Classic) או כבה את **Visibility** (New Content Experience).

נושאי טיוטה אינם נראים לסטודנטים. מדריכים ועוזרי הוראה עדיין רואים אותם עם תג "Draft".

כדי להגביל את הנושא לקבוצה או מקטע ספציפי:

1. פתח את הנושא.
2. לחץ בתפריט כותרת הנושא > **Edit Properties In-place** (Classic) או **Edit** > **Restrictions** (New Content Experience).
3. תחת **Release Conditions**, לחץ **Create**.
4. בחר **Group enrollment** או **Section enrollment**, בחר את הקבוצה/הקטע, ושמור.

תנאי שחרור מצטברים עם מיפוי התפקידים של FastComments. תלמידים שאינם יכולים לראות את הנושא לא מקבלים LTI launch.

#### מה הסטודנטים רואים בהשקה הראשונה

כאשר סטודנט לוחץ על הנושא (או טוען נושא HTML עם הטמעה):

1. Brightspace מבצעת את השקת ה-LTI 1.3 ברקע.
2. FastComments מקבל את שם הסטודנט, כתובת האימייל שלו, URL לאווטאר, ותפקיד ה-LMS שלו, ומתחבר אותו אוטומטית. אין תיבת כניסה של FastComments.
3. שרשור התגובות עבור קישור המשאב נטען בתוך ה-iframe של Brightspace.

מיפוי תפקידים בעת ההשקה:

- Brightspace `Administrator` הופך ל-FastComments **admin** עבור השרשור (גישה מלאה למוודרציה, מחיקה, חסימה וקונפיגורציה).
- Brightspace `Instructor` הופך ל-FastComments **moderator** (הצמדה, הסתרה, מחיקה, חסימה).
- כל התפקידים האחרים (`Learner`, `TeachingAssistant`, וכו') הופכים למגיבים רגילים.

תגובות משויכות לחשבון Brightspace של הסטודנט. אם הסטודנט ערך את שמו או את האווטאר ב-Brightspace, ההשקה הבאה באמצעות LTI תסנכרן את השינוי.

#### גובה ה-Iframe והתאמה (Resize)

FastComments משדר את ההודעה postMessage `org.imsglobal.lti.frameResize` על כל רינדור של השרשור ובשינויים בתוכן (תגובה חדשה, הרחבת תגובות). Brightspace מאזינה להודעה זו ומכווינה את גובה ה-iframe כך שהשרשור לא ייחתך ולא יופיע פס גלילה פנימי.

אם ה-iframe נשאר בגובה קצר קבוע:

- אשר שהקורס נטען דרך HTTPS. מאזין ה-postMessage של Brightspace דוחה מסגרות של תוכן מעורב (mixed-content).
- אשר שלא קיימת הרחבת דפדפן שחוסמת את ערוץ ה-postMessage.
- עבור ההטמעות מקו בתוך נושא HTML, ה-HTML הסובב לא יכול לעטוף את ה-iframe בתוך מיכל בעל גובה קבוע. הסר כל `style="height: ..."`_inline מהאלמנט ההורה.

#### בעיות ספציפיות ל-Brightspace

**הכלי לא מוצג ב-picker של Add Existing.** הפריסה אינה מופעלת עבור יחידת הארגון של הקורס. מנהל צריך להוסיף את יחידת הארגון (או יחידת על) לרשימת ה-Org Units של הפריסה. רישום הכלי לבדו אינו מספיק; הפריסה קובעת אילו קורסים רואים את הכלי.

**התנגשות ב-`deployment_id` בהשקה.** FastComments מקבע (TOFU) את ה-`deployment_id` הראשון שהוא רואה עבור רישום. אם מנהל מוחק את הפריסה המקורית ויוצר פריסה חדשה, השקות מהפריסה החדשה נדחות עם שגיאת אי-התאמה של הפריסה. התיקון הוא לרשום מחדש את FastComments (ליצור URL רישום חדש ולהריץ Dynamic Registration שוב); רשומת הקונפיגורציה הישנה מוחלפת.

**הכלי משיק אך מציג "Invalid LTI launch".** הקורס נמצא במבנה tenant/org שונה ממה שהפריסה מכסה, או שהפריסה הושבתה לאחר הרישום. בדוק שוב **Admin Tools** > **Manage Extensibility** > **LTI Advantage** > FastComments > כפתור **Enabled** ורשימת יחידות הארגון של הפריסה.

**שמות ותפקידים חסרים בתוך FastComments.** Brightspace שולח השקות LTI עם טענות Names and Role Provisioning Services (NRPS). אם קורס שודרג מקישור LTI 1.1 ישן, ההשקה עשויה להחסיר את הטענות `name` ו-`email`. הוסף שוב את נושא FastComments דרך **Add Existing** (אל תעביר את הקישור הישן) כדי שההשקה תשתמש ב-LTI 1.3.

**הטמעה מציגה מסך התחברות במקום SSO אוטומטי.** נושא ה-HTML הוכנס כ-iframe פשוט שמצביע ל-FastComments במקום דרך **Insert Stuff** > **LTI Advantage**. iframes פשוטים מדלגים על השקת ה-LTI ומוציאים את המשתמשים לעמוד הציבורי של FastComments. מחק את ה-iframe והכנס מחדש דרך זרימת Insert Stuff.
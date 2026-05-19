עמוד זה מסביר כיצד להוסיף את FastComments לקורס ב-Brightspace לאחר שמנהל המערכת רשם את הכלי ויצר פריסה. אם הכלי עדיין לא נרשם, עיין תחילה במדריך הרישום של D2L.

<div class="screenshot white-bg">
    <div class="title">FastComments משובץ כנושא יחידה ב-Brightspace</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-d2l-comments-in-unit.png" alt="FastComments פועל בתוך יחידת Brightspace, מציג תגובות שזורות ובוחר אזכור @-mention" />
</div>

Brightspace מספקת שתי חוויות עריכת תוכן: **Classic Content** ו-**New Content Experience** (המכונה גם **Lessons**). שתיהן מאפשרות שימוש ב-FastComments, אבל מסלולי התפריטים שונים. כל קטע מטה מכסה את שתי הגרסאות כאשר יש הבדל.

#### אתר את הכלי FastComments

הכלי FastComments מופיע בשני מקומות בתוך עורך התוכן של הקורס:

1. ב-picker של פעילויות, הנגיש מתוך כפתור **Add Existing** של מודול/יחידה (בלגרסאות ישנות של Brightspace מסומן **Add Existing Activities**). בגרסאות הנוכחיות של Brightspace FastComments מופיע ישירות ב-picker; בגרסאות ישנות הוא מוחבא תחת תת-תפריט **External Learning Tools**. כל דרך מוסיפה את FastComments כנושא עצמאי.
2. בדיאלוג **Insert Stuff** בתוך עורך ה-HTML, תחת **LTI Advantage**. זה משבץ את FastComments באופן מוטמע בתוך נושא HTML באמצעות זרימת ה-LTI deep linking.

אם FastComments אינו מופיע באף אחד מה-picker-ים, הפריסה (deployment) אינה מאופשרת עבור יחידת הארגון (org unit) שמכילה את הקורס. בקש ממנהל Brightspace לפתוח **Admin Tools** > **Manage Extensibility** > **LTI Advantage** > כלי FastComments > **View Deployments**, לפתוח את הפריסה, ולהוסיף את יחידת הארגון של הקורס (או יחידת ארגון אב) תחת **Org Units**.

#### הוספת FastComments כנושא במודול

Classic Content:

1. פתח את הקורס ולחץ **Content** בסרגל הניווט.
2. בחר את המודול שבו אמור להיות הדיון (או צור אחד דרך **Add a module**).
3. לחץ **Add Existing** (Brightspace ישן: **Add Existing Activities** > **External Learning Tools**).
4. ב-picker, לחץ **FastComments**. Brightspace יוצר נושא במודול ומחזיר אותך לצפיית התוכן.
5. לחץ על הנושא החדש. שנה את שמו למשהו תיאורי כמו `FastComments Discussion` באמצעות עורך הכותרת המוטמע.

New Content Experience (Lessons):

1. פתח את הקורס ולחץ **Content**.
2. פתח את היחידה והלקח (lesson) שבו אמור להיות הדיון.
3. לחץ **Add** > **Existing Activity** ובחר **FastComments** (Brightspace ישן: מקונן תחת **External Learning Tools**).
4. הפעילות מתווספת ללקח.
5. לחץ על כותרת הפעילות כדי לשנות את שמה.

בפעם הראשונה שכל משתמש (מרצה או סטודנט) פותח את הנושא, FastComments מאתחל את השרשור (thread) עבור קישור המשאב (resource link) הזה. השרשור קשור ל-resource link ID, כך ששינוי שם או העברה של הנושא לא ישנו את השרשור שנפתח.

#### שיבוץ FastComments בתוך נושא HTML

השתמש בזרימה זו כאשר אתה רוצה שהתגובות יופיעו מתחת לקריאה, סרטון או תוכן אחר בתוך אותה עמודת נושא במקום כנושא נפרד.

1. פתח או צור נושא HTML במודול/לקח.
2. לחץ **Edit HTML** כדי לפתוח את עורך ה-HTML של Brightspace.
3. הנח את הסמן במקום שבו צריך להופיע שרשור התגובות.
4. לחץ על כפתור **Insert Stuff** (אייקון פאזל בסרגל העורך).
5. בדיאלוג Insert Stuff, גלול ל-**LTI Advantage** ולחץ **FastComments**.
6. FastComments פותח picker ל-deep linking. אשר את המיקום (האפשרויות המוגדרות כברירת מחדל מתאימות לדיוני תוכן); לחץ **Insert** או **Continue**.
7. Brightspace יחזור לעורך ה-HTML עם בלוק מציין מקום המייצג את הפעלת ה-LTI. לחץ **Save and Close** על הנושא.

כאשר הנושא נטען, Brightspace מחליף את ה-placholder ב-iframe שמפעיל את FastComments אוטומטית דרך ה-LTI. הסטודנטים יראו את שרשור הדיון משובץ בתוך ה-iframe של Brightspace.

נושא HTML יחיד יכול להכיל מספר משובצים של FastComments באמצעות deep linking. כל שיבוץ מקבל שרשור נפרד מכיוון שכל deep link מייצר resource link ID שונה.

#### נושא מודול לעומת קישור מהיר משובץ בתוך הדף

בחר בגישת ה-נושא במודול כאשר:

- הדיון הוא הפעילות הראשית עבור הצעד הזה במודול.
- אתה רוצה שהנושא יופיע בתוכן העניינים של Brightspace, במעקב השלמת פעילויות וב-Class Progress.

בחר בגישת ה-שיבוץ המוטמע כאשר:

- יש למקם את התגובות מתחת לתוכן אחר באותו עמוד.
- אינך רוצה פריט נפרד שניתן לעקוב אחר השלמתו בטבלת התוכן.

#### נראות, מצב טיוטה ותנאי שחרור

נושא FastComments חדש גלוי לסטודנטים כברירת מחדל. כדי להסתיר אותו בזמן ההגדרה:

1. בעורך התוכן, לחץ על כותרת הנושא (Classic) או על תפריט שלוש הנקודות של הפעילות (New Content Experience).
2. קבע סטטוס ל-**Draft** (Classic) או נטר את **Visibility** לכיבוי (New Content Experience).

נושאי טיוטה אינם ניתנים לצפייה על ידי סטודנטים. מרצים ועוזרי הוראה עדיין רואים אותם עם תג "Draft".

כדי להגביל את הנושא לקבוצה או קטע מסוים:

1. פתח את הנושא.
2. לחץ על תפריט כותרת הנושא > **Edit Properties In-place** (Classic) או **Edit** > **Restrictions** (New Content Experience).
3. תחת **Release Conditions**, לחץ **Create**.
4. בחר **Group enrollment** או **Section enrollment**, בחר את הקבוצה/הסעיף ושמור.

תנאי השחרור מצטברים עם מיפוי התפקידים של FastComments. סטודנטים שאינם יכולים לראות את הנושא לא יקבלו הפעלת LTI.

#### מה הסטודנטים רואים בהשקה הראשונה

כשהסטודנט לוחץ על הנושא (או טוען נושא HTML עם שיבוץ):

1. Brightspace מבצע את הפעלת ה-LTI 1.3 ברקע.
2. FastComments מקבל את שם הסטודנט, אימייל, כתובת URL של אווטאר ותפקיד ה-LMS, ומחבר אותם באופן אוטומטי. אין מסך כניסה ל-FastComments.
3. שרשור התגובות עבור קישור המשאב נטען בתוך ה-iframe של Brightspace.

מיפוי תפקידים בזמן ההשקה:

- Brightspace `Administrator` הופך ל-admin של FastComments עבור השרשור (גישה מלאה למודרציה, מחיקה, חסימה וקונפיגורציה).
- Brightspace `Instructor` הופך למודרטור (moderator) של FastComments (הצמדה, הסתרה, מחיקה, חסימה).
- כל תפקיד אחר (`Learner`, `TeachingAssistant`, וכו') הופך למגיב רגיל.

תגובות מופיעות בשמם של חשבונות Brightspace של הסטודנטים. אם הסטודנט משנה את שמו או אווטאר ב-Brightspace, ההשקה הבאה של ה-LTI מסנכרנת את השינוי.

#### נעל גישה ציבורית (מומלץ)

ברירת המחדל היא שנתוני התגובות של FastComments ניתנים לקריאה לציבור. כל מי שיכול לנחש את כתובת השרשור או נקודת הקצה של ה-API יכול לצפות בתגובות, גם מחוץ ל-Brightspace. לשיחי קורסים רוב הסיכויים שתרצה להגביל צפייה רק ללומדים הרשומים.

פתח את דף <a href="https://fastcomments.com/auth/my-account/customize-widget" target="_blank">התאמת הווידג'ט</a> ויצר כלל עם האפשרות **Require SSO To View Comments** מופעלת, ואז הגדר את רמת האבטחה ל-**Secure SSO** כך ששרשורים יוכלו להיטען רק דרך השקת LTI חתומה.

ראה [Protecting Comment Threads With Single-Sign-On](/guide-customizations-and-configuration.html#sso-require-to-view-comments) להדרכה המלאה, כולל כיצד להגביל את הכלל לדומיין או דף יחיד.

#### גובה ה-iframe ושינוי גודל

FastComments שולח את הודעת ה-postMessage בשם `org.imsglobal.lti.frameResize` בכל טעינת שרשור ובשינויים בתוכן (תגובה חדשה, הרחבת תגובות). Brightspace מאזין להודעה זו ומתאים את גובה ה-iframe כך שהשרשור לא ייחתך ולא תופיע גלילה פנימית.

אם ה-iframe נשאר בגובה קצר קבוע:

- אשר שהקורס נטען דרך HTTPS. מאזין ה-postMessage של Brightspace דוחה frames בתוכן מעורב (mixed-content).
- אשר שלא קיימת הרחבת דפדפן שחוסמת את ערוץ ה-postMessage.
- עבור שיבוצים בתוך נושא HTML, ה-HTML המקיף לא יעטוף את ה-iframe בתוך מיכל עם גובה קבוע. הסר כל inline `style="height: ..."` מהרכיב ההורה.

#### בעיות ספציפיות ל-Brightspace

**הכלי לא מופיע ב-picker של Add Existing.** הפריסה אינה מאופשרת עבור יחידת הארגון של הקורס. מנהל צריך להוסיף את יחידת הארגון (או יחידת אב) לרשימת **Org Units** של הפריסה. רישום הכלי כשלעצמו לא מספיק; הפריסה קובעת אילו קורסים רואים את הכלי.

**שגיאת `deployment_id` mismatch בעת ההשקה.** FastComments מקבע (TOFU) את ה-`deployment_id` הראשון שהוא רואה עבור ההרשמה. אם מנהל מוחק את הפריסה המקורית ויוצר פריסה חדשה, השקות מהפריסה החדשה נדחות עם שגיאת אי-התאמת פריסה. התיקון הוא לרשום מחדש את FastComments (ליצור כתובת רישום חדשה (<a href="https://fastcomments.com/auth/my-account/lti-config" target="_blank">קבל אותה כאן</a>) ולהפעיל Dynamic Registration שוב); רשומת הקונפיגורציה הישנה מוחלפת.

**הכלי עולה אך מראה "Invalid LTI launch".** הקורס נמצא במבנה דייר/ארגון שונה ממה שהפריסה מכסה, או שהפריסה הושבתה אחרי הרישום. בדוק שוב **Admin Tools** > **Manage Extensibility** > **LTI Advantage** > FastComments > מצבי **Enabled** ורשימת יחידות הארגון של הפריסה.

**שמות ותפקידים חסרים בתוך FastComments.** Brightspace שולח השקות LTI עם טענות NRPS (Names and Role Provisioning Services). אם קורס שודרג מקישור LTI 1.1 ישן, בהשקה יחסר `name` ו-`email`. הוסף מחדש את נושא FastComments דרך **Add Existing** (אל תעביר/תמיר את הקישור הישן) כדי שההשקה תשתמש ב-LTI 1.3.

**השיבוץ מראה מסך כניסה במקום SSO אוטומטי.** נושא ה-HTML הוכנס כ-plain `<iframe>` שמפנה ל-FastComments במקום דרך **Insert Stuff** > **LTI Advantage**. iFrames פשוטים מדלגים על השקת ה-LTI ומובילים משתמשים לדף הציבורי של FastComments. מחק את ה-iframe והכנס מחדש דרך זרימת Insert Stuff.
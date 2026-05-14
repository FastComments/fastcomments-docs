דף זה מכסה את הוספת FastComments לקורס Brightspace לאחר שמנהל המערכת רשם את הכלי ויצר פריסה. אם הכלי עדיין לא נרשם, ראו תחילה את מדריך הרישום של D2L.

<div class="screenshot white-bg">
    <div class="title">FastComments embedded as a unit topic in Brightspace</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-d2l-comments-in-unit.png" alt="FastComments running inside a Brightspace unit, showing threaded comments and an @-mention picker" />
</div>

Brightspace מספקת שתי חוויות יצירת תוכן: **Classic Content** ו-**New Content Experience** (נקראת גם **Lessons**). שתיהן מציעות את FastComments, אך נתיבי התפריטים שונים. כל קטע למטה מכסה את שתי האפשרויות כאשר הן שונות.

#### Locate the FastComments Tool

כלי FastComments מופיע בשני מקומות בתוך עורך התוכן של הקורס:

1. ב-picker של הפעילויות, הנגיש מתוך כפתור **Add Existing** של מודול/יחידה (מוסמן כ-**Add Existing Activities** בגרסאות Brightspace ישנות יותר). בגרסאות הנוכחיות של Brightspace FastComments מופיע ישירות ב-picker; בגרסאות ישנות יותר הוא מקונן תחת תפריט משנה **External Learning Tools**. כל נתיב מוסיף את FastComments כנושא עצמאי.
2. בדיאלוג **Insert Stuff** בתוך עורך ה-HTML, תחת **LTI Advantage**. זה משלב את FastComments בתוך נושא HTML באמצעות זרימת deep linking של LTI.

אם FastComments לא מופיע באף picker, הפריסה (deployment) לא מאופשרת עבור יחידת הארגון (org unit) שמכילה את הקורס. בקשו ממנהל Brightspace לפתוח **Admin Tools** > **Manage Extensibility** > **LTI Advantage** > FastComments tool > **View Deployments**, לפתוח את הפריסה, ולהוסיף את יחידת הארגון של הקורס (או יחידת אב) תחת **Org Units**.

#### Add FastComments as a Topic in a Module

Classic Content:

1. פתחו את הקורס ולחצו **Content** בסרגל הניווט.
2. בחרו את המודול שיורכב ממנו הדיון (או צרו אחד דרך **Add a module**).
3. לחצו **Add Existing** (Brightspace ישן: **Add Existing Activities** > **External Learning Tools**).
4. ב-picker לחצו **FastComments**. Brightspace ייצור נושא במודול ויחזיר אתכם לתצוגת התוכן.
5. לחצו על הנושא החדש. שנו את שמו למשהו מתאר כמו `FastComments Discussion` באמצעות עורך הכותרת הפנימי.

New Content Experience (Lessons):

1. פתחו את הקורס ולחצו **Content**.
2. פתחו את היחידה והלקסון (lesson) שיורכב מהם הדיון.
3. לחצו **Add** > **Existing Activity** ובחרו **FastComments** (Brightspace ישן: מקונן תחת **External Learning Tools**).
4. הפעילות תתווסף ללקסון.
5. לחצו על כותרת הפעילות כדי לשנות את שמה.

בפעם הראשונה שמשתמש (מרצה או סטודנט) פותח את הנושא, FastComments מאתחל את השרשור עבור אותו resource link. השרשור קשור ל-resource link ID, כך ששינוי שם או העברת הנושא לא ישנו את השרשור שנטען.

#### Embed FastComments Inline in an HTML Topic

השתמשו בזרימה זו כאשר אתם רוצים שהתגובות יופיעו מתחת לקריאה, וידאו או תוכן אחר בתוך אותו דף נושא במקום כנושא נפרד.

1. פתחו או צרו נושא HTML במודול/לקסון.
2. לחצו **Edit HTML** כדי לפתוח את עורך ה-HTML של Brightspace.
3. מקמו את הסמן במקום שבו שרשור התגובות אמור להופיע.
4. לחצו על כפתור **Insert Stuff** (אייקון פאזל בסרגל הכלים של העורך).
5. בדיאלוג Insert Stuff גללו ל-**LTI Advantage** ולחצו **FastComments**.
6. FastComments יפתח picker של deep linking. אשרו את המיקום (ההגדרות המבוררות עובדות עבור דיוני תוכן); לחצו **Insert** או **Continue**.
7. Brightspace יחזור לעורך ה-HTML עם בלוק בעל מציין מקום המייצג את ה-launch של LTI. לחצו **Save and Close** על הנושא.

כשהנושא נטען, Brightspace מחליף את מציין המקום ב-iframe שמבצע auto-launch ל-FastComments דרך LTI. סטודנטים יראו את שרשור הדיון באופן משולב בתוך הדף.

נושא HTML אחד יכול להכיל מספר embed-ים של FastComments שנוצרו ב-deep link. לכל embed יש שרשור משלו כי לכל deep link נוצר resource link ID נפרד.

#### Module Topic vs Inline Quicklink

בחרו בגישת ה-**module topic** כאשר:

- הדיון הוא הפעילות העיקרית של הצעד במודול.
- אתם רוצים שהנושא יופיע בטבלת התוכן של Brightspace, במעקב השלמה (completion tracking), וב-Class Progress.

בחרו בגישת ה-**inline embed** כאשר:

- התגובות אמורות לשבת מתחת לתוכן אחר באותו דף.
- אינכם רוצים פריט נפרד שניתן לעקוב אחריו להשלמה בטבלת התוכן.

#### Visibility, Draft, and Release Conditions

נושא FastComments חדש גלוי לסטודנטים כברירת מחדל. כדי להסתירו בזמן ההגדרה:

1. בעורך התוכן לחצו על כותרת הנושא (Classic) או על תפריט שלוש הנקודות בפעילות (New Content Experience).
2. קבעו סטטוס ל-**Draft** (Classic) או כבה את **Visibility** (New Content Experience).

נושאי Draft אינם נראים לסטודנטים. מרצים ועוזרים (TAs) עדיין יראו אותם עם תגית "Draft".

כדי להגביל את הנושא לקבוצה או חלק מסוים (section):

1. פתחו את הנושא.
2. לחצו על תפריט כותרת הנושא > **Edit Properties In-place** (Classic) או **Edit** > **Restrictions** (New Content Experience).
3. תחת **Release Conditions**, לחצו **Create**.
4. בחרו **Group enrollment** או **Section enrollment**, בחרו את הקבוצה/החלק ושמרו.

תנאי שחרור מצטברים עם מיפוי התפקידים של FastComments עצמו. סטודנטים שאינם יכולים לראות את הנושא לא יקבלו LTI launch.

#### What Students See on First Launch

כאשר סטודנט לוחץ על הנושא (או טוען נושא HTML עם embed):

1. Brightspace מבצע את ה-LTI 1.3 launch ברקע.
2. FastComments מקבל את שם הסטודנט, אימייל, כתובת ה-avatar שלו ותפקיד ב-LMS, ומתחבר אותו אוטומטית. אין בקשת התחברות ל-FastComments.
3. שרשור התגובות עבור אותו resource link מוצג בתוך ה-iframe של Brightspace.

מיפוי תפקידים בעת ה-launch:

- Brightspace `Administrator` הופך ל-FastComments **מנהל** עבור השרשור (כל הזכויות למודרציה, מחיקה, חסימה וגישה להגדרות).
- Brightspace `Instructor` הופך ל-FastComments **מנחה** (הצמדה, הסתרה, מחיקה, חסימה).
- כל שאר התפקידים (`Learner`, `TeachingAssistant`, וכו') הופכים ל-מגיבים רגילים.

תגובות משויכות לחשבון Brightspace של הסטודנט. אם הסטודנט משנה את שמו או את ה-avatar ב-Brightspace, ה-launch הבא יסנכרן את השינוי.

#### Iframe Height and Resize

FastComments משדר את ההודעה postMessage `org.imsglobal.lti.frameResize` בכל רינדור של השרשור ובשינויים בתוכן (תגובה חדשה, פתיחת תשובות). Brightspace מאזין להודעה זו ומכוונן את גובה ה-iframe כך שהשרשור לא ייחתך ולא יופיע פס גלילה פנימי.

אם ה-iframe נשאר בגובה קבוע וקצר:

- ודאו שהקורס נטען דרך HTTPS. מאזין ה-postMessage של Brightspace דוחה iframes בתוכן מעורב (mixed-content).
- ודאו שאף תוסף דפדפן אינו חוסם את ערוץ ה-postMessage.
- עבור embeds inline בתוך נושא HTML, ה-HTML המקיף לא אמור לעטוף את ה-iframe במיכל בעל גובה קבוע. הסירו כל inline `style="height: ..."` מהאלמנט ההורי.

#### Brightspace-Specific Gotchas

**Tool not showing in the Add Existing picker.** הפריסה לא מאופשרת ל-unit של הקורס הזה. מנהל צריך להוסיף את יחידת הארגון (או יחידת אב) לרשימת **Org Units** של הפריסה. רישום הכלי לא מספיק; הפריסה קובעת אילו קורסים רואים את הכלי.

**`deployment_id` mismatch on launch.** FastComments TOFU-pins את ה-`deployment_id` הראשון שהוא רואה עבור רישום. אם מנהל מוחק את הפריסה המקורית ויוצר פריסה חדשה, השקות מהפריסה החדשה נענשות בשגיאת חוסר התאמה של הפריסה. התיקון הוא לרשום מחדש את FastComments (ליצור URL רישום חדש ולהריץ Dynamic Registration שוב); רשומת התצורה הישנה תוחלף.

**Tool launches but shows "Invalid LTI launch".** הקורס נמצא במבנה tenant/ארגון שונה ממה שהפריסה מכסה, או שהפריסה נכבתה לאחר הרישום. בדקו שוב **Admin Tools** > **Manage Extensibility** > **LTI Advantage** > FastComments > מתג **Enabled** וודאו את רשימת יחידות הארגון של הפריסה.

**Names and roles missing inside FastComments.** Brightspace שולחת שיגורי LTI עם טענות Names and Role Provisioning Services (NRPS). אם הקורס שודרג מקישור LTI 1.1 ישן, ה-launch חסר את טענות ה-`name` ו-`email`. הוסיפו מחדש את נושא FastComments דרך **Add Existing** (אל תנווטו את הקישור הישן) כדי שה-launch ישתמש ב-LTI 1.3.

**Embed shows a login screen instead of auto-SSO.** נושא ה-HTML הוכנס כ-iframe פשוט שמצביע ל-FastComments במקום דרך **Insert Stuff** > **LTI Advantage**. iframes פשוטים מדלגים על ה-LTI launch ומביאים משתמשים לדף הציבורי של FastComments. מחקו את ה-iframe והכניסו מחדש דרך זרימת Insert Stuff.
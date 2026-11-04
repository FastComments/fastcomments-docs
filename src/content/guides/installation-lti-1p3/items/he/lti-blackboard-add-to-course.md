לאחר שמנהל מערכת רשם את FastComments ככלי LTI 1.3 Advantage ואישר את מדיניות המוסד, מרצים מוסיפים אותו לקורסים דרך נקודות ההצבה הסטנדרטיות של Blackboard. השלבים המדויקים שונים בין Ultra Course View ל-Original Course View, ולכן שניהם מתוארים להלן.

#### Ultra Course View

Ultra Course View הוא ברירת המחדל ב-Blackboard Learn SaaS החל מ-2026.

1. פתחו את הקורס ועברו לדף **Course Content**.
2. רחפו או הקישו במקום שבו אתם רוצים ששרשור התגובות יופיע במתאר, ולחצו על כפתור ה-**+** הסגול (הוספת תוכן).
3. בחרו **Content Market**. פאנל ה-Content Market מציג את כל כלי ה-LTI המאושרים והצבות ה-Building Block של המוסד שלכם.
4. מצאו את האריח של **FastComments** ולחצו עליו. Blackboard יוצר פריט תוכן במיקום שבו פתחתם את תפריט ה-**+**.
5. הפריט מופיע במתאר כרשומה "Visible to students" כברירת מחדל עבור מרצים שהגדירו **Hide from students** ככבוי בברירת המחדל האישית שלהם. אם ברירת המחדל שלכם היא **Hidden**, הפריט נוצר מוסתר ותוכלו להפוך את בורר הנראות בשורת הפריט כשתהיו מוכנים.
6. כדי לשנות את שם הפריט, לחצו על הכותרת במתאר והקלידו תווית חדשה. הכותרת שהסטודנטים רואים במתאר היא בלתי תלויה במזהה השרשור של FastComments, ולכן שינוי שם בטוח בכל עת.

אם אינכם רואים את **Content Market** כאפשרות, המוסד שלכם הסתיר את ההצבה. ניתן להגיע לאותו בורר גם דרך **More tools** באותו תפריט **+** תחת קבוצת **LTI Tools**.

#### Original Course View

Original Course View עדיין נתמך ב-Learn SaaS ונשאר החוויה העיקרית עבור אתרי Learn 9.1 המתארחים בעצמם בקו המהדורה Q4 2024 CU.

1. פתחו את הקורס והיכנסו ל-**Content Area** (למשל, אזור **Information** או **Content** המוגדר כברירת מחדל בתפריט הקורס).
2. הפעילו את **Edit Mode** באמצעות המתג בפינה השמאלית העליונה של הדף.
3. לחצו על **Build Content** בסרגל הפעולות.
4. תחת תפריט המשנה **Learning Tools**, לחצו על **FastComments**. תפריט המשנה Learning Tools מאוכלס מהצבות כלי LTI 1.3 לאחר שמנהל רושם את הכלי. אם אינכם רואים אותו, עיינו בסעיף הבעיות המוכרות להלן.
5. בטופס **Create FastComments**, הגדירו:
   - **Name**: התווית שהסטודנטים רואים באזור התוכן.
   - **Description**: טקסט אופציונלי המוצג מעל השרשור המוטמע.
   - **Permit Users to View this Content**: מתג זמינות כן/לא.
   - **Track Number of Views**: הפעילו אם אתם רוצים את סטטיסטיקות הצפיות הפר-פריט של Blackboard. FastComments מפעיל ניתוח נתונים משלו באופן עצמאי.
   - **Date and Time Restrictions**: חלונות **Display After** / **Display Until** אופציונליים.
6. שלחו. הכלי מופיע כפריט לחיץ באזור התוכן.

#### הטמעה בתוך פריט או מסמך

בשני מצבי הקורס, מרצים מטמיעים את FastComments בתוך גוף של פריט, מסמך או כל שדה טקסט עשיר באמצעות כפתור ה-LTI Advantage של עורך התוכן.

Ultra Course View:

1. צרו או ערכו **Document**.
2. לחצו על **Add content** בתוך גוף המסמך במקום שבו אתם רוצים שהשרשור יופיע.
3. בסרגל הכלים של העורך, פתחו את תפריט **Insert content** ולחצו על **Content Market** (נקודת הכניסה של LTI Advantage / Deep Linking).
4. בחרו **FastComments**. FastComments מחזיר מטען deep-link ו-Blackboard מכניס בלוק מוטמע בגוף המסמך במיקום הסמן.
5. שמרו את המסמך. הסטודנטים רואים את השרשור מוצג בתוך הזרימה כשהם גוללים על פניו.

Original Course View:

1. ערכו כל פריט עם גוף טקסט עשיר.
2. בסרגל הכלים של עורך התוכן, לחצו על סמל הפלוס **Add Content** ובחרו **Content Market** (מסומן **Add Content from External Tool** בגרסאות Q4 2024 CU ישנות יותר).
3. בחרו **FastComments**. העורך מכניס בלוק placeholder המפנה למשאב ה-deep-linked.
4. שלחו את הפריט.

כל הטמעת deep-link מייצרת שרשור FastComments משלה, ולכן פריט עם שני בלוקים מוטמעים של FastComments מכיל שני זרמי תגובות עצמאיים.

#### נראות, תנאי שחרור והגבלות קבוצות

פריטי תוכן של FastComments מתנהגים כמו כל פריט תוכן אחר של Blackboard מבחינת כללי בקרת הגישה המרובבים עליהם.

- Ultra: לחצו על בורר הנראות בשורה (**Visible to students**, **Hidden from students**, **Conditional availability**). Conditional availability תומך בחלונות תאריך/שעה, כללי ביצועים מול פריטי ציונים, וכללי חברים מול קבוצות קורס.
- Original: פתחו את תפריט ההקשר של הפריט ובחרו **Adaptive Release** או **Adaptive Release: Advanced** כדי להגביל את הכלי לפי תאריך, חברות, ציון או סטטוס סקירה. השתמשו ב-**Set Group Availability** בפריט כדי להגביל לקבוצות קורס ספציפיות.

FastComments מכבד את מה שהשער של Blackboard מחליט. אם Blackboard מסתיר את הפריט מסטודנט, ההשקה של LTI לא מתרחשת עבור אותו סטודנט והוא לא מופיע בתצוגת המנחה.

#### התנהגות ספר הציונים

FastComments אינו מדווח ציונים חזרה דרך LTI Advantage Assignment and Grade Services. לא נוצרת באופן אוטומטי עמודת ציונים עבור פריטי תוכן של FastComments.

אם ה-tenant של Blackboard שלכם מוגדר ליצור עמודת ציונים באופן אוטומטי עבור כל פריט תוכן חדש ללא קשר למטא-נתוני ההערכה, עמודה ריקה תופיע בכל זאת. כדי להסתיר אותה:

- Ultra: פתחו את **Gradebook**, לחצו על כותרת העמודה, בחרו **Edit**, וכבו את **Show to students** וגם **Include in calculations**. או השתמשו ב-**Delete** אם המוסד שלכם מתיר מחיקת עמודות עבור פריטים ללא הערכה.
- Original: פתחו את **Grade Center**, לחצו על החץ של העמודה, בחרו **Hide from Users (on/off)**, ובאופן אופציונלי **Hide from Instructor View** תחת **Column Organization**.

#### מה סטודנטים רואים

כאשר סטודנט פותח את פריט FastComments או גולל לבלוק מוטמע:

1. Blackboard משיק את הודעת ה-LTI 1.3 אל FastComments. הסטודנט מחובר באמצעות SSO עם זהות ה-Blackboard שלו (שם, אימייל, אווטאר, תפקיד) מבלי לראות טופס התחברות.
2. שרשור התגובות מוצג ב-iframe. שרשור, תגובות, אזכורים ותגובות רגש זמינים כולם בהתאם להגדרות ווידג'ט התגובות שהוגדרו ב-FastComments.
3. התגובות שלו מיוחסות לחשבון ה-Blackboard שלו. אם הסטודנט עורך את שמו או תמונתו ב-Blackboard מאוחר יותר, ההשקה הבאה מעדכנת את הפרופיל ב-FastComments.

מיפוי תפקידים מ-Blackboard ל-FastComments:

- **System Administrator** ו-**Course Builder** ממופים ל-**admin** של FastComments.
- **Instructor** ו-**Teaching Assistant** ממופים ל-**moderator** של FastComments.
- **Student**, **Guest** ו-**Observer** ממופים ל-**commenter** של FastComments.

מנחים רואים פקדי ניהול (נעיצה, הסתרה, חסימה, מחיקה) בתוך הזרימה על כל תגובה בשרשור.

#### נעילת גישה ציבורית (מומלץ)

כברירת מחדל, נתוני התגובות של FastComments קריאים לציבור. כל מי שיכול לנחש את ה-URL של שרשור או נקודת קצה של API יכול לצפות בתגובותיו, גם מחוץ ל-Blackboard. עבור דיוני קורס כמעט בוודאות תרצו להגביל את הצפייה לסטודנטים רשומים בלבד.

פתחו את <a href="https://fastcomments.com/auth/my-account/customize-widget" target="_blank">דף התאמת הווידג'ט</a> שלכם וצרו כלל עם **Require SSO To View Comments** מופעל, ואז הגדירו את רמת האבטחה ל-**Secure SSO** כך ששרשורים ייטענו רק דרך השקת ה-LTI החתומה.

ראו [Protecting Comment Threads With Single-Sign-On](/guide-customizations-and-configuration.html#sso-require-to-view-comments) להדרכה המלאה, כולל כיצד להגביל את הכלל לדומיין או דף בודד.

#### היקף שרשורים

FastComments מגדיר את היקף כל שרשור לפי **(מארח Blackboard, מזהה קורס, מזהה resource link)**. שני פריטי FastComments באותו קורס מייצרים שני שרשורים. אותו פריט שהועתק בין שני קונכיות קורס (למשל, דרך העתקת קורס) מייצר שני שרשורים, מכיוון ש-Blackboard מנפיק מזהה resource link חדש במהלך ההעתקה. כדי לשמור שרשור משותף על פני העתקות קורס, השתמשו ב-Deep Linking עם URN שרשור מפורש שהוגדר ב-FastComments לפני השקת ההעתקה.

#### בעיות מוכרות ספציפיות ל-Blackboard

**אריח FastComments חסר בתפריט Build Content (Original) או ב-Content Market (Ultra).** מנהל המערכת אישר את הכלי אך השאיר מדיניות מוסדית שחוסמת את ההצבה הרלוונטית. עברו אל **Administrator Panel** > **Integrations** > **LTI Tool Providers**, ערכו את הרשומה של FastComments, וודאו ששתי ההצבות **Course Content Tool** (Original) ו-**Course Content Tool - allow students** / **Deep Linking content tool** (Ultra) מופעלות. שמרו ורעננו את דף הקורס.

**שגיאת "Tool not configured for this context" או "Tool is not deployed" בהשקה.** היקף ה-deployment שנרשם במהלך הרישום הדינמי אינו תואם להקשר המוסדי שאליו שייך הקורס. ברשומת ספק הכלים של Blackboard, ודאו שה-**Deployment ID** תואם למה ש-FastComments מציג בדף ההגדרות LTI 1.3 שלו עבור tenant זה. אם הם שונים, מחקו את ההצבה והריצו רישום דינמי מחדש מ-URL רישום טרי (<a href="https://fastcomments.com/auth/my-account/lti-config" target="_blank">קבלו אותו כאן</a>).

**גובה ה-iframe נראה קבוע או שהתוכן נחתך.** חלק מה-tenants של Blackboard מגיעים עם מדיניות Content Security Policy מחמירה שחוסמת את הודעת ה-postMessage המובנית של שינוי גודל ה-iframe של LTI. FastComments פולט גם את הודעת `lti.frameResize` בסגנון Canvas וגם את הודעת `org.imsglobal.lti.frameResize` לפי מפרט IMS כדי למקסם תאימות, אך עקיפת CSP ברמת tenant חוסמת את המאזין בהורה. בקשו ממנהל המערכת לוודא ש-`*.fastcomments.com` נמצא ברשימת ההיתרים של כלי ה-LTI ושאין כותרת CSP מותאמת שמסננת אירועי postMessage. לאחר מכן שינוי הגודל עובד ללא הגדרה נוספת.

**העתקת קורס משכפלת שרשורים.** העתקת קורס של Blackboard מנפיקה מזהי resource link חדשים עבור הצבות LTI, ולכן קורסים מועתקים מתחילים עם שרשורים ריקים. זה צפוי. אם אתם צריכים שהקורס המועתק ירש את השרשור המקורי, הגדירו Deep Linking עם URN שרשור מפורש לפני ההעתקה, או פנו לתמיכת FastComments למיפוי מחדש של מזהי שרשורים בכמות גדולה.

**סטודנט רואה שגיאת Blackboard כללית בהשקה.** הסיבה היא claim של `email` חסר או לא עדכני. ודאו שמדיניות המוסד עבור FastComments מפעילה **Role**, **Name** ו-**Email Address** תחת **User Fields to Send**. שמרו, ואז השיקו שוב בסשן דפדפן טרי.
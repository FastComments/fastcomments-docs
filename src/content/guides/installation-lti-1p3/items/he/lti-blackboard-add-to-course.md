ברגע שמנהל רשם את FastComments ככלי LTI 1.3 Advantage ואישר את מדיניות המוסד, מרצים מוסיפים אותו לקורסים דרך נקודות המיקום הסטנדרטיות ב-Blackboard. השלבים המדויקים שונים בין Ultra Course View ו-Original Course View, לכן שניהם מכוסים למטה.

#### Ultra Course View

Ultra Course View הוא ברירת המחדל ב-Blackboard Learn SaaS מאז 2026.

1. פתח את הקורס ועבור לדף **Course Content**.
2. הרחף או הקש במקום שבו ברצונך שהשרשור של ההערות יופיע במבנה ולחץ על הכפתור הסגול **+** (Add content).
3. בחר **Content Market**. פאנל ה-Content Market מציג את כל כלי ה-LTI המאושרים ואת מיקומי ה-Building Block עבור המוסד שלך.
4. מצא את ה-Tile של **FastComments** ולחץ עליו. Blackboard יוצר פריט תוכן במיקום שבו פתחת את התפריט **+**.
5. הפריט נוצר במבנה כברירת מחדל ככניסה "Visible to students" עבור מרצים שההגדרה האישית שלהם היא **Hide from students** כבוי. אם ברירת המחדל שלך היא **Hidden**, הפריט נוצר כמוסתר ותוכל להפוך את בורר הנראות בשורת הפריט כשהיה מתאים.
6. כדי לשנות את שם הפריט, לחץ על הכותרת במבנה והקלד תגית חדשה. הכותרת שהסטודנטים רואים במבנה היא בלתי תלויה במזהה השרשור של FastComments, לכן שינוי שם הוא בטוח בכל עת.

אם אינך רואה את **Content Market** כאפשרות, המוסד חסם את המיקום. ניתן גם להגיע לאותו בורר דרך **More tools** באותו תפריט **+** תחת קבוצת **LTI Tools**.

#### Original Course View

Original Course View עדיין נתמך ב-Learn SaaS ונשאר החוויה העיקרית עבור אתרי Learn 9.1 self-hosted בשורת ה-CU של Q4 2024.

1. פתח את הקורס והיכנס ל-**Content Area** (לדוגמה, אזור ברירת המחדל **Information** או **Content** בתפריט הקורס).
2. הפעל את **Edit Mode** באמצעות המתג בפינה העליונה-ימנית של הדף.
3. לחץ על **Build Content** בסרגל הפעולות.
4. תחת תפריט המשנה **Learning Tools**, לחץ על **FastComments**. תפריט המשנה של Learning Tools מתמלא ממיקומי כלי LTI 1.3 אחרי שמנהל רשם את הכלי. אם אינך רואה אותו, ראה את סעיף ה-gotchas למטה.
5. בטופס **Create FastComments**, הגדר:
   - **Name**: התווית שהסטודנטים רואים באזור התוכן.
   - **Description**: טקסט אופציונלי שיוצג מעל השרשור המוטמע.
   - **Permit Users to View this Content**: מתג זמינות כן/לא.
   - **Track Number of Views**: הפעל אם ברצונך את סטטיסטיקת הצפיות לפריט של Blackboard. ל-FastComments יש אנליטיקס משלו שפועלים באופן עצמאי.
   - **Date and Time Restrictions**: חלונות אופציונליים **Display After** / **Display Until**.
6. שלח. הכלי מופיע כפריט קליקבילי באזור התוכן.

#### Embedding Inside an Item or Document

בשתי תצורות הקורס, מרצים מטמיעים את FastComments בקו בתוך גוף של Item, Document, או כל שדה טקסט עשיר דרך כפתור ה-LTI Advantage בעורך התוכן.

Ultra Course View:

1. צור או ערוך **Document**.
2. לחץ על **Add content** בתוך גוף המסמך במקום שבו תרצה שהשרשור יופיע.
3. בסרגל הכלים של העורך, פתח את התפריט **Insert content** ולחץ על **Content Market** (נקודת הכניסה של LTI Advantage / Deep Linking).
4. בחר **FastComments**. FastComments מחזיר payload של deep-link ו-Blackboard מכניס בלוק מוטמע בגוף המסמך במיקום הסמן.
5. שמור את המסמך. הסטודנטים יראו את השרשור מוטמע כשהם גוללים אליו.

Original Course View:

1. ערוך כל פריט שיש לו גוף טקסט עשיר.
2. בסרגל הכלים של Content Editor, לחץ על סמל ה-Plus של **Add Content** ובחר **Content Market** (מתויג **Add Content from External Tool** ב-CUs ישנים של Q4 2024).
3. בחר **FastComments**. העורך מכניס בלוק מייצג שמפנה למשאב ה-deep-linked.
4. שלח את הפריט.

כל הטמעת deep-link מייצרת שרשור FastComments עצמאי, כך שפריט עם שני בלוקי FastComments מוטמעים יכיל שני סטרימים של הערות נפרדים.

#### Visibility, Release Conditions, and Group Restrictions

פריטי תוכן של FastComments מתנהגים כמו כל פריט תוכן אחר ב-Blackboard מבחינת חוקי בקרת גישה שמונחים עליהם.

- Ultra: לחץ על בורר הנראות בשורה (**Visible to students**, **Hidden from students**, **Conditional availability**). זמיןויות מותנות תומכות בחלונות תאריכים/זמנים, כללי ביצועים מול פריטי ציון, וכללי חברות כנגד קבוצות קורס.
- Original: פתח את תפריט ההקשר של הפריט ובחר **Adaptive Release** או **Adaptive Release: Advanced** כדי להגביל את הכלי לפי תאריך, חברות, ציון, או מצב סקירה. השתמש ב-**Set Group Availability** על הפריט כדי להגביל לקבוצות קורס ספציפיות.

FastComments מכבד את מה שהשער של Blackboard קובע. אם Blackboard מסתיר את הפריט מתלמיד, ההשקה של ה-LTI לעולם לא מתבצעת בשביל אותו תלמיד והם לא מופיעים בממשק המנהלים.

#### Gradebook Behavior

FastComments איננו מדווח ציונים חזרה דרך LTI Advantage Assignment and Grade Services. לא נוצרת עמודת ציון אוטומטית עבור פריטי תוכן של FastComments.

אם השוכר (tenant) של Blackboard שלך מוגדר ליצור אוטומטית עמודת ציון עבור כל פריט תוכן חדש ללא קשר למטה-נתוני הניקוד, תופיע עמודה ריקה בכל זאת. כדי להסתיר אותה:

- Ultra: פתח את ה-**Gradebook**, לחץ על כותרת העמודה, בחר **Edit**, וכבה את **Show to students** בנוסף ל-**Include in calculations**. או השתמש ב-**Delete** אם המוסד שלך מאפשר מחיקה של עמודות לפריטים ללא דירוג.
- Original: פתח את ה-**Grade Center**, לחץ על ה-chevron של העמודה, בחר **Hide from Users (on/off)**, ואופציונלית **Hide from Instructor View** תחת **Column Organization**.

#### What Students See

כאשר סטודנט פותח את פריט FastComments או גולל לבלוק מוטמע:

1. Blackboard משגר את הודעת ה-LTI 1.3 ל-FastComments. הסטודנט נכנס דרך SSO באמצעות זהות ה-Blackboard שלו (שם, אימייל, אוואטר, תפקיד) מבלי לראות טופס כניסה.
2. שרשור ההערות מצויר ב-iframe. תיוג שרשורים, תגובות, אזכורים, וריאקציות זמינים בהתאם להגדרות ווידג'ט ההערות המוגדרות ב-FastComments.
3. ההערות שלהם משויכות לחשבון ה-Blackboard שלהם. אם הסטודנט מעדכן את שמו או התמונה ב-Blackboard מאוחר יותר, ההשקה הבאה תעדכן את הפרופיל ב-FastComments.

מיפוי תפקידים מ-Blackboard ל-FastComments:

- **System Administrator** ו-**Course Builder** ממופים ל-FastComments כ-**admin**.
- **Instructor** ו-**Teaching Assistant** ממופים ל-FastComments כ-**moderator**.
- **Student**, **Guest**, ו-**Observer** ממופים ל-FastComments כ-**commenter**.

מנחים רואים את כלי המודרציה (הצמדה, הסתרה, חסימה, מחיקה) בשורה על כל תגובה בשרשור.

#### Thread Scoping

FastComments מגדיר את ההיקף של כל שרשור על פי **(Blackboard host, course ID, resource link ID)**. שני פריטי FastComments באותו קורס מייצרים שני שרשורים. אותו פריט המועתק דרך שתי מעטפות קורס (למשל, דרך העתקת קורס) מייצר שני שרשורים, מכיוון ש-Blackboard מוציאה מזהה resource link חדש במהלך ההעתקה. כדי לשמור על שרשור משותף בחצאי קורסים שעוברים העתקה, השתמש ב-Deep Linking עם URN של שרשור מפורש המוגדר ב-FastComments לפני הרצת ההעתקה.

#### Blackboard-Specific Gotchas

**FastComments tile חסר מתפריט Build Content (Original) או Content Market (Ultra).** המנהל אישר את הכלי אך השאיר מדיניות מוסד שחוסמת את המיקום הרלוונטי. עבור אל **Administrator Panel** > **Integrations** > **LTI Tool Providers**, ערוך את הערך של FastComments, ואשר ששני המיקומים **Course Content Tool** (Original) ו-**Course Content Tool - allow students** / **Deep Linking content tool** (Ultra) מופעלים. שמור ורענן את דף הקורס.

**שגיאה "Tool not configured for this context" או "Tool is not deployed" בהשקה.** היקף ההטמעה שנרשם במהלך הרישום הדינמי אינו תואם להקשר המוסדי שהקורס שייך אליו. ברשומת ספק הכלי של Blackboard, אמת שה-**Deployment ID** תואם למה ש-FastComments מציג בדף ה-LTI 1.3 Configuration עבור ה-tenant הזה. אם הם שונים, מחק את המיקום והריץ מחדש רישום דינמי מתוך כתובת רישום חדשה (<a href="https://fastcomments.com/auth/my-account/lti-config" target="_blank">קבל/י אותו כאן</a>).

**גובה ה-iframe נראה קבוע או שהתוכן נחתך.** כמה שוכרים של Blackboard מציעים מדיניות אבטחה של תוכן (Content Security Policy) נוקשה שחוסמת את ברירת המחדל של שינוי גובה ה-iframe דרך postMessage של LTI. FastComments משדר גם את הודעת סגנון Canvas `lti.frameResize` וגם את הודעת המפרט של IMS `org.imsglobal.lti.frameResize` כדי למקסם תאימות, אך גזירת CSP ברמת ה-tenant חוסמת את המאזין בהורה. בקש מהמנהל שלך לאשר ש-*.fastcomments.com מופיע ברשימת ההרשאה של כלי ה-LTI ושאין כותרת CSP מותאמת שמסירה אירועי postMessage. לאחר מכן שינוי הגודל יעבוד ללא תצורה נוספת.

**העתקת קורס משכפלת שרשורים.** העתקת קורס ב-Blackboard מנפיקה מזהי resource link חדשים עבור מיקומי LTI, לכן קורסים שהועתקו מתחילים עם שרשורים ריקים. זה צפוי. אם אתה צריך שהקורס שהועתק ירש את השרשור המקורי, הגדר Deep Linking עם URN של שרשור מפורש לפני ההעתקה, או פנה לתמיכת FastComments כדי למפות מחדש מזהי שרשורים בכמות גדולה.

**סטודנט רואה שגיאה גנרית של Blackboard בהשקה.** הגורם הוא claim של `email` חסר או לא עדכני. אמת את מדיניות המוסד ל-FastComments שיש בה אפשרויות **Role**, **Name**, ו-**Email Address** מופעלות תחת **User Fields to Send**. שמור, ואז השקה שוב בסשן דפדפן חדש.
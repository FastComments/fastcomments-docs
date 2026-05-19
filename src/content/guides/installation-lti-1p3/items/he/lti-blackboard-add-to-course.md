ברגע שמנהל הרשיא את FastComments ככלי LTI 1.3 Advantage ואישר את מדיניות המוסד, מרצים מוסיפים אותו לקורסים דרך נקודות המיקום הסטנדרטיות של Blackboard. השלבים המדויקים שונים בין Ultra Course View ו-Original Course View, לכן שניהם מפורטים להלן.

#### Ultra Course View

Ultra Course View הוא ברירת המחדל ב-Blackboard Learn SaaS מאז 2026.

1. פתח את הקורס ועבור לעמוד ה-**Course Content**.
2. הרחף או הקש במקום שבו אתה רוצה שהשרשור של התגובות יופיע במבנה וקליק על הכפתור הסגול **+** (Add content).
3. בחר **Content Market**. פנל ה-Content Market מפרט כל כלי LTI מאושר וכל מיקום Building Block עבור המוסד שלך.
4. מצא את התיבה של **FastComments** ולחץ עליה. Blackboard יוצר פריט תוכן במיקום שבו פתחת את התפריט **+**.
5. הפריט מופיע במבנה כברירת מחדל ככניסה "Visible to students" עבור מרצים שיש להם כברירת מחדל אישי את **Hide from students** כבצורה כבויה. אם ברירת המחדל שלך היא **Hidden**, הפריט נוצר כמוסתר ותוכל להחליף את מתג הנראות בשורת הפריט כשאתה מוכן.
6. כדי לשנות שם לפריט, לחץ על הכותרת במבנה והקלד תווית חדשה. הכותרת שהתלמידים רואים במבנה בלתי תלויה במזהה השרשור של FastComments, לכן שינוי השם בטוח בכל זמן.

אם אינך רואה את **Content Market** כאופציה, המוסד שלך מחביא את המיקום. ניתן גם להגיע לאותו בורר דרך **More tools** באותו תפריט **+** תחת קבוצת **LTI Tools**.

#### Original Course View

Original Course View נתמך עדיין ב-Learn SaaS ונשאר החוויה העיקרית לאתרים של Learn 9.1 self-hosted בקו ה-CU של Q4 2024.

1. פתח את הקורס והיכנס ל-**Content Area** (לדוגמה, אזור ברירת המחדל **Information** או **Content** בתפריט הקורס).
2. הפעל את **Edit Mode** עם מתג בפינה העליונה-ימנית של הדף.
3. לחץ על **Build Content** בסרגל הפעולות.
4. תחת תפריט המשנה **Learning Tools**, לחץ על **FastComments**. תפריט המשנה Learning Tools מתמלא ממיקומי כלי LTI 1.3 לאחר שננהל הרשיא את הכלי. אם אתה לא רואה אותו, ראה את הסעיף של הבעיות בהמשך.
5. בטופס **Create FastComments**, הגדר:
   - **Name**: התווית שהתלמידים רואים באזור התוכן.
   - **Description**: טקסט אופציונלי המוצג מעל השרשור המוטמע.
   - **Permit Users to View this Content**: מתג זמינות כן/לא.
   - **Track Number of Views**: אפשר אם אתה רוצה את סטטיסטיקות הצפיות לפריט ב-Blackboard. ל-FastComments יש אנליטיקה משלו באופן עצמאי.
   - **Date and Time Restrictions**: חלונות אופציונליים של **Display After** / **Display Until**.
6. שלח. הכלי מופיע כפריט קליקבילי באזור התוכן.

#### Embedding Inside an Item or Document

בשתי תצורות הקורס, מרצים מטמיעים את FastComments באופן פנימי בתוך גוף של Item, Document, או כל שדה טקסט עשיר דרך כפתור LTI Advantage בעורך התוכן.

Ultra Course View:

1. צור או ערוך **Document**.
2. לחץ על **Add content** בתוך גוף המסמך במקום שבו אתה רוצה שהשרשור יופיע.
3. בסרגל העורך, פתח את תפריט **Insert content** ולחץ על **Content Market** (נקודת הכניסה של LTI Advantage / Deep Linking).
4. בחר **FastComments**. FastComments מחזיר payload של deep-link ו-Blackboard מכניס בלוק מוטמע בגוף המסמך במיקום הסמן.
5. שמור את המסמך. תלמידים רואים את השרשור מוצג בקו בתוך המסמך כשהם גוללים לעברו.

Original Course View:

1. ערוך כל פריט עם גוף טקסט עשיר.
2. בסרגל העורך, לחץ על סמל הפלוס **Add Content** ובחר **Content Market** (מסומן **Add Content from External Tool** בגרסאות Q4 2024 הישנות יותר).
3. בחר **FastComments**. העורך מכניס בלוק מציין מקום שמתייחס למשאב ה-deep-linked.
4. שלח את הפריט.

כל הטמעה של deep-link מייצרת את שרשור ה-FastComments שלה, כך שפריט עם שני בלוקים מוטמעים של FastComments יכיל שני זרמי תגובות בלתי תלויים.

#### Visibility, Release Conditions, and Group Restrictions

פריטי תוכן של FastComments מתנהגים כמו כל פריט תוכן אחר ב-Blackboard מבחינת כללי שליטת הגישה שמניחים עליהם.

- Ultra: לחץ על מתג הנראות בשורה (**Visible to students**, **Hidden from students**, **Conditional availability**). הזמינות המותנית תומכת בחלונות תאריך/שעה, כללי ביצוע מול פריטים בטבלת הציונים, וכללי חברות מול קבוצות הקורס.
- Original: פתח את תפריט ההקשר של הפריט ובחר **Adaptive Release** או **Adaptive Release: Advanced** כדי לחסום את הכלי לפי תאריך, חברות, ציון, או סטטוס סקירה. השתמש ב-**Set Group Availability** על הפריט להגבלה לקבוצות קורס ספציפיות.

FastComments מכבד את ההחלטה של מערכת Blackboard. אם Blackboard מסתירה את הפריט מתלמיד, ההשקה של LTI לא מתבצעת עבור אותו תלמיד והם לא מופיעים בתצוגת המפקח.

#### Gradebook Behavior

FastComments לא מדווח ציונים חזרה דרך LTI Advantage Assignment and Grade Services. לא נוצר עמודת ציונים אוטומטית לפריטי תוכן של FastComments.

אם ה-tenant של Blackboard שלך מוגדר ליצירת עמודת טבלת ציונים אוטומטית עבור כל פריט תוכן חדש ללא קשר למטא-דאטת דירוג, תופיע עמודה ריקה בכל מקרה. כדי להסתיר אותה:

- Ultra: פתח את ה-**Gradebook**, לחץ על כותרת העמודה, בחר **Edit**, וכבה **Show to students** ועוד **Include in calculations**. או השתמש ב-**Delete** אם המוסד שלך מאפשר מחיקת עמודות לפריטים ללא ציון.
- Original: פתח את ה-**Grade Center**, לחץ על החץ בעמודת הציונים, בחר **Hide from Users (on/off)**, ואופציונלית **Hide from Instructor View** תחת **Column Organization**.

#### What Students See

כשהתלמיד פותח את פריט FastComments או גולל לבלוק מוטמע:

1. Blackboard משגר את הודעת LTI 1.3 אל FastComments. התלמיד מחובר באמצעות SSO בעזרת זהות Blackboard שלו (שם, אימייל, אווטאר, תפקיד) בלי לראות טופס התחברות.
2. שרשור התגובות מוצג ב-iframe. שִרשּוּר תגובות, תגובות משנה, אזכורים, ותגובות (reactions) זמינים בהתאם להגדרות ווידג'ט התגובות ב-FastComments.
3. התגובות שלהם משויכות לחשבון ה-Blackboard שלהם. אם התלמיד יערוך את שמו או תמונתו ב-Blackboard מאוחר יותר, ההשקה הבאה תעדכן את הפרופיל ב-FastComments.

מיפוי תפקידים מ-Blackboard אל FastComments:

- **System Administrator** ו-**Course Builder** מתמפים ל-FastComments **admin**.
- **Instructor** ו-**Teaching Assistant** מתמפים ל-FastComments **moderator**.
- **Student**, **Guest**, ו-**Observer** מתמפים ל-FastComments **commenter**.

מנחים רואים בקרות מתינות (pin, hide, ban, delete) מקוּמוֹת על כל תגובה בשרשור.

#### Lock Down Public Access (Recommended)

כברירת מחדל, נתוני תגובות של FastComments ניתנים לקריאה באופן ציבורי. כל מי שיכול לנחש את כתובת ה-URL של השרשור או נקודת ה-API שלו יכול לצפות בתגובות, אפילו מחוץ ל-Blackboard. עבור דיונים בקורסים קרוב לוודאי שתרצה להגביל את הצפייה רק לסטודנטים הרשומים.

פתח את <a href="https://fastcomments.com/auth/my-account/customize-widget" target="_blank">עמוד ההתאמה האישית של הווידג\'ט</a> וצרו כלל עם **Require SSO To View Comments** מופעל, ואז קבע את רמת האבטחה ל-**Secure SSO** כדי ששרשורים ייטענו רק דרך השקת LTI החתומה.

ראה [Protecting Comment Threads With Single-Sign-On](/guide-customizations-and-configuration.html#sso-require-to-view-comments) להליכה מלאה של השלבים, כולל איך להגביל את הכלל לדומיין או דף יחיד.

#### Thread Scoping

FastComments מגדיר את כל שרשור לפי **(Blackboard host, course ID, resource link ID)**. שני פריטי FastComments באותו קורס מייצרים שני שרשורים. אותו פריט המועתק בין שני קורסים (למשל דרך העתקת קורס) מייצר שני שרשורים, כי Blackboard מנפיק מזהה resource link חדש במהלך ההעתקה. כדי לשמור על שרשור משותף בין העתקות קורס, השתמש ב-Deep Linking עם URN של שרשור מפורש המוגדר ב-FastComments לפני ביצוע ההעתקה.

#### Blackboard-Specific Gotchas

**FastComments tile חסר מתפריט Build Content (Original) או Content Market (Ultra).** המנהל אישר את הכלי אך השאיר מדיניות מוסד שחוסמת את המיקום הרלוונטי. עבור אל **Administrator Panel** > **Integrations** > **LTI Tool Providers**, ערוך את רשומת FastComments, ואשר ששני המיקומים **Course Content Tool** (Original) ו-**Course Content Tool - allow students** / **Deep Linking content tool** (Ultra) מופעלים. שמור ורענן את עמוד הקורס.

**שגיאה "Tool not configured for this context" או "Tool is not deployed" בעת ההשקה.** טווח הפריסה שנרשם במהלך ההרשמה הדינמית לא תואם להקשר המוסדי שהקורס שייך לו. ברשומת ספק הכלים של Blackboard, אמת שה-**Deployment ID** תואם למה ש-FastComments מציג בדף ה-LTI 1.3 Configuration עבור ה-tenant הזה. אם הם שונים, מחק את המיקום והריץ שוב הרשמה דינמית מ-URL הרשמה חדש (<a href="https://fastcomments.com/auth/my-account/lti-config" target="_blank">get it here</a>).

**גובה ה-iframe נראה קבוע או תוכן נחתך.** חלק מ-tenant-ים של Blackboard מגיעים עם Content Security Policy נוקשה שחוסם את postMessage של שינוי גודל ה-LTI המוגדר כברירת מחדל. FastComments שולחת גם את ההודעה בסגנון Canvas `lti.frameResize` וגם את גרסת תקן ה-IMS `org.imsglobal.lti.frameResize` כדי למקסם תאימות, אך ביטול CSP ברמת ה-tenant חוסם את המאזין בצד ההורה. בקש מהמנהל שלך לאשר ש-`*.fastcomments.com` ברשימת ההרשאה של כלי ה-LTI ושאין כותרת CSP מותאמת שמסירה אירועי postMessage. לאחר מכן שינוי הגודל יעבוד בלי הגדרות נוספות.

**העתקת קורס משכפלת שרשורים.** העתקת קורס ב-Blackboard מנפיקה מזהי resource link חדשים למיקומי LTI, כך שקורסים שהועתקו מתחילים עם שרשורים ריקים. זה צפוי. אם אתה צריך שהקורס המועתק יורש את השרשור המקורי, הגדר Deep Linking עם URN של שרשור מפורש לפני ההעתקה, או פנה לתמיכת FastComments כדי לשייך מחדש מזהי שרשור בכמות גדולה.

**התלמיד רואה שגיאה כללית של Blackboard בעת ההשקה.** הסיבה היא claim של `email` חסר או מיושן. אמת שמדיניות המוסד עבור FastComments כוללת את **Role**, **Name**, ו-**Email Address** תחת **User Fields to Send**. שמור, ואז השקה שוב בסשן דפדפן חדש.
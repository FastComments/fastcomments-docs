לאחר ש-FastComments נרשם בפלטפורמה, המדריכים מוסיפים אותו לתוכן הקורס באמצעות זרימות הכלים החיצוניים הסטנדרטיות של הפלטפורמה. דף זה מכסה את Sakai 23.x ו-Schoology Enterprise.

#### Lock Down Public Access (Recommended)

כברירת מחדל נתוני התגובות של FastComments ניתנים לקריאה לציבור בכל אחת מהפלטפורמות. כל מי שיכול לנחש את כתובת ה-thread או נקודת הקצה של ה-API יכול לצפות בתגובות, גם מחוץ ל-Sakai או Schoology. עבור דיונים בקורס סביר להניח שתרצו להגביל את הצפייה לתלמידים הרשומים בלבד.

פתחו את <a href="https://fastcomments.com/auth/my-account/customize-widget" target="_blank">דף ההתאמה אישית של ה-widget</a> וצרו כלל עם **Require SSO To View Comments** מופעל, ואז קבעו את רמת האבטחה ל-**Secure SSO** כך ששרשורים ייטענו רק דרך ה-LTI launch החתום.

ראו [הגנה על שרשורי תגובות באמצעות Single-Sign-On](/guide-customizations-and-configuration.html#sso-require-to-view-comments) להדרכה המלאה, כולל כיצד להגביל את הכלל לדומיין או לדף יחיד.

#### Sakai

**1. Add FastComments to a site**

המנהל של האתר מאפשר את הכלי על בסיס כל אתר בנפרד:

1. פתחו את האתר ולחצו על **Site Info** בניווט השמאלי.
2. לחצו על **Manage Tools**.
3. גללו לרשימת **External Tools** והפעילו את **FastComments**.
4. לחצו על **Continue**, בדקו את רשימת הכלים, ואז לחצו **Finish**.

FastComments מופיע כעת כפריט ניווט משמאל באתר.

**2. Reorder the left-nav entry**

עבורו ל-**Site Info** > **Tool Order**. גררו את **FastComments** למיקום הרצוי ולחצו **Save**. אתם יכולים גם לשנות את תווית הניווט ולהסתיר אותה מהסטודנטים במסך זה.

**3. Embed inline in a Lessons page**

כדי למקם את FastComments ישירות בתוך דף **Lessons** במקום ככלי נפרד בניווט המשמאל:

1. פתחו את כלי ה-**Lessons** באתר.
2. לחצו **Add Content** > **Add External Tool**.
3. בחרו **FastComments** מהרשימה.
4. אם FastComments פרסם Deep Linking בזמן הרישום, Sakai יפתח את בוחר התוכן של הכלי כדי שתוכלו לבחור או לסמן את ה-thread. אם Deep Linking לא פורסם, Sakai יכניס קישור פתיחה ברירת מחדל.
5. שמרו את פריט ה-Lessons.

לכל אינסטנס המוטמע יש את ה-thread הייחודי שלו, המושתה למשאב הקישור המתאים.

**4. Permission tweaks for student access**

Sakai מווסת את הפתיחות להטענת כלים חיצוניים דרך Realms. כדי לאשר שתלמידים יוכלו להפעיל את FastComments:

1. היכנסו כמנהל Sakai ופתחו **Administration Workspace** > **Realms**.
2. פתחו את ה-realm הרלוונטי (למשל, `!site.template.course` או ה-realm של האתר הספציפי).
3. אשרו של-role של `access` יש את `lti.launch` מופעל ושהרשאות התפקיד בקבוצת **external.tools** ניתנות.
4. שמרו את ה-realm.

להגדרות עקיפה ברמת האתר, המנהל יכול להתאים את נראות הכלי לכל תפקיד מ-**Site Info** > **Tool Order** על ידי הסתרה או הצגה של FastComments לפי תפקיד.

**5. What students see**

התלמידים לוחצים על פריט הניווט של FastComments (או גוללים אל הבלוק המוטמע ב-Lessons) ונכנסים ישירות לתצוגת השרשור של התגובות. SSO הוא אוטומטי: Sakai שולח את זהות המשתמש ב-LTI launch ו-FastComments מחבר אותם תחת חשבון Sakai שלהם.

מיפוי תפקידים:

- Sakai `Instructor` -> FastComments moderator
- Sakai `Admin` (admin in Administration Workspace) -> FastComments admin
- Sakai `Student` / `access` -> FastComments commenter

**6. Sakai gotchas**

- **Tool not visible in Manage Tools.** אם FastComments לא מופיע ברשימת External Tools, מנהל Sakai צריך לפתוח את רכיב הרישום של הכלים (**Administration Workspace** > **External Tools** > **FastComments**) ולהגדיר **Stealthed** ל-`false`. כלים שמסומנים כ-Stealthed מוסתרים ממ_picker של Manage Tools לכל אתר.
- **Launches breaking in shared-session browsers.** ה-CSRF token של הפורטל של Sakai קשור למושב הדפדפן. אם תלמיד מחובר לשני אתרי Sakai בכרטיסיות שונות או שיש לו מושב ישן, ה-launch יחזיר 403. תיקון: סגור כרטיסיות אחרות של Sakai, התנתק, היכנס מחדש ונסה להפעיל שוב. מנהלים יכולים גם להעלות את `sakai.csrf.token.cache.ttl` אם זה קורה בכל הקלאסטר.
- **Frame embedding.** אשרו ש-`lti.frameheight` ב-`sakai.properties` גדול מספיק (600 או יותר) כדי שהשרשור לא ייחתך בתוך דף Lessons.

#### Schoology

Schoology Enterprise כוללת שני תרחישי התקנה. אשרו איזה מהם חל לפני הוספת הכלי לקורס.

**1. Two installation scenarios**

- **(a) Enterprise-level install.** מנהל המערכת של Schoology התקין את FastComments ברמת הארגון והקצה אותו לכל הקורסים או לתבניות קורס ספציפיות. המדריכים מדלגים על ההתקנה ועוברים ישירות ל-"Add Materials".
- **(b) Instructor self-install.** המדריך מתקין את הכלי לקורס בודד מתוך **Course Options** > **External Tools** > **Install LTI Apps**. התקנה עצמית דורשת שמנהל המערכת אישר את אפליקציית FastComments ברמת הארגון קודם לכן.

**2. Add FastComments as a course material**

בתוך הקורס:

1. פתחו את הקורס ועברו ל-**Materials**.
2. לחצו **Add Materials** > **Add File/Link/External Tool**.
3. בחרו **External Tool**.
4. בחרו **FastComments** מרשימת הכלים הרשומים.
5. קבעו **Name** (זו התצוגה שהתלמידים רואים ברשימת החומרים) ותיאור אופציונלי **Description**.
6. השאירו את **Enable Grading** (grade passback) **OFF**. FastComments לא מדווח ציונים חזרה ל-Schoology, כך שהפעלת grade passback יוצרת עמודת ניקוד ריקה בספר הציונים.
7. לחצו **Submit**.

החומר מופיע כעת ברשימת החומרים של הקורס ונפתח את ה-thread של FastComments כאשר לוחצים עליו.

**3. Inline embedding via the Rich Text editor**

אם מנהל המערכת אפשר Deep Linking placement עבור FastComments במהלך הרישום, המדריכים יכולים להטמיע את שרשור התגובות בתוך כל שדה Rich Text (הוראות מטלה, גוף דף, הנחיות לדיון):

1. פתחו את העורך Rich Text בעמוד היעד.
2. לחצו על סמל ה-**External Tool** (פאזל) בסרגל הכלים.
3. בחרו **FastComments**.
4. קונפיגרו את ההטמעה בדיאלוג ה-deep-linking ולחצו **Insert**.
5. שמרו את הדף.

אם כפתור ה-External Tool לא מופיע בעורך Rich Text, Deep Linking מושבת עבור כלי זה ב-tenant. ראו את הנקודות הבעייתיות להלן.

**4. Visibility and section assignments**

Schoology מגדירה זמינות כלים לפי Sections דרך Course Options:

1. מתוך הקורס לחצו **Course Options** > **External Tools**.
2. עבור כל אפליקציית LTI המותקנת, אתם שולטין האם היא זמינה לכל הסקשנים בקורס או לסקשנים ספציפיים.
3. כדי להגביל את FastComments לסקשנים מסוימים, הסירו את הסימון מהסקשנים שלא אמורים לראות את הכלי.
4. גישה ברמת הסקשן גם מווסתת אילו סקשנים רואים את הפריט **Add Materials** > **External Tool** של FastComments.

**5. What students see**

התלמידים לוחצים על חומר ה-FastComments (או גוללים אל ההטמעה השורה) ונכנסים לדיון השרשורי. SSO הוא אוטומטי דרך ה-LTI launch של Schoology תחת חשבון ה-Schoology שלהם.

מיפוי תפקידים:

- Schoology `Administrator` -> FastComments admin
- Schoology `Instructor` -> FastComments moderator
- Schoology `Student` -> FastComments commenter

**6. Schoology gotchas**

- **Enterprise-only.** חשבונות אישיים וחינמיים של Schoology לא יכולים להתקין כלים ב-LTI 1.3. אם ה-tenant שלכם על גימור החינמי, אופציית **External Tools** חסרה ב-Course Options. שדרגו ל-Schoology Enterprise כדי להשתמש ב-FastComments.
- **Deep Linking disabled by tenant default.** חלק מה-tenants של Schoology מגבילים את מיקום ה-Deep Linking ברמת הארגון. כאשר זה המצב, המדריכים רואים רק את הזרימה **Add Materials** > **External Tool** ולא את כפתור ה-External Tool בעורך Rich Text. כדי לאפשר הטמעה בשורה, מנהל המערכת הולך ל-**System Settings** > **Integration** > **LTI 1.3** > **FastComments** ומאפשר את המיקום **Content Item / Deep Linking**, ואז שומר.
- **Per-section assignment override.** אם FastComments הוקצה ברמת הארגון אך המדריך לא רואה אותו ב-**Add Materials**, הסקשן של הקורס הוסר בהקצאת הארגון. בקשו ממנהל המערכת להוסיף את הסקשן להקצאת אפליקציית FastComments.
- **Material name vs. thread identity.** שינוי שם החומר ב-Schoology אינו מעביר את שרשור התגובות. השרשורים מקושרים על ידי ה-LTI resource link ID, כך ששינוי שם שומר על אותו thread; מחיקה ויצירה מחדש של החומר יוצרת thread חדש וריק.
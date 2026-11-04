[related-parameter-start name = 'simpleSSO'; type = 'FastCommentsSSOSimple'; typeLink = 'https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts#L14' related-parameter-end]

עם Simple SSO, אנו יכולים לספק לוידג'ט ההערות מידע על המשתמש כך שהם לא יצטרכו להזין שם משתמש או דוא"ל כדי להגיב.

אנו יכולים להגדיר Simple SSO כך:

[code-example-start config = {simpleSSO: { username: "Bob", email: "bob@example.com", avatar: "https://example.com/bob.png", websiteUrl: "https://example.com/profiles/bob", displayName: "Bob's Name", displayLabel: "VIP User", loginURL: 'https://example.com/login', logoutURL: 'https://example.com/logout', badgeConfig: { badgeIds: ['badge-id-1', 'badge-id-2'], pageBadgeIds: ['badge-id-3'], override: false } }}; linesToHighlight = [6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19]; title = 'Simple SSO'; code-example-end]

המשתמש ייכנס למערכת, ותיווצר עבורו משתמש SSO ברקע. למשתמש יוגדר `createdFromSimpleSSO` ל-`true` אם הוא נלקח מה-API.

הערות:

- דוא"ל הוא מזהה ייחודי עבור Simple SSO.
- מתן דוא"ל עם Simple SSO אינו נדרש, עם זאת כברירת מחדל ההערות שלהם יופיעו כ"לא מאומתות". <b>אם לא נמסר דוא"ל, המשתמש לא יכול להיות מאומת במלואו.</b>
- **NEW** מאז ינואר 2022: שמות משתמש אינם חייבים להיות ייחודיים בכל fastcomments.com
- Simple SSO יכול ליצור ולעדכן משתמשי SSO באופן אוטומטי, אם נמסר דוא"ל, והמשתמש לא נוצר במקור מ- Secure SSO. אם מטען Simple SSO מציין משתמש שנוצר על ידי Secure SSO או ה-API, משתמש זה נשאר ללא שינוי והמבקר מקבל משתמש נפרד.
- ניתן לציין תגיות (badges) עבור המשתמש באמצעות המאפיין `badgeConfig`. המערך `badgeIds` מכיל את המזהים של תגיות גלובליות שיש לשייך למשתמש. המערך `pageBadgeIds` מכיל מזהי תגיות המוגדרים לעמוד הנוכחי (`urlId`) — תגיות אלו מוצגות רק בעמוד שבו הוקצו. אם `override` מוגדר ל-`true`, הוא יחליף תגיות מוצגות קיימות (גלובליות ומוגדרות לעמוד מוחלפות באופן עצמאי); אם `false` הוא יוסיף לתגיות קיימות.

#### הגדרות Simple SSO

Simple SSO תמיד זמין. שני הגדרות קובעות עד כמה FastComments סומך עליו. ניתן למצוא אותן על ידי חיפוש SSO ב<a href="https://fastcomments.com/auth/my-account/customize-widget" target="_blank">כלל התאמה אישית</a>. שני ההגדרות כבויות כברירת מחדל, והן חלות גם אם "Require SSO To View Comments" מופעל.

חשבונות שכבר השתמשו ב-Simple SSO כאשר נוספו הגדרות אלו, שני ההגדרות הופעלו עבורם, ולכן לא השתנה דבר עבורם.

רק כלל ההתאמה האישית המתאים ביותר לעמוד מתבצע, ולכן אם יש לך יותר מכלל אחד, יש להגדיר זאת בכל אחד מהם. כלל חדש מתחיל עם ההגדרות דולקות אם אחד מהכללים הקיימים שלך מכיל אותן דולקות.

##### אימות אוטומטי של תגובות באמצעות Simple SSO

כאשר אפשרות זו מופעלת, תגובות ממשתמשי Simple SSO מאומתות מייד.

כאשר אפשרות זו כבויה, תגובות ממשתמשי Simple SSO מוצגות כ"לא מאומתות" עד שהמשתמש לוחץ על קישור שאנו שולחים במייל לכתובת שבמטען Simple SSO. אם אתה מאשר אוטומטית רק תגובות מאומתות, הן גם מחכות לקישור זה. אם אין דוא"ל במטען, התגובה נשארת לא מאומתת.

הגדרה זו משפיעה רק על תגובות. הצבעות ממשתמשי Simple SSO תמיד נספרות מייד.

##### אפשר למנהלי Simple SSO

כאשר אפשרות זו מופעלת, משתמש Simple SSO יכול להיות מנהל או מודרטור של החשבון שלך:

- משתמש Simple SSO שיש לו הרשאות מנהל או מודרטור שומר עליהם.
- משתמש Simple SSO עם אותו דוא"ל כאחד מהמנהלים או המודרטורים שלך מקבל את ההרשאות שלהם.

כאשר אפשרות זו כבויה, Simple SSO אינו מעניק הרשאות אלו. מטען Simple SSO שמציין משתמש בעל ההרשאות הללו יוצר משתמש נפרד ללא הרשאות, והמשתמש המקורי נשאר ללא שינוי.

מטעני Simple SSO אינם חתומים, ולכן כאשר אפשרות זו מופעלת, כל מי שיודע את דוא"ל המנהל או מזהה המשתמש יכול לפעול במקומו. אם זה חשוב לאתר שלך, השאר אפשרות זו כבויה והשתמש ב- Secure SSO למנהלים שלך.

משתמשים שנוצרו על ידי Secure SSO או ה-API אינם נגישים באמצעות Simple SSO, בין אם אפשרות זו מופעלת או לא.
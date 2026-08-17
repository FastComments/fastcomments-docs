[related-parameter-start name = 'simpleSSO'; type = 'FastCommentsSSOSimple'; typeLink = 'https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts#L14' related-parameter-end]

עם Simple SSO, אנו יכולים לספק לוידג'ט ההערות מידע על המשתמש כך שהם לא יצטרכו להזין שם משתמש או אימייל כדי להגיב.

אנו יכולים להגדיר Simple SSO כך:

[code-example-start config = {simpleSSO: { username: "Bob", email: "bob@example.com", avatar: "https://example.com/bob.png", websiteUrl: "https://example.com/profiles/bob", displayName: "Bob's Name", displayLabel: "VIP User", loginURL: 'https://example.com/login', logoutURL: 'https://example.com/logout', badgeConfig: { badgeIds: ['badge-id-1', 'badge-id-2'], pageBadgeIds: ['badge-id-3'], override: false } }}; linesToHighlight = [6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19]; title = 'Simple SSO'; code-example-end]

המשתמש ייכנס למערכת, ותיווצר עבורו משתמש SSO ברקע. למשתמש יהיה `createdFromSimpleSSO` מוגדר ל-`true` אם הוא נלקח מה-API.

הערות:

- האימייל הוא מזהה ייחודי עבור Simple SSO.
- מתן אימייל עם Simple SSO אינו נדרש, עם זאת כברירת מחדל ההערות שלהם יופיעו כ"לא מאומת". <b>אם לא סופק אימייל, המשתמש לא יכול להיות מאומת במלואו.</b>
- **חדש** מאז ינואר 2022: שמות משתמש אינם חייבים להיות ייחודיים בכל fastcomments.com
- Simple SSO יכול ליצור ולעדכן משתמשי SSO באופן אוטומטי, אם סופק אימייל, והמשתמש לא נוצר במקור מ- Secure SSO.
- אתה יכול לציין תגיות (`badges`) עבור המשתמש באמצעות המאפיין `badgeConfig`. המערך `badgeIds` מכיל את המזהים של תגיות גלובליות שיש לשייך למשתמש. המערך `pageBadgeIds` מכיל מזהי תגיות המוגדרים לעמוד הנוכחי (`urlId`) — תגיות אלו מוצגות רק בעמוד שבו הן הוקצו. אם `override` מוגדר ל-`true`, הוא יחליף תגיות מוצגות קיימות (גלובליות ותגיות ספציפיות לעמוד מוחלפות באופן עצמאי); אם `false` הוא יוסיף לתגיות הקיימות.
[related-parameter-start name = 'simpleSSO'; type = 'FastCommentsSSOSimple'; typeLink = 'https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts#L14' related-parameter-end]

עם Simple SSO, נוכל לספק לוֹשֶׁק התגובות מידע על המשתמש כדי שלא יצטרך להזין את שם המשתמש או האימייל שלו כדי להגיב.

ניתן להגדיר את Simple SSO כך:

[code-example-start config = {simpleSSO: { username: "Bob", email: "bob@example.com", avatar: "https://example.com/bob.png", websiteUrl: "https://example.com/profiles/bob", displayName: "Bob's Name", displayLabel: "VIP User", loginURL: 'https://example.com/login', logoutURL: 'https://example.com/logout', badgeConfig: { badgeIds: ['badge-id-1', 'badge-id-2'], override: false } }}; linesToHighlight = [6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18]; title = 'Simple SSO'; code-example-end]

המשתמש יהיה מחובר, ויווצר עבורו משתמש SSO מאחורי הקלעים. למשתמש יהיה `createdFromSimpleSSO` עם הערך `true` אם הוא נשלף מה‑API.

Notes: 

- האימייל הוא המזהה הייחודי עבור Simple SSO.
- אין חובה לספק אימייל ב‑Simple SSO, אך כברירת מחדל ההערות שלהם יופיעו כ "לא מאומת". <b>אם לא סופק אימייל, לא ניתן לאמת את המשתמש באופן מלא.</b>
- **חדש** מאז ינואר 2022: שמות משתמש אינם חייבים להיות ייחודיים בכל אתר fastcomments.com
- Simple SSO יכול ליצור ולעדכן משתמשי SSO באופן אוטומטי אם נמסר אימייל, והמשתמש לא נוצר במקור באמצעות Secure SSO.
- ניתן לציין תגיות (badges) למשתמש באמצעות המאפיין `badgeConfig`. מערך `badgeIds` מכיל את מזהי התגים שיש לקשר למשתמש. אם `override` מוגדר כ־`true`, זה יחליף את כל התגים הקיימים שמוצגים בהערות; אם `false`, זה יוסיף על התגים הקיימים.
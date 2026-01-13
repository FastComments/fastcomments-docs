[related-parameter-start name = 'moderationGroupIds'; type = 'Array<string>'; related-parameter-end]

רשימת מזהים שנוצרו מדף [קבוצות פיקוח](https://fastcomments.com/auth/my-account/moderate-comments/moderation-groups).

כאשר מצוין, תגובות שהושארו באמצעות התצורה המצוינת יכילו את אותו מערך של `moderationGroupIds`.

אם ל-`Moderator` יש אחת או יותר [קבוצות פיקוח](https://fastcomments.com/auth/my-account/moderate-comments/moderation-groups) מוגדרות, הם
יראו רק תגובות בדף `Moderate Comments` המשויכות לקבוצה(ות) שלהם.

[code-example-start config = {moderationGroupIds: ['mxZAhjzdb', 'FT19nXbqA']}; linesToHighlight = [6, 7, 8, 9]; title = 'Specify Moderation Groups'; code-example-end]

---
---
[related-parameter-start name = 'collapseReplies'; type = 'boolean'; related-parameter-end]

בברירת מחדל, תגובות לתגובות ברמה העליונה מוצגות.

ניתן להגדיר זאת כך שהמשתמש יצטרך ללחוץ על "Show Replies" בתגובות ברמה העליונה כדי לראות את התגובות המשניות.

[code-example-start config = {collapseReplies: true}; linesToHighlight = [6]; title = 'Collapse Replies to Top Level Comments'; code-example-end]

ניתן להתאים זאת ללא קוד, בעמוד התאמת הווידג'ט:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.collapse-replies'; alt='Collapse replies option in the widget customization UI, hiding child comments behind a Show Replies link'; title='Collapse Replies' app-screenshot-end]

הגדרה זו לא תשפיע על מספר תגובות ברמה העליונה שנטענות בתחילה. אם יש לך תגובה אחת ברמה העליונה, ו‑29 תגובות משניות, עם הגדרה זו פעילה, אתה ת:
- ראה את תגובת הרמה העליונה.
- ראה את "Show Replies" (29) מתחת לתגובה זו.

אם ברצונך להציג את כל תגובות הרמה העליונה בשילוב עם אפשרות זו, הגדר את [דף התחלה ל‑-1](#starting-page).

---
[related-parameter-start name = 'collapseReplies'; type = 'boolean'; related-parameter-end]

כברירת מחדל, תגובות להערות ברמה העליונה מוצגות.

ניתן להגדיר כך שהמשתמש יצטרך ללחוץ "הצג תגובות" בהערות ברמה העליונה כדי לראות את התגובות המשניות.

[code-example-start config = {collapseReplies: true}; linesToHighlight = [6]; title = 'כווץ תגובות להערות ברמה העליונה'; code-example-end]

ניתן להתאים זאת ללא קוד, בדף התאמת הווידג'ט:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.collapse-replies'; title='כווץ תגובות' app-screenshot-end]

הגדרה זו לא תשפיע על מספר ההערות ברמה העליונה שמוטענות בתחילה. אם יש לך הערה אחת ברמה העליונה, ו-29 תגובות משניות, עם הגדרה זו מופעלת תראה:

- תראה את ההערה ברמה העליונה.
- תראה "הצג תגובות (29)" מתחת להערה הזו.

אם ברצונך להציג את כל ההערות ברמה העליונה בשילוב עם אפשרות זו, קבע את [starting page to -1](#starting-page).
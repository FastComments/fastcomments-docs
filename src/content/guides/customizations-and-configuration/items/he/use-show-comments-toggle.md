[related-parameter-start name = 'useShowCommentsToggle'; type = 'boolean'; related-parameter-end]

בברירת מחדל, FastComments יציג את תיבת הקלט של ההערה ואת שרשרת ההערות באותו זמן. כדי לחסוך במרחב אנכי,
הוא גם יסתיר כל שדה נדרש אחר עד שהווידג'ט יזוהה.

עם זאת, ניתן להסתיר את וידג'ט ההערות מאחורי כפתור, לדוגמה:

[app-screenshot-start width=700; url=`https://fastcomments.com/embed?config=%7B%22tenantId%22%3A%22L177BUDVvSe%22%2C%22useShowCommentsToggle%22%3A%22true%22%2C%22urlId%22%3A%22file%253A%252F%252F%252FC%253A%252FUsers%252Fwinrid%252FAppData%252FRoaming%252FJetBrains%252FWebStorm2021.2%252Fscratches%252Fscratch_295.html%22%2C%22url%22%3A%22file%253A%252F%252F%252FC%253A%252FUsers%252Fwinrid%252FAppData%252FRoaming%252FJetBrains%252FWebStorm2021.2%252Fscratches%252Fscratch_295.html%22%2C%22pageTitle%22%3A%22%22%2C%22instanceId%22%3A%220.1281898364813452.1655790389169%22%7D&wId=comment-ui-v2`; selector = '.fast-comments'; delay=2000; alt='וידג\'ט ההערות מצומצם מאחורי כפתור שמציג את ספירת ההערות עד שהקורא לוחץ עליו'; title='לחץ כדי להציג תגובות' app-screenshot-end]

הכפתור משתמש בטקסט מתורגם שונה בהתאם אם ההערות מוצגות כרגע או לא. אם ההערות מוסתרות, הוא משתמש ב-`translations.SHOW_COMMENTS_BUTTON_TEXT`. אם
ההערות מוצגות, הוא משתמש ב-`translations.HIDE_COMMENTS_BUTTON_TEXT`. התרגומים יכולים להכיל את הטקסט `[count]` אשר
יוחלף בספירה המתורגמת.

[code-example-start config = {useShowCommentsToggle: true}; linesToHighlight = [6]; title = 'Click to Show or Hide Comments'; code-example-end]

זה נועד להחליף את תצורת `hideCommentsUnderCountTextFormat`.

הספירה מתעדכנת בזמן אמת עם שרשרת ההערות. הכפתור אינו מוצג אם אין הערות.

זה ניתן להפעיל ללא קוד על ידי יצירת כלל התאמה והפעלת "לחץ כדי להציג תגובות":

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.click-to-show-comments']; selector = '.click-to-show-comments'; alt='תיבת הסימון \'לחץ כדי להציג תגובות\' מסומנת בכלל התאמה בדף התאמת הווידג\'ט'; title='הפעלת לחצן להצגת תגובות' app-screenshot-end]
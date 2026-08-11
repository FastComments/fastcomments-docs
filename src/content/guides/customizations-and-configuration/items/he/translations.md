[related-parameter-start name = 'translations'; type = 'Record<string, string>'; related-parameter-end]

עם FastComments, כל הטקסט בווידג'ט ההערות ניתן להתאמה.

אתה יכול לשנות קטע טקסט יחיד, כמו כפתור השליחה, או את כל הטקסט בווידג'ט ההערות כולו.

בברירת מחדל, הטקסט בווידג'ט ההערות מתורגם בהתאם לשפת המשתמש. עם זאת, ניתן לשנות את הטקסט אם אנחנו בטוחים שהקהל שלנו משתמש באותו אזור/שפה, לדוגמה:

[code-example-start config = {translations: { SUBMIT_REPLY: 'Reply' } }; linesToHighlight = [6, 7, 8]; title = 'טקסט מותאם'; code-example-end]

כל התרגומים הניתנים להתאמה ניתן למצוא <a href="https://fastcomments.com/auth/my-account/get-acct-code#translations" target="_blank">כאן</a> תחת לשונית "אפשרויות מתקדמות".

עם זאת, יש דרך קלה יותר, דרך ממשק ההתאמה של הווידג'ט. שם, אנחנו יכולים פשוט למצוא את הטקסט שמופיע בווידג'ט ההערות במקומות EN_US, ולציין החלפה.

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.show-custom-text-option', '#custom-text-options .dropdown-btn', '.dropdown-items a[data-value="JUST_NOW"]']; selector = '#custom-text-options'; alt='פאנל טקסט מותאם עם מחרוזת וידג\'ט שנבחרה מהתפריט הנפתח ושדה טקסט להחלפה'; title='טקסט מותאם' app-screenshot-end]

כל שינויים בתרגומים משפיעים כעת על כל האזורים.
[related-parameter-start name = 'translations'; type = 'Record<string, string>'; related-parameter-end]

עם FastComments, כל הטקסט בווידג' התגובות ניתן להתאמה אישית.

ניתן להחליף פיסת טקסט אחת, כמו כפתור השליחה, או את כל הטקסט בכל ווידג' התגובות.

ברירת מחדל, הטקסט בווידג' התגובות מתורגם בהתאם ל-locale של המשתמש. עם זאת, נוכל להחליף את הטקסט אם אנו בטוחים שבסיס המשתמשים שלנו משתמש באותו locale/שפה, לדוגמה:

[code-example-start config = {translations: { SUBMIT_REPLY: 'Reply' } }; linesToHighlight = [6, 7, 8]; title = 'Custom Text'; code-example-end]

כל התרגומים שניתנים להתאמה אישית נמצאים <a href="https://fastcomments.com/auth/my-account/get-acct-code#translations" target="_blank">כאן</a> תחת הלשונית "אפשרויות מתקדמות".

עם זאת, יש דרך קלה יותר, דרך ממשק ההתאמה של הווידג'. שם ניתן פשוט למצוא את הטקסט שמוצג בווידג' התגובות ב-locale EN_US, ולציין
החלפה.

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.show-custom-text-option', '#custom-text-options .dropdown-btn', '.dropdown-items a[data-value="JUST_NOW"]']; selector = '#custom-text-options'; title='Custom Text' app-screenshot-end]

כל ההחלפות של התרגומים כרגע חלות על כל ה-locales.
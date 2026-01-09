[related-parameter-start name = 'mentionGroupIds'; type = 'Array<string>'; related-parameter-end]

רשימה של מזהים לשימוש בהשלמה האוטומטית של **@mentions**. שימושי כאשר ברצונך למנוע תיוג משתמשים שאין להם קבוצות חופפות.

כאשר מוגדר, רק משתמשים מקבוצות אחרות יוצעו בהשלמה האוטומטית לאחר הקלדת התו `@`.

[code-example-start config = {mentionGroupIds: ['yxZAhjzda', 'QT19nXbqB']}; linesToHighlight = [6, 7, 8, 9]; title = 'Limit Groups for Mentions'; code-example-end]

---
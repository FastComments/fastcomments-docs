[related-parameter-start name = 'absoluteAndRelativeDates'; type = 'boolean'; related-parameter-end]

בברירת מחדל, נעשה שימוש בתאריכים יחסיים מקומיים. לדוגמה, ליד תגובה שהושארה לאחרונה ייתכן שתראה "לפני 11 דקות".

ייתכן ויהיה צורך או רצון לשמור על פורמט תאריך יחסי זה, אך גם להציג את התאריך המלא לצדיו, ובמקרה זה יש להגדיר פרמטר זה ל‑true. 

[code-example-start config = {absoluteAndRelativeDates: true}; linesToHighlight = [6]; title = 'Use Both Absolute and Relative Dates'; code-example-end]

ניתן להתאים זאת ללא קוד, בדף התאמת הווידג'ט, תחת אפשרויות מתקדמות. תחילה יהיה עליך להפעיל תאריכים מוחלטים כדי לראות אפשרות זו בממשק המשתמש.

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.show-advanced-option', '.absolute-dates', '.relative-and-absolute-dates']; selector = '.relative-and-absolute-dates'; alt='אפשרויות מתקדמות בדף התאמת הווידג\'ט עם תאריכים מוחלטים והגדרת תאריך יחסי משולבת מופעלת'; title='השתמש גם בתאריכים מוחלטים וגם בתאריכים יחסיים' app-screenshot-end]
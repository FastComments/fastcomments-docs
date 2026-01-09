[related-parameter-start name = 'absoluteAndRelativeDates'; type = 'boolean'; related-parameter-end]

בברירת מחדל, נעשה שימוש בתאריכים יחסיים מותאמים לאזור/שפה. לדוגמה, לצד תגובה שנכתבה לאחרונה תוכל לראות "לפני 11 דקות".

ייתכן שיהיה צורך או רצוי לשמור על פורמט התאריך היחסי הזה, אך גם להציג את התאריך המלא לצד זה — במקרה כזה יש להגדיר פרמטר זה כ-true. 

[code-example-start config = {absoluteAndRelativeDates: true}; linesToHighlight = [6]; title = 'Use Both Absolute and Relative Dates'; code-example-end]

ניתן להתאים זאת ללא קוד, בדף התאמת הווידג'ט, תחת אפשרויות מתקדמות. תחילה יהיה עליך להפעיל תאריכים מוחלטים כדי לראות אפשרות זו בממשק המשתמש.

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.show-advanced-option', '.absolute-dates', '.relative-and-absolute-dates']; selector = '.relative-and-absolute-dates'; title='Use Both Absolute and Relative Dates' app-screenshot-end]

---
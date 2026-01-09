[related-parameter-start name = 'countAll'; type = 'boolean'; related-parameter-end]

מספר התגובות המוצג בראש רכיב התגובות יכול להציג או את כל התגובות "ברמת-העל", כלומר אותן תשובות ש-
מגיבות ישירות לעמוד או למאמר עצמו, או שהוא יכול להיות ספירה של **כל** התגובות המקוננות.

בברירת מחדל, זה `true` - זו ספירה של האפשרות השנייה - כל התגובות. בגרסאות ישנות יותר של רכיב התגובות ערך ברירת המחדל הוא `false`.

ניתן לשנות את ההתנהגות, כך שהיא תהיה ספירה של **כל** התגובות המקוננות על ידי קביעת הדגל **countAll** ל-true.

[code-example-start config = {countAll: true}; linesToHighlight = [6]; title = 'Counting All Comments'; code-example-end]

אם נרצה שהספירה תשקף רק את התגובות ברמת-העל, נגדיר את הדגל ל-false.

[code-example-start config = {countAll: false}; linesToHighlight = [6]; title = 'Counting Top Level Comments'; code-example-end]

כרגע לא ניתן להתאים זאת ללא שינויי קוד.
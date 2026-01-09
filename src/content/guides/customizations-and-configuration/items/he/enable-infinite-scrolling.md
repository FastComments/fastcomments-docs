[related-parameter-start name = 'enableInfiniteScrolling'; type = 'boolean'; related-parameter-end]

בברירת המחדל, הווידג'ט של FastComments יסתגל לגובה עצמו כדי להתאים את כל התגובות הנראות. עימוד מתבצע באמצעות כפתור "הצג הבא"
בסוף העמוד הנוכחי, כי מצאנו שזו האינטראקציה שמרגישה הכי נעימה לרוב המשתמשים.

עם זאת, ישנם מקרים שבהם מעדיפים גלילה אינסופית. לדוגמה, אנו משתמשים בתכונה זו במוצר Stream Chat שלנו.

ניתן להסתיר את כפתורי "הצג הבא" ולעבור לגלילה אינסופית על ידי הגדרת הדגל **enableInfiniteScrolling** לtrue:

[code-example-start config = {enableInfiniteScrolling: true}; linesToHighlight = [6]; title = 'Enabling Infinite Scrolling'; code-example-end]

זה גם דורש הוספה של CSS מותאם אישית. הוסף CSS מותאם אישית לבוחר `.comments` כדי לאפשר גלילה, לדוגמה:

[inline-code-attrs-start title = 'הפעלת גלילה אינסופית'; type = 'css'; inline-code-attrs-end]
[inline-code-start]
.comments {
    max-height: 500px;
    overflow-y: auto;
}
[inline-code-end]

דוגמה עובדת מלאה תהיה:

[code-example-start config = {enableInfiniteScrolling: true, customCSS: '.comments { max-height: 500px;  overflow-y: auto; }'}; linesToHighlight = [6, 7]; title = 'Enabling Infinite Scrolling'; code-example-end]

בדוגמה שלעיל אנו משתמשים במאפיין `customCSS`, עם זאת מומלץ להשתמש במקום זאת ב-Widget Configuration UI מסיבות ביצועים. [ראה את התיעוד של CSS מותאם אישית.](/guide-customizations-and-configuration.html#custom-css)

---
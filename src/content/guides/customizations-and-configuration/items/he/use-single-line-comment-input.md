[related-parameter-start name = 'useSingleLineCommentInput'; type = 'boolean'; related-parameter-end]

בברירת מחדל, FastComments מאפשרת למשתמש להכניס תגובה עם כמה שורות שהוא רוצה, עד למגבלת התווים המוגדרת כברירת מחדל.

עם זאת, ייתכן ויהיה רצוי להגביל את המשתמש להזנת שורה אחת בלבד של טקסט. כמה דוגמאות לשימוש כוללות הצעות מקוונות, או צ'אט חי, שבו ניתן להשתמש ב-FastComments.

אנו מפעילים את הדגל **useSingleLineCommentInput** כך:

[code-example-start config = {useSingleLineCommentInput: true}; linesToHighlight = [6]; title = 'הפעלת קלט תגובה בשורה אחת'; code-example-end]

ניתן לבצע זאת גם ללא קוד. בדף התאמה אישית של הווידג'ט, ראו את הסעיף "Enable Single-Line Comment Input".

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.single-line-comment-input'; selector = '.single-line-comment-input'; alt='תיבת הסימון של קלט תגובה בשורה אחת מופעלת בדף התאמה אישית של הווידג\'ט, מגבילה קלט לשורה אחת'; title='הפעלת קלט תגובה בשורה אחת' app-screenshot-end]

שימו לב, שהתגובות בכל דף עבור כל כיוון מיון מחושבות מראש, ולכן לכל כיווני המיון יש את אותה ביצועים.
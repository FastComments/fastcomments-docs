[related-parameter-start name = 'useSingleLineCommentInput'; type = 'boolean'; related-parameter-end]

כברירת מחדל, FastComments יאפשר למשתמש להזין תגובה בכמה שורות שירצה, עד למגבלת התווים ברירת המחדל.

עם זאת, ייתכן שתרצו להגביל את המשתמש להזין רק שורת טקסט אחת. דוגמאות לשימושים כוללות הצעות מחיר מקוונות או צ'אט חי, שניתן להשתמש ב-FastComments
למטרה זו.

אנו מפעילים את הדגל **useSingleLineCommentInput** כך:

[code-example-start config = {useSingleLineCommentInput: true}; linesToHighlight = [6]; title = 'Enable Single-Line Comment Input'; code-example-end]

ניתן גם לבצע זאת ללא קוד. בעמוד התאמת הווידג'ט, ראו את הסעיף 'הפעלת קלט תגובה בשורה יחידה'.

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.single-line-comment-input'; selector = '.single-line-comment-input'; title='Enable Single-Line Comment Input' app-screenshot-end]

שימו לב, התגובות בכל עמוד עבור כל כיוון מיון מחושבות מראש, כך שלכל כיווני המיון הביצועים זהים.
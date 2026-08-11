---
בברירת מחדל, FastComments יציג את שם המשתמש כפי שהוזן, או כפי שהועבר אלינו דרך SSO.

עם זאת, ייתכן ויהיה רצוי להסתיר או להציג את שם המשתמש בצורה שונה. לדוגמה, אם שם המשתמש הוא Allen Rex, אולי
תרצה להציג רק "Allen R.".

זה ניתן לבצע ללא קוד בממשק התאמה אישית של הווידג'ט, תחת ההגדרה שנקראת `Commenter Name Format`:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.commenter-name-format select'; selector = '.commenter-name-format'; alt='תפריט נפתח של פורמט שם המגיב עם אפשרויות כגון Capitalize, Last Initial ו-All Initials'; title='שנה פורמט שם' app-screenshot-end]

הפורמטים הזמינים הם:

- Capitalize (הצגת משתמש לדוגמה כ-Example User)
- Last Initial (הצגת Example User כ-Example U.)
- All Initials (הצגת Example User כ-E. U.)
- הצג "Anonymous"

ההשפעה של שינוי זה מיידית. המשתמשים עדיין יראו את שם המשתמש המלא שלהם בחלק העליון של אזור ההערות, עבור עצמם, אך ההערות שלהם יציגו את שם המשתמש שהשתנה.

שמות המשתמשים מוסתרים בצד השרת כדי להגן על המשתמשים.
---
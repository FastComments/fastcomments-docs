בברירת מחדל, FastComments יציג את שם המשתמש כפי שהזין אותו, או כפי שהועבר אלינו דרך SSO.

עם זאת, ייתכן שתרצו להסוות או להציג את שם המשתמש באופן שונה. לדוגמה, אם שם המשתמש הוא Allen Rex, אולי תרצו להציג רק "Allen R.".

ניתן לעשות זאת ללא קוד בממשק התאמה אישית של הווידג'ט, תחת ההגדרה בשם `Commenter Name Format`:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.commenter-name-format select'; selector = '.commenter-name-format'; title='Change Name Format' app-screenshot-end]

הפורמטים הזמינים הם:

- Capitalize (יציג Example User כ־Example User)
- Last Initial (יציג את Example User כ־Example U.)
- All Initials (יציג את Example User כ־E. U.)
- Show "Anonymous"

השפעת השינוי מיידית. משתמשים עדיין יראו את שם המשתמש המלא בראש אזור התגובות, עבורם, אבל בתגובותיהם יוצג
שם המשתמש המשונה.

שמות המשתמשים מוסווים בצד השרת כדי להגן על המשתמשים.

---
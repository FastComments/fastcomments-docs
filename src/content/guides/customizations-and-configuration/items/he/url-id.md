[related-parameter-start name = 'urlId'; type = 'string'; related-parameter-end]

כאשר מציגים שרשור תגובות, או משאירים תגובה, FastComments צריך לדעת לאיזה עמוד, מאמר או מוצר
שייכות אותן תגובות.

לשם כך אנחנו משתמשים במשהו שאנחנו קוראים לו "URL ID". זה יכול להיות מזהה, כמו מחרוזת או מספר, או כתובת URL.

כברירת מחדל, אם אינך מציין את urlId, הוא יהפוך לכתובת ה-URL של העמוד. ניקח את כתובת ה-URL הנוכחית של העמוד, וננקה אותה כדי להסיר
כל פרמטר שיווקי נפוץ או מזהי מעקב.

במקרים של אינטגרציות צד שלישי, כמו WordPress, בדרך כלל התוסף שלנו ישתמש במזהה שמייצג את המידע הנצפה כרגע כ
URL ID, למשל מזהה המאמר/העמוד.

[code-example-start config = {urlId: 'https://example.com/page'}; linesToHighlight = [6]; title = 'Defining a Custom URL ID'; code-example-end]

אחת מהנקודות שנזכיר לעתים קרובות במסמך זה היא ה- <a href="https://fastcomments.com/auth/my-account/customize-widget/new">Widget Customization UI</a>.

ניתן להשתמש בממשק זה כדי לבצע שינויים רבים בווידג'ט התגובות ללא שימוש בקוד.

כאשר יוצרים כלל התאמה אישית, לעתים נרצה שהוא יחול על כל העמודים באתר שלנו. עם זאת, במקרים מסוימים נרצה להתאים את ווידג'ט התגובות
לעמוד מסוים, בין אם כדי להחיל עיצוב מותאם אישית, או אולי להפוך את התגובות בעמוד זה לאנונימיות. אפשר גם, לדוגמה, להציג תגובות חיים
מיד בחלק מהעמודים, בזמן שבל אחרים להסתיר אותן מתחת לכפתורי התראה.

כל זה אפשרי דרך שדה קלט ה-URL ID בעמוד זה, שנראה כך:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.url-id'; title='URL ID Input in The Widget Customization Page' app-screenshot-end]

הערך בשדה זה צריך להתאים לפרמטר *urlId* המועבר לווידג'ט התגובות. אם ברצונך שהכלל שלך לא יהיה תלוי ב-*urlId*, השאר שדה זה ריק או הזן *.

נכון ל-2023 שדה ה-`URL ID` בהתאמה האישית של הווידג'ט גם תומך בתבניות! לדוגמה ייתכן שיהיו לך
`*/blog/*` כדי להוסיף עיצוב ייחודי לבלוג שלך ו-`*/store/*` כדי שיהיה עיצוב ייחודי לחנות שלך,
וכל זאת תוך שימוש באותו דומיין.

### נקודות חשובות

1. If your page has hash parameters (like example.com#page-1) - this will become part of the URL ID, by default.
2. During migrations, for example from WordPress to Gatsby, you may have to migrate the URL ID comment values after the initial migration. For that, reach out to us.
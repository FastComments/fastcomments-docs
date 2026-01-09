[related-parameter-start name = 'enableCommenterLinks'; type = 'boolean'; related-parameter-end]

ברירת המחדל, FastComments יבקש מהמשתמש רק את תגובתו, שם המשתמש שלו, ואת האימייל שלו.

עם זאת, במצבים מסוימים ייתכן שתרצה שהמשתמש ישאיר קישור לבלוג או לאתר שלו.

אנחנו יכולים לאפשר הצגת שדה קלט נוסף להשארת כתובת האתר של המשתמש על ידי הגדרת הדגל **enableCommenterLinks** ל-true:

[code-example-start config = {enableCommenterLinks: true}; linesToHighlight = [6]; title = 'Enabling Commenter Links'; code-example-end]

כאשר כתובת URL זו מסופקת, חשבון המשתמש יעודכן ושם המשתמש שלו בכל התגובות הקודמות והעתידיות יקשר לכתובת URL זו.

ניתן להתאים זאת ללא קוד, בדף התאמה אישית של הווידג'ט:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.click-to-show-comments', '.commenter-links']; selector = '.commenter-links'; title='Enabling Commenter Links' app-screenshot-end]
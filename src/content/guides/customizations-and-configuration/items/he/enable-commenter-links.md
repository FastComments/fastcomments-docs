[related-parameter-start name = 'enableCommenterLinks'; type = 'boolean'; related-parameter-end]

בברירת מחדל, FastComments יבקש מהמשתמש רק את ההערה שלו, שם המשתמש שלו, ואת האימייל שלו.

עם זאת, במצבים מסוימים ייתכן שתרצו שהמשתמש יוסיף קישור לבלוג או לאתר האינטרנט שלו.

אנו יכולים לאפשר הצגת שדה קלט נוסף כדי להוסיף את כתובת האתר של המשתמש על ידי הגדרת הדגל **enableCommenterLinks** ל‑true:

[code-example-start config = {enableCommenterLinks: true}; linesToHighlight = [6]; title = 'הפעלת קישורי מגיב'; code-example-end]

כאשר כתובת ה‑URL הזו מסופקת, חשבון המשתמש יעודכן וכל שם המשתמש שלו בכל ההערות הקודמות והעתידיות יקשר לכתובת זו.

זה ניתן להתאמה ללא קוד, בדף התאמת הווידג'ט:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.click-to-show-comments', '.commenter-links']; selector = '.commenter-links'; alt='דף התאמת הווידג\'ט עם תיבת הסימון של קישורי המגיב מסומנת כדי להוסיף שדה כתובת אתר של משתמש לטופס ההערה'; title='הפעלת קישורי מגיב' app-screenshot-end]
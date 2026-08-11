[related-parameter-start name = 'showBadgesInTopBar'; type = 'boolean'; related-parameter-end]

בברירת מחדל, FastComments יציג תגיות משתמש רק על ההערות שלהם בתוך שרשרת ההערות.

עם זאת, אנו יכולים להציג תגיות משתמש ליד שמם מעל טופס ההערה על ידי הפעלת תכונה זו בעמוד התאמת הווידג'ט:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.show-badges-in-top-bar'; alt='הצגת תגיות בתיבת הסימון של סרגל העליון בעמוד התאמת הווידג'ט, מציב תגיות ליד השם מעל טופס ההערה'; title='אפשרות הצגת תגיות בסרגל העליון' app-screenshot-end]

זה יציג את תגיות המשתמש לצד שמו באזור סרגל העליון, מה שהופך את הישגיו ומצבו לבולטים יותר כאשר הוא כותב תגובה.

שימו לב שהפונקציה הזו חייבת להיות מופעלת בממשק התאמת הווידג'ט כדי לעבוד. ניתן באופן אופציונלי להגדיר את הדגל **showBadgesInTopBar** ל‑false בתצורת הקוד שלכם כדי להשבית אותו באופן סלקטיבי גם כאשר הוא מופעל ברמת השרת:

[code-example-start config = {showBadgesInTopBar: false}; linesToHighlight = [6]; title = 'Disable Show Badges in Top Bar'; code-example-end]
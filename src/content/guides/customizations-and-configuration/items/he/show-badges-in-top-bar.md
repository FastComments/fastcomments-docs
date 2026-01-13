[related-parameter-start name = 'showBadgesInTopBar'; type = 'boolean'; related-parameter-end]

כברירת מחדל, FastComments יציג תגי משתמש רק על ההערות שלהם בתוך שרשור ההערות.

עם זאת, ניתן להציג את תגי המשתמש לצד שמם מעל טופס ההערה על‑ידי הפעלת תכונה זו בדף התאמה אישית של הווידג'ט:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.show-badges-in-top-bar'; title='אפשרות הצגת תגי משתמש בסרגל העליון' app-screenshot-end]

זה יציג את תגי המשתמש לצדן של שמם באזור הסרגל העליון, מה שיבליט את ההישגים והסטטוס שלהם בעת כתיבת תגובה.

שים לב שתכונה זו חייבת להיות מופעלת בממשק התאמת הווידג'ט כדי שתעבוד. ניתן, באופן אופציונלי, להגדיר את הדגל **showBadgesInTopBar** כ-false בקונפיגורציית הקוד שלך כדי להשבית אותה באופן סלקטיבי גם כשהיא מופעלת ברמת השרת:

[code-example-start config = {showBadgesInTopBar: false}; linesToHighlight = [6]; title = 'השבתת הצגת תגי משתמש בסרגל העליון'; code-example-end]
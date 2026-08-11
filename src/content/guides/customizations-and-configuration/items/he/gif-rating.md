[related-parameter-start name = 'gifRating'; type = 'string'; related-parameter-end]

בברירת מחדל, וידג'ט ההערות של FastComments יגדיר `gif rating` של `pg`.

האפשרויות הזמינות הן `g`, `pg`, `pg-13`, ו-`r`.

ניתן להגדיר זאת בקוד או דרך הממשק. בקוד אפשר לעשות זאת כך:

[code-example-start config = {gifRating: 'pg-13'}; linesToHighlight = [6]; title = 'Set Gif Rating'; code-example-end]

בממשק, תמצאו זאת תחת `Gif Picker Rating` כל עוד `Disable Image Uploads?` אינו מסומן.

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.gif-rating'; alt='תפריט נפתח של Gif Picker Rating בדף התאמת הווידג\'ט המציע g, pg, pg-13 ו-r'; title='הגדרת דירוג ה‑Gif' app-screenshot-end]
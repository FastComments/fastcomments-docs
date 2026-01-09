---
[related-parameter-start name = 'gifRating'; type = 'string'; related-parameter-end]

כברירת מחדל, ווידג' התגובות של FastComments יקבע `gif rating` של `pg`.

האופציות הזמינות הן `g`, `pg`, `pg-13` ו־`r`.

ניתן להגדיר זאת בקוד או דרך ממשק המשתמש. בקוד ניתן לעשות זאת כך:

[code-example-start config = {gifRating: 'pg-13'}; linesToHighlight = [6]; title = 'Set Gif Rating'; code-example-end]

בממשק המשתמש, תמצאו זאת תחת `Gif Picker Rating` כל עוד האפשרות `Disable Image Uploads?` אינה מסומנת.

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.gif-rating'; title='Setting The Gif Rating' app-screenshot-end]

---
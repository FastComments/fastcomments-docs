[related-parameter-start name = 'enableSpoilers'; type = 'boolean'; related-parameter-end]

ניתן להפעיל תמיכה בספוילרים על ידי הגדרת הדגל **enableSpoilers** ל-true:

[code-example-start config = {enableSpoilers: true}; linesToHighlight = [6]; title = 'Enabling Spoilers'; code-example-end]

ניתן לבצע זאת גם ללא קוד. בדף התאמת הווידג'ט, בחרו באפשרות "הפעלת ספוילרים".

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.enable-spoilers']; selector = '.enable-spoilers'; title='Enable Spoilers' app-screenshot-end]

כאשר טקסט מסומן, וכפתור `SPOILER` שמופיע כעת נלחץ, הטקסט יוסתר עד שהמשתמש יעביר את הסמן מעליו. במצב כהה אנחנו עושים את אותו הדבר, עם צבעים שונים
שמתאימים יותר למצב כהה.

זה גם תואם לעורך WYSIWYG.
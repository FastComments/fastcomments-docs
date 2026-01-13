---
[related-parameter-start name = 'maxCommentCharacterLength'; type = 'number'; related-parameter-end]

ניתן להגביל את מספר התווים המקסימלי שיכולים להיכנס בשדה הקלט של התגובה באמצעות הפרמטר **maxCommentCharacterLength**.

הברירת מחדל היא 2000.

פריטים כגון כתובות URL של תמונות אינם נכללים בחישוב האורך.

[code-example-start config = {maxCommentCharacterLength: 500}; linesToHighlight = [6]; title = 'Limit Comment Length'; code-example-end]

ניתן להתאים זאת ללא קוד, בדף התאמה אישית של הווידג'ט:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.max-comment-size'; title='Limit Comment Length' app-screenshot-end]

---
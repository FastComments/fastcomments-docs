[related-parameter-start name = 'maxCommentCharacterLength'; type = 'number'; related-parameter-end]

המספר המרבי של תווים המותר להכניס בשדה הקלט של התגובה ניתן להגבלה באמצעות הפרמטר **maxCommentCharacterLength**.

ברירת המחדל היא 2000.

פריטים כגון כתובות URL של תמונות אינם נכללים בחישוב האורך.

[code-example-start config = {maxCommentCharacterLength: 500}; linesToHighlight = [6]; title = 'הגבלת אורך תגובה'; code-example-end]

ניתן להתאים זאת ללא קוד, בדף התאמה אישית של הווידג'ט:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.max-comment-size'; alt='שדה גודל תגובה מקסימלי בדף התאמה אישית של הווידג\'ט, משמש להגבלת מספר התווים שהתגובה יכולה להכיל'; title='הגבלת אורך תגובה' app-screenshot-end]

---
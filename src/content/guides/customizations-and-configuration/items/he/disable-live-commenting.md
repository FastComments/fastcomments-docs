[related-parameter-start name = 'disableLiveCommenting'; type = 'boolean'; related-parameter-end]

בברירת מחדל, FastComments יפעיל תגובות בזמן אמת.

זה אומר שכל צופה בשרשרת ההערות צריך לראות את אותו תוכן.

לדוגמה, אם מתווספת תגובה, אותה תגובה תוצג. אם תגובה נערכת או מוסרת,
אז אותן תגובות ייערכו או יוסרו עבור כל הצופים בשרשרת. אותו דבר לגבי הצבעות וכל פעולות המודרציה.

עם זאת, ניתן להשבית זאת:

[code-example-start config = {disableLiveCommenting: true}; linesToHighlight = [6]; title = 'Disable Live Commenting'; code-example-end]

זה גם ניתן לבצע ללא קוד. בעמוד התאמת הווידג'ט, ראו את המקטע "Disable Live Commenting".

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.disable-live-commenting']; selector = '.disable-live-commenting'; alt='מקטע השבתת תגובות בזמן אמת בעמוד התאמת הווידג\'ט, מכבה עדכונים בזמן אמת של השרשרת'; title='השבתת תגובות בזמן אמת' app-screenshot-end]

---
---
[related-parameter-start name = 'disableLiveCommenting'; type = 'boolean'; related-parameter-end]

ברירת המחדל של FastComments היא שהתגובות החיות מופעלות.

זה אומר שכל צופה בחוט התגובות יראה את אותו תוכן.

לדוגמה, אם תתווסף תגובה, תגובה זו תוצג. אם תגובה תעודכן או תוסר,
אז אותן תגובות יעודכנו או יוסרו עבור כל הצופים בחוט. אותו הדבר לגבי הצבעות וכל פעולות המודרציה.

עם זאת, ניתן להשבית זאת:

[code-example-start config = {disableLiveCommenting: true}; linesToHighlight = [6]; title = 'Disable Live Commenting'; code-example-end]

ניתן גם לעשות זאת ללא קוד. בדף התאמה אישית של הווידג'ט, ראה את הסעיף "השבתת תגובות חיות".

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.disable-live-commenting']; selector = '.disable-live-commenting'; title='Disable Live Commenting' app-screenshot-end]

---
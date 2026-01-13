[related-parameter-start name = 'enableViewCounts'; type = 'boolean'; related-parameter-end]

ברירת המחדל של FastComments אינה עוקבת מי צפה בכל תגובה ואינה מספקת סטטיסטיקות על כך.

עם זאת, ניתן להפעיל תכונה זו, ואז המערכת תתחיל לעקוב כאשר כל משתמש גולל אל תגובה.

כאשר זה קורה, הספירה שלצד סמל עין שמוצג על כל תגובה תוגדל. הספירה מתעדכנת בזמן אמת ומוצגת בקיצור בהתאם לאזור המועדף של המשתמש.

ניתן להפעיל זאת על‑ידי הגדרת הדגל **enableViewCounts** כ‑true:

[code-example-start config = {enableViewCounts: true}; linesToHighlight = [6]; title = 'Enabling Comment View Counts'; code-example-end]

ניתן להתאים זאת ללא קוד, בדף ההתאמה אישית של הווידג'ט:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.enable-view-counts']; selector = '.enable-view-counts'; title='Enabling Comment View Counts' app-screenshot-end]

אנחנו עוקבים אחרי מזהה המשתמש* שצפה בתגובה, כדי שאם תצפה בתגובה שוב זה לא יגדיל את הספירה. אם תצפה בתגובה שוב
לאחר שתי שנים, הספירה תגדל שוב.

- *הערה: או מזהה מושב אנונימי, או כתובת ה‑IP של המשתמש כערך מוצפן.
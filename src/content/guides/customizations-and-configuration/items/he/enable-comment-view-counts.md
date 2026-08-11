[related-parameter-start name = 'enableViewCounts'; type = 'boolean'; related-parameter-end]

בברירת מחדל, FastComments אינו עוקב אחרי מי צפה בכל תגובה ולא מספק שום סטטיסטיקה בנושא זה.

עם זאת, אנו יכולים להפעיל תכונה זו, ואז המערכת תתחיל לעקוב כאשר כל משתמש גולל לתגובה.

כאשר זה קורה, ספירה לצד סמל העין שמופיע על כל תגובה תגדל. הספירה מתעדכנת בזמן אמת ומקוצרת בהתאם לשפת המשתמש.

אנו יכולים להפעיל זאת על ידי הגדרת הדגל **enableViewCounts** ל‑true:

[code-example-start config = {enableViewCounts: true}; linesToHighlight = [6]; title = 'הפעלת ספירת צפיות בתגובות'; code-example-end]

ניתן להתאים זאת ללא קוד, בדף התאמת הווידג'ט:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.enable-view-counts']; selector = '.enable-view-counts'; alt='דף התאמת הווידג''ט עם תיבת הסימון של ספירת הצפיות מסומנת כך שכל תגובה מציגה סמל עין וספירה'; title='הפעלת ספירת צפיות בתגובות' app-screenshot-end]

אנו עוקבים אחרי מזהה המשתמש* שצפה בתגובה, כך שאם אתה **צופה** בתגובה שוב היא לא תגדל. אם אתה **צופה** בתגובה שוב אחרי שנתיים, הספירה תגדל יותר.

- *הערה: או מזהה הסשן האנונימי, או כתובת ה‑IP של המשתמש כערך מוצפן.
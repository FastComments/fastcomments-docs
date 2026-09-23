[related-parameter-start name = 'enableViewCounts'; type = 'boolean'; related-parameter-end]

בברירת מחדל, FastComments אינו עוקב אחרי מי צפה בכל תגובה ולא מספק שום סטטיסטיקה בנושא זה.

עם זאת, אנו יכולים להפעיל תכונה זו, ואז המערכת תתחיל לעקוב כאשר כל משתמש גולל לתגובה.

כאשר זה קורה, ספירה לצד סמל העין המופיע בכל תגובה תגדל. הספירה מתעדכנת בזמן אמת ומקוצרת בהתאם לשפת המשתמש.

אנו יכולים להפעיל זאת על ידי הגדרת הדגל **enableViewCounts** ל‑true:

[code-example-start config = {enableViewCounts: true}; linesToHighlight = [6]; title = 'Enabling Comment View Counts'; code-example-end]

זה ניתן להתאמה ללא קוד, בעמוד התאמת הווידג'ט:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.enable-view-counts']; selector = '.enable-view-counts'; alt='דף התאמת הווידג\'ט עם תיבת הסימון של ספירות הצפייה מסומנת כך שכל תגובה מציגה סמל עין וספירה'; title='הפעלת ספירות צפייה בתגובות' app-screenshot-end]

אנו עוקבים אחרי מזהה המשתמש* שצפה בתגובה במשך שבוע, כך שאם תצפה בתגובה שוב בתוך השבוע זה לא יגדיל. אם תצפה בתגובה שוב לאחר שהשבוע עבר, הספירה תגדל שוב.

- *הערה: או מזהה הסשן האנונימי, או כתובת ה‑IP של המשתמש כערך מוצפן.
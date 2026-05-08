[related-parameter-start name = 'usersListLocation'; type = 'number'; related-parameter-end]
[related-parameter-start name = 'usersListIncludeOffline'; type = 'boolean'; related-parameter-end]

בברירת מחדל, FastComments אינו מציג רשימת משתמשים בעמוד.

ניתן להציג רשימה של אנשים הצופים כעת בעמוד, לצד ווידג'ט התגובות. הרשימה מתעדכנת בזמן אמת כשהמשתמשים מצטרפים ועוזבים, ומציגה את שמם, תמונת הפרופיל שלהם ואינדיקטור מקוון.

יש שלוש אפשרויות פריסה:

- `1` - עליון: שורת תמונות פרופיל אופקית החופפות זו לזו המוצגת מעל התגובות.
- `2` - שמאל: סרגל צד עם שמות ונקודות מקוונות המוצג משמאל לווידג'ט.
- `3` - ימין: אותו סרגל צד המוצג מימין לווידג'ט.

הגדר את הדגל **usersListLocation** כדי להפעיל את התכונה:

[code-example-start config = {usersListLocation: 3}; linesToHighlight = [6]; title = 'הצג רשימת משתמשים מימין'; code-example-end]

כברירת מחדל הרשימה מציגה רק משתמשים שמחוברים כרגע. כדי לכלול גם אנשים שהגיבו בעמוד בעבר (אך אינם צופים בו כרגע), הגדר את **usersListIncludeOffline** ל-true:

[code-example-start config = {usersListLocation: 3, usersListIncludeOffline: true}; linesToHighlight = [6, 7]; title = 'כלול מגיבים קודמים'; code-example-end]

מגיבים קודמים מוצגים ללא הנקודה הירוקה שמציינת מקוון, כדי שיהיה ברור מי נוכח כרגע.

משתמשים בעלי פרופיל פרטי מוצגים עם תמונת פרופיל כללית ותווית "פרופיל פרטי" כך שהמונה נשאר מדויק מבלי לחשוף זהויות.

ניתן גם להגדיר זאת ללא קוד. בדף ההתאמה של הווידג'ט, ראו את האפשרות "מיקום רשימת המשתמשים". כשהמיקום מוגדר לערך שאינו Off, מופיעה מתחתיה תיבת סימון "כלול מגיבים מהעבר".

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.users-list-settings'; title='הגדרות רשימת משתמשים'; actions=[{type: 'set-value', selector: '#users-list-location-input', value: '3'}] app-screenshot-end]

מעבר ל-500 משתמשים חיים, הרשימה עלולה להתעדכן באיחור של עד 30 שניות.
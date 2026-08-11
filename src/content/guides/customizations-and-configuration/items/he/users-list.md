[related-parameter-start name = 'usersListLocation'; type = 'number'; related-parameter-end]
[related-parameter-start name = 'usersListIncludeOffline'; type = 'boolean'; related-parameter-end]

בברירת מחדל, FastComments אינו מציג רשימת משתמשים בעמוד.

אתה יכול להציג רשימה של אנשים שמצפים כרגע בעמוד, לצד וידג'ט ההערות. הרשימה מתעדכנת בזמן אמת כאשר משתמשים מצטרפים ועוזבים, ומציגה את שמם, תמונת הפרופיל שלהם, וסמן מצב מקוון.

יש שלוש אפשרויות פריסה:

- `1` - למעלה: שורה אופקית של אווטארים חופפים המוצגים מעל ההערות.
- `2` - שמאל: סרגל צד עם שמות ונקודות מקוונות המוצגים משמאל לוידג'ט.
- `3` - ימין: אותו סרגל צד המוצג מימין לוידג'ט.

הגדר את הדגל **usersListLocation** כדי להפעיל את הפונקציה:

[code-example-start config = {usersListLocation: 3}; linesToHighlight = [6]; title = 'הצג רשימת משתמשים מימין'; code-example-end]

בברירת מחדל, הרשימה מציגה רק משתמשים שמקוונים כרגע. כדי לכלול גם אנשים שהגיבו בעמוד בעבר (אך אינם מצפים בו כרגע), הגדר את **usersListIncludeOffline** ל‑true:

[code-example-start config = {usersListLocation: 3, usersListIncludeOffline: true}; linesToHighlight = [6, 7]; title = 'כלול מגיבים קודמים'; code-example-end]

מגיבים קודמים מוצגים ללא הנקודה הירוקה של מקוון, כך שניתן לראות מי נוכח כרגע.

משתמשים עם פרופילים פרטיים מוצגים עם אווטאר גנרי ותווית "פרופיל פרטי" כך שהספירה נשארת מדויקת מבלי לחשוף זהויות.

זה ניתן גם להגדיר ללא קוד. בעמוד התאמה אישית של הווידג'ט, ראה את האפשרות "מיקום רשימת משתמשים". כאשר המיקום מוגדר לכל ערך שאינו Off, תיבת הסימון "כלול מגיבים קודמים" מופיעה מתחתיו.

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.users-list-settings'; alt='מיקום רשימת משתמשים מוגדר לימין, עם תיבת הסימון לכלול מגיבים קודמים מוצגת מתחתיו'; title='הגדרות רשימת משתמשים'; actions=[{type: 'set-value', selector: '#users-list-location-input', value: '3'}] app-screenshot-end]

לאחר 500 משתמשים חיים, הרשימה יכולה להיות עד 30 שניות לא מעודכנת.
[related-parameter-start name = 'usersListLocation'; type = 'number'; related-parameter-end]
[related-parameter-start name = 'usersListIncludeOffline'; type = 'boolean'; related-parameter-end]

כברירת מחדל, FastComments אינו מציג רשימת משתמשים בעמוד.

ניתן להציג רשימה של אנשים הצופים כעת בעמוד, לצד ווידג'ט התגובות. הרשימה מתעדכנת בשידור חי כאשר משתמשים מצטרפים ועוזבים, ומציגה את שמם, האווטאר שלהם ומצב חיבור מקוון.

יש שלוש אפשרויות פריסה:

- `1` - למעלה: שורה אופקית של אווטארים החופפים שמוצגים מעל התגובות.
- `2` - משמאל: סרגל צדדי עם שמות ונקודות מצב מקוון שמוצג משמאל לווידג'ט.
- `3` - מימין: אותו סרגל צדדי שמוצג מימין לווידג'ט.

הגדר את הדגל **usersListLocation** כדי להפעיל את התכונה:

[code-example-start config = {usersListLocation: 3}; linesToHighlight = [6]; title = 'Show Users List on the Right'; code-example-end]

כברירת מחדל הרשימה מציגה רק משתמשים שמחוברים כעת. כדי לכלול גם אנשים שהגיבו בעמוד בעבר (אך אינם צופים בו כרגע), הגדר את **usersListIncludeOffline** ל-true:

[code-example-start config = {usersListLocation: 3, usersListIncludeOffline: true}; linesToHighlight = [6, 7]; title = 'Include Past Commenters'; code-example-end]

הגולשים שהגיבו בעבר מוצגים ללא הנקודה הירוקה שמציינת 'מחובר', כך ברור מי נוכח כרגע.

משתמשים עם פרופילים פרטיים מוצגים עם אווטאר כללי ותווית "פרופיל פרטי" כדי שמספר המשתמשים יישאר מדויק מבלי לחשוף זהויות.

ניתן גם להגדיר זאת ללא קוד. בדף התאמה אישית של הווידג'ט, ראה את אפשרות "מיקום רשימת המשתמשים". כאשר המיקום מוגדר לכל ערך שאינו 'כבוי', תופיע מתחתיה תיבת סימון "כלול מגיבים קודמים".

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.users-list-settings'; title='Users List Settings'; actions=[{type: 'set-value', selector: '#users-list-location-input', value: '3'}] app-screenshot-end]

---
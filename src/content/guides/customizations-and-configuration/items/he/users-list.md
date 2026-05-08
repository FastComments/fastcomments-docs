[related-parameter-start name = 'usersListLocation'; type = 'number'; related-parameter-end]
[related-parameter-start name = 'usersListIncludeOffline'; type = 'boolean'; related-parameter-end]

בברירת מחדל, FastComments לא מציג רשימת משתמשים בעמוד.

ניתן להציג רשימה של אנשים שכרגע צופים בעמוד, לצד הווידג'ט של התגובות. הרשימה מתעדכנת בזמן אמת כאשר משתמשים מצטרפים ועוזבים, ומציגה את שמם, את תמונת הפרופיל שלהם ואת הסמן שמציין שהם באינטרנט.

There are three layout options:

- `1` - Top: שורה אופקית של אווטארים חופפים שמוצגת מעל התגובות.
- `2` - Left: סרגל צד עם שמות ונקודות מקוונות שמוצג משמאל לווידג'ט.
- `3` - Right: אותו סרגל צד שמוצג מימין לווידג'ט.

Set the **usersListLocation** flag to enable the feature:

[code-example-start config = {usersListLocation: 3}; linesToHighlight = [6]; title = 'Show Users List on the Right'; code-example-end]

בברירת מחדל הרשימה מציגה רק משתמשים שמחוברים כעת. כדי לכלול גם אנשים שהגיבו בעמוד בעבר (אך אינם צופים בו כרגע), הגדר את **usersListIncludeOffline** ל-true:

[code-example-start config = {usersListLocation: 3, usersListIncludeOffline: true}; linesToHighlight = [6, 7]; title = 'Include Past Commenters'; code-example-end]

מגיבים מהעבר מוצגים ללא הנקודה הירוקה שמציינת שהם באינטרנט, כדי שיהיה ברור מי נוכח כרגע.

משתמשים עם פרופילים פרטיים יוצגו עם תמונת פרופיל כללית ותווית "פרופיל פרטי" כך שהמספר יישאר מדויק מבלי לחשוף זהויות.

ניתן גם להגדיר זאת ללא קוד. בדף ההתאמה האישית של הווידג'ט, ראה את האפשרות "מיקום רשימת המשתמשים":

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.users-list-location'; title='Users List Location' app-screenshot-end]

כאשר המיקום מוגדר לכל ערך שהוא חוץ מ‑Off, תיבת הסימון "כלול מגיבים קודמים" מוצגת מתחתיו:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.users-list-include-offline'; title='Include Past Commenters' app-screenshot-end]
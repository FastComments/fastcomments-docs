[related-parameter-start name = 'locale'; type = 'string'; related-parameter-end]

בברירת מחדל, FastComments יציג את וידג'ט ההערות במיקום (locale) שנקבע על ידי מערכת והדפדפן של המשתמש.

כאשר משתמש מגיב או נכנס למערכת, אנו מעדכנים את ה‑locale האחרון שבו השתמש ומשתמשים בו גם לשליחת אימיילים.

זה משפיע על האופן שבו וידג'ט ההערות מתורגם למשתמש. locale מורכב משפת המשתמש והאזור שלו, ולכן הגדרת locale בדרך כלל תשנה את השפה שבה מוצג הטקסט למשתמש.

#### דרך הממשק

זה ניתן להגדיר באמצעות ממשק התאמת הווידג'ט. ראה את האפשרות "Locale / Language":

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.locale-override'; alt='תפריט נפתח Locale / Language בעמוד התאמת הווידג\'ט המשמש לשינוי ה‑locale שזוהה עבור המבקר'; title='Changing The Locale / Language' app-screenshot-end]

#### דרך קוד

זה ניתן לעקוף עם locale רצוי.

[code-example-start config = {locale: 'ru_ru'}; linesToHighlight = [6]; title = 'Manually Defining the User\'s Locale'; code-example-end]

### שפות נתמכות וקודי Locale

[אתה יכול למצוא את הרשימה המלאה של השפות הנתמכות וקודי ה‑locale המתאימים כאן.](/guide-supported-languages.html#supported-languages)

### הערה על SSO

אם אתה משתמש ב‑SSO, ייתכן שתרצה להעביר את ה‑locale של המשתמש באובייקט המשתמש, כך שהאימיילים ודברים אחרים יתורגמו כראוי עבורו.
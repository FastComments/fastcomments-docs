[related-parameter-start name = 'locale'; type = 'string'; related-parameter-end]

כברירת מחדל, FastComments יציג את ווידג'ט התגובות ב‑locale שנקבע על-ידי מערכת המשתמש והדפדפן.

כאשר משתמש מגיב או נכנס, אנו מעדכנים את ה‑locale האחרון שבו השתמש ומשתמשים בו גם לשליחת מיילים.

זה משפיע על האופן שבו ווידג'ט התגובות מתורגם עבור המשתמש. ה‑locale מורכב משפת המשתמש והאזור שלו, ולכן הגדרת ה‑locale בדרך כלל תשנה את השפה שבה מוצג הטקסט למשתמש.

#### Via The UI

ניתן להגדיר זאת דרך ממשק התאמת הווידג'ט. ראו את האפשרות "Locale / Language":

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.locale-override'; title='Changing The Locale / Language' app-screenshot-end]

#### Via Code

ניתן לעקוף זאת ולהגדיר locale רצוי.

[code-example-start config = {locale: 'ru_ru'}; linesToHighlight = [6]; title = 'Manually Defining the User\'s Locale'; code-example-end]

### Supported Languages and Locale Codes

[ניתן למצוא את הרשימה המלאה של השפות הנתמכות וקודי ה‑locale המתאימים כאן.](/guide-supported-languages.html#supported-languages)

### SSO Note

אם אתם משתמשים ב‑SSO, ייתכן שתרצו להעביר את ה‑locale של המשתמש באובייקט המשתמש, כדי שמיילים ודברים אחרים יהיו מותאמים נכון עבורו.
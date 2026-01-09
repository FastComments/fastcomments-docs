[related-parameter-start name = 'disableBlocking'; type = 'boolean'; related-parameter-end]

ברירת המחדל, FastComments מאפשרת למשתמשים לחסום משתמשים אחרים. חסימת משתמש תגרום להסתרת תגובותיו
ותמנע שליחת התראות בין המשתמשים, וכן הלאה.

ייתכן ויהיה רצוי להשבית פונקציונליות זו. ניתן לעשות זאת כך:

[code-example-start config = {disableBlocking: true}; linesToHighlight = [6]; title = 'Disable Blocking'; code-example-end]

ניתן גם לבצע זאת ללא קוד, מה שמאפשר גם אימות צד-שרת תקין, דרך ממשק התאמה אישית של הווידג'ט:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.disable-blocking']; selector = '.disable-blocking'; title='Disable Blocking' app-screenshot-end]
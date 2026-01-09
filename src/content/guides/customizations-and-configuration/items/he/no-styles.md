[related-parameter-start name = 'noStyles'; type = 'boolean'; related-parameter-end]

בפרויקטים גדולים יותר של עיצוב מותאם אישית, ייתכן שיהיה רצוי להתחיל מדף נקי ולא להשתמש כלל בעיצוב המוגדר כברירת מחדל.

ניתן להסיר את כל עיצוב ברירת המחדל על ידי הגדרת הפרמטר **noStyles** ל־true, באופן הבא:

[code-example-start config = {noStyles: true}; linesToHighlight = [6]; title = 'Disabling All Default Styles'; code-example-end]

ניתן להתאים זאת ללא קוד, בדף ההתאמה האישית של הווידג'ט, תחת אפשרויות מתקדמות:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.show-advanced-option', '.disable-all-default-styling']; selector = '.disable-all-default-styling'; title='Disabling All Default Styles' app-screenshot-end]
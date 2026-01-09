[related-parameter-start name = 'disableNotificationBell'; type = 'boolean'; related-parameter-end]

כברירת מחדל, FastComments יציג פעמון התראות בפינה הימנית העליונה של אזור התגובות.

הפעמון הזה יהפוך לאדום ויציג מונה של מספר ההודעות שיש למשתמש. כמה דוגמאות להודעות הן:

- משתמש הגיב לך.
- משתמש הגיב בשרשור שבו הגבת.
- משתמש הצביע בעד התגובה שלך.
- משתמש הגיב בעמוד שנרשמת אליו.

פעמון ההודעות מספק גם מנגנון למנוי לעמוד שלם.

עם זאת, ניתן להשבית את פעמון ההודעות לחלוטין:

[code-example-start config = {disableNotificationBell: true}; linesToHighlight = [6]; title = 'Disable Notification Bell'; code-example-end]

ניתן לבצע זאת גם ללא קוד. בדף התאמה אישית של הווידג'ט, ראה את הסעיף "השבת פעמון התראות".

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.disable-notification-bell']; selector = '.disable-notification-bell'; title='Disable Notification Bell' app-screenshot-end]

---
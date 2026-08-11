[related-parameter-start name = 'disableNotificationBell'; type = 'boolean'; related-parameter-end]

בברירת מחדל, FastComments יציג פעמון התראות בפינה הימנית העליונה של אזור ההערות.

פעמון זה יהפוך לאדום ויציג ספירה של מספר ההתראות שיש למשתמש. כמה דוגמאות להתראות הן:

- המשתמש השיב לך.
- המשתמש השיב בשרשור שבו הגבת.
- המשתמש הצביע בעד ההערה שלך.
- המשתמש השיב לעמוד שאתה מנוי עליו.

פעמון ההתראות מספק גם מנגנון למנוי על כל העמוד, בנוסף.

עם זאת, ניתן להשבית את פעמון ההתראות לחלוטין:

[code-example-start config = {disableNotificationBell: true}; linesToHighlight = [6]; title = 'Disable Notification Bell'; code-example-end]

זה גם ניתן לבצע ללא קוד. בעמוד התאמת הווידג'ט, ראה את הסעיף "Disable Notification Bell" section.

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.disable-notification-bell']; selector = '.disable-notification-bell'; alt='דף התאמת הווידג''ט עם תיבת הסימון של השבתת פעמון ההתראות מסומנת'; title='השבתת פעמון ההתראות' app-screenshot-end]
[related-parameter-start name = 'defaultAvatarSrc'; type = 'string'; related-parameter-end]

כאשר משתמש מגיב עם FastComments בפעם הראשונה, אנו ננסה לאחזר את תמונת הפרופיל שלו מ-<a href="https://gravatar.com/" target="_blank">https://gravatar.com/</a>.

עם זאת, אם לא נמצא תמונת פרופיל, או שהמשתמש אף פעם לא מגדיר אחת בחשבונו, אנו מציגים תמונת ברירת מחדל סטטית.

כדי לציין תמונת פרופיל סטטית משלך, ניתן להשתמש בהגדרה *defaultAvatarSrc*.

[code-example-start config = {defaultAvatarSrc: "https://example.com/some-image.png"}; linesToHighlight = [6]; title = 'Override The Default Avatar'; code-example-end]

זה גם ניתן לבצע ללא קוד. בעמוד התאמת הווידג'ט, ראו את הסעיף "Default Avatar".

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.default-avatar'; alt='סעיף Avatar ברירת המחדל בעמוד התאמת הווידג\'ט, שבו אתה מגדיר את כתובת ה-URL של תמונת הפרופיל המחליפה'; title='התאמת Avatar ברירת המחדל' app-screenshot-end]

שימו לב שהגדרת תמונת הפרופיל למשתמש ספציפי, כמו עם SSO, מכוסה בסעיף נפרד משלה.
[related-parameter-start name = 'defaultAvatarSrc'; type = 'string'; related-parameter-end]

כאשר משתמש מגיב עם FastComments בפעם הראשונה, ננסה לאחזר את תמונת הפרופיל שלו מ-<a href="https://gravatar.com/" target="_blank">http://gravatar.com/</a>.

עם זאת, אם לא נמצא תמונת פרופיל, או שהמשתמש אף פעם לא מגדיר אחת בחשבונו, נציג תמונת ברירת מחדל סטטית.

כדי לציין תמונת פרופיל סטטית משלך, ניתן להשתמש בהגדרת *defaultAvatarSrc*.

[code-example-start config = {defaultAvatarSrc: "https://example.com/some-image.png"}; linesToHighlight = [6]; title = 'החלפת תמונת ברירת המחדל'; code-example-end]

זה גם ניתן לבצע ללא קוד. בעמוד התאמת הווידג'ט, ראה את החלק "Default Avatar".

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.default-avatar'; alt='החלק של תמונת ברירת המחדל בעמוד התאמת הווידגט, שבו אתה מגדיר את כתובת ה-URL של תמונת ברירת המחדל'; title='התאמת תמונת ברירת המחדל' app-screenshot-end]

שימו לב שהגדרת תמונת פרופיל למשתמש ספציפי, כמו עם SSO, מכוסה בחלק נפרד.
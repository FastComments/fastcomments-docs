---
[related-parameter-start name = 'defaultAvatarSrc'; type = 'string'; related-parameter-end]

כאשר משתמש מגיב ב-FastComments בפעם הראשונה, ננסה לשלוף את תמונת הפרופיל שלו מ-<a href="http://gravatar.com/" target="_blank">http://gravatar.com/</a>.

עם זאת, אם לא נמצא תמונת פרופיל, או שהמשתמש מעולם לא קבע אחת בחשבונו, אנו מציגים תמונת פרופיל סטטית כברירת מחדל.

כדי להגדיר תמונת פרופיל סטטית משלך, ניתן להשתמש בהגדרה *defaultAvatarSrc*.

[code-example-start config = {defaultAvatarSrc: "https://example.com/some-image.png"}; linesToHighlight = [6]; title = 'Override The Default Avatar'; code-example-end]

ניתן גם לבצע זאת ללא קוד. בעמוד התאמת הווידג'ט, ראה את הסעיף 'תמונת ברירת מחדל'.

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.default-avatar'; title='Customizing The Default Avatar' app-screenshot-end]

שימו לב שהגדרת תמונת פרופיל למשתמש ספציפי, כמו באמצעות SSO, מכוסה בסעיף נפרד.

---
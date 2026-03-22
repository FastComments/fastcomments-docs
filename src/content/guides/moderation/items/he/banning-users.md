יש שתי דרכים לחסום משתמשים מלהגיב באתר שלך באמצעות FastComments.

הראשונה היא שאם כבר ידועה לך כתובת הדוא"ל שלהם, תוכל להזין אותה בעמוד ה- <a href="https://fastcomments.com/auth/my-account/moderate-comments/banned-users" target="_blank">משתמשים חסומים</a>.

[app-screenshot-start url='/auth/my-account/moderate-comments/banned-users'; selector = '.content .account-block'; title='The Banned Users Page' app-screenshot-end]

ניתן לגשת לעמוד זה דרך ניהול תגובות -> משתמשים חסומים

כאשר נחסום משתמש, נוכל לבחור סוג — חסימה קבועה או חסימה סמויה קבועה:

[app-screenshot-start url='/auth/my-account/moderate-comments/banned-users/new'; selector = '.content .account-block'; title='Banning a User' app-screenshot-end]

הדרך השנייה לחסום משתמש היא על-ידי לחיצה על כפתור החסימה שממוקם על כל תגובה בעמוד ניהול התגובות.

כאשר נלחץ על כפתור החסימה, יוצגו בפנינו כמה אפשרויות שבהן נוכל לציין את סוג החסימה ומשך הזמן שלה.

### סיומות דואר אלקטרוני (Email Aliases)

בעת חסימת משתמש לפי דוא"ל, FastComments מתעלמת באופן אוטומטי מכינויים עם `+`. לדוגמה, חסימת `user+alias@gmail.com` תחסום גם את `user@gmail.com` ואת כל השינויים האחרים עם `+` של הכתובת הזו, כמו `user+other@gmail.com`.

### חסימות סמויות

חסימה סמויה היא סוג של חסימה שגורמת לכך שנראה כאילו תגובת או הצבעת המשתמש נשמרה בהצלחה, בעוד שבפועל לא נשמרה. מצב זה עשוי להיות רצוי במצבים מסוימים.

### חסימה באמצעות כתובת IP

אלא אם שוכר מעוניין לבטל זאת, FastComments תומכת בחסימה לפי IP על ידי אחסון גרסה מוצפנת (hashed) של כתובת ה-IP של המגיב.

---
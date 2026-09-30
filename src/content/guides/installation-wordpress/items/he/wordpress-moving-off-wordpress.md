אם אתה מעביר את האתר שלך מ‑WordPress ורוצה FastComments באתר החדש, אינך צריך את תוסף WordPress. ייצא את ההערות שלך מ‑WordPress, ואז העלה את הקובץ ב[דף הייבוא](https://fastcomments.com/auth/my-account/manage-data/import) בלוח הבקרה של FastComments.

אנו תומכים בשני פורמטים של ייצוא מ‑WordPress.

### WordPress XML (מומלץ)

זהו הקובץ מהייצוא המובנה של WordPress, ולכן אין צורך בתוסף נוסף.

1. בממשק הניהול של WordPress, עבור אל `Tools -> Export`.
2. בחר `All content` ולחץ על `Download Export File`.
3. ב[דף הייבוא](https://fastcomments.com/auth/my-account/manage-data/import) של FastComments, בחר `WordPress (.xml)` והעלה את הקובץ.

כל תגובה מקושרת לכתובת ה‑URL של הפוסט שבו היא נכתבה, שכבר נמצאת בקובץ.

הייבוא שומר את שם המחבר, האימייל והאתר, את התאריך, התוכן, שרשור תגובות, והאם התגובה אושרה. תמונות הפרופיל של המגיבים מועברות מ‑Gravatar. הצבעות אינן חלק מהפורמט הזה.

### WordPress CSV

זהו הקובץ מ[תוסף ייבוא וייצוא תגובות של WebToffee](https://wordpress.org/plugins/comments-import-export-woocommerce/).

1. התקן את התוסף בממשק הניהול של WordPress וייצא את ההערות שלך כ‑CSV.
2. החלף כל ערך של `comment_post_ID` בכתובת ה‑URL של הפוסט.
3. ב[דף הייבוא](https://fastcomments.com/auth/my-account/manage-data/import) של FastComments, בחר `WordPress (.csv)` והעלה את הקובץ.

כל תגובה מקושרת לעמודת `comment_post_ID`. WordPress ממלא את העמודה הזו עם מזהה הפוסט, והאתר החדש שלך אינו מכיל מזהי פוסטים של WordPress, ולכן שלב 2 מחליף זאת בכתובת ה‑URL.

הייבוא שומר את שם המחבר, האימייל והאתר, את התאריך, התוכן, שרשור תגובות, והאם התגובה אושרה. תמונות הפרופיל של המגיבים מועברות מ‑Gravatar. הוא גם שומר את סימן הספאם של WordPress, ואת הלייקים והדיסלייקים של wpDiscuz כאשר הקובץ כולל אותם.

### התאמת תגובות לדפים החדשים שלך

אם האתר החדש שלך שומר על אותם כתובות URL כמו האתר ב‑WordPress, התגובות יופיעו בדפים המתאימים ללא צורך בהגדרות נוספות.

אם הדומיין משתנה, הפעל את [כלי מיגרציית דומיינים](/guide-migrations.html#migrating-domains) לאחר הייבוא. אם כתובות ה‑URL של דפים בודדים משתנות, ניתן [להעביר כל דף](/guide-migrations.html#migrating-pages) מכתובת ה‑URL הישנה לחדשה.

להעברות דפים בכמות גדולה, כגון הסרת הדומיין מהערך שאתה מעביר לשדה [urlId](/guide-customizations-and-configuration.html#url-id) של וידג׳ט התגובה, [פתח כרטיס תמיכה](https://fastcomments.com/auth/my-account/help) ואנו נטפל בזה עבורך.

### לפני שאתה מעביר

אתה יכול להריץ את הייבוא כמה פעמים שתרצה. ייבוא מחדש של אותו קובץ [אינו יוצר כפילויות](/guide-migrations.html#importing-data), ולכן ניתן לייבא פעם אחת כדי לבדוק את האתר החדש, ואז לייבא שוב עם ההערות האחרונות שלך לפני המעבר.

לקבצי ייצוא גדולים מ‑1 GB, [פנה לתמיכה](https://fastcomments.com/auth/my-account/help).

להוספת FastComments לאתר החדש שלך, ראה את [מדריך ההתקנה](/guide-installation.html).
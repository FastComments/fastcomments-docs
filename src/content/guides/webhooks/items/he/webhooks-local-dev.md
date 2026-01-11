לצורך פיתוח מקומי, השתמש בכלי כמו [ngrok](https://ngrok.com/).

כדי לפשט את שמירת המערכת מאובטחת, הפיתוח המקומי עוקב אחרי אותו תהליך כמו התקנה ואבטחת סביבות אחרות. 

### שלב 1: הוסף את "localhost" לדומיינים בחשבונך.

הוסף את "localhost" [כדומיין כאן](https://fastcomments.com/auth/my-account/configure-domains).

[app-screenshot-start url='https://fastcomments.com/auth/my-account/configure-domains/new'; selector = '.content .account-block'; title='Add localhost'; actions=[{type: 'set-value', selector: 'input[name=domainNames]', value: 'localhost'}] app-screenshot-end]

### שלב 2: בחר מפתח API

נוסיף קונפיגורציית webhook לדומיין שלך, לכן נצטרך מפתח API. [אפשר לעשות זאת כאן.](https://fastcomments.com/auth/my-account/api-secret)

[app-screenshot-start url='https://fastcomments.com/auth/my-account/api-secret/add'; selector = '.content .account-block'; title='Add Testing API Key'; actions=[{type: 'set-value', selector: 'select[name=domain]', value: 'localhost'}, {type: 'set-value', selector: 'input[name=name]', value: 'Testing'}] app-screenshot-end]

מתחת ל-"Associate with domain" - בחר את הדומיין "localhost".

**הערה: לחלופין, ניתן להשתמש בסוד API אחד לכל פעילות הבדיקה וסביבות ה-staging. פשוט הוסף סוד API עבור "All Domains", ותגיד לו שם כמו "test".**

וודא שיש לך סוד API מוגדר עבור דומיין/דומיינים של production. אירועים עבור שאר הדומיינים ישתמשו בסוד ה-wildcard (לצורך בדיקות).

### שלב 3: הוסף את ה-webhook שלך

בעת הרצת ngrok או כלי דומה, הגדר את הערך עבור "localhost" [כאן](https://fastcomments.com/auth/my-account/manage-data/webhooks).

[app-screenshot-start url='https://fastcomments.com/auth/my-account/manage-data/webhooks'; selector = '.content'; title='Add Testing Webhook'; actions=[{type: 'wait', selector: 'button[type=submit]'}, {type: 'set-value', selector: '#domain-select', value: 'localhost'}, {type: 'set-value', selector: 'input[name="comment-created-url"]', value: 'http://xxxx-xxxx-xxxx-xxxx.ngrok.io/some-route'}] app-screenshot-end]

כאשר תלחץ על `Send Test Payload`, אנחנו נשלח שני אירועי בדיקה כדי לבדוק שאכן אתם מאמתים את מפתח ה‑API.

אם האימות יצליח, לחץ על `Save`.

### שלב 4: הוסף תגובה

כעת תוכל/תוכלו להוסיף, לערוך או למחוק תגובות ולראות שנקרא למכונת הפיתוח המקומית שלך עם האירועים, תוך שימוש במפתח ה‑API לניסוי. עשוי להיות עיכוב של עד 30 שניות
עד שהאירועים יגיעו למכונה שלך.
לפיתוח מקומי (Local development), השתמשו בכלי כמו [ngrok](https://ngrok.com/).

על מנת לפשט את שמירת האבטחה של המערכת, פיתוח מקומי עוקב אחר אותו תהליך כמו הגדרה ואבטחה של סביבות אחרות.

### שלב 1: הוסיפו "localhost" לדומיינים בחשבונכם.

הוסיפו "localhost" [כדומיין כאן](https://fastcomments.com/auth/my-account/configure-domains).

[app-screenshot-start url='https://fastcomments.com/auth/my-account/configure-domains/new'; cacheBuster = 'v3'; selector = '.content .account-block'; alt='טופס הוספת דומיין בהגדרות החשבון עם localhost שהוזן בשדה שמות הדומיינים'; title='הוספת localhost'; actions=[{type: 'set-value', selector: 'input[name=domainNames]', value: 'localhost'}] app-screenshot-end]

### שלב 2: בחרו מפתח API

אנחנו הולכים להוסיף הגדרת webhook עבור הדומיין שלכם, ולכן נזדקק למפתח API. [תוכלו לעשות זאת כאן.](https://fastcomments.com/auth/my-account/api-secret)

[app-screenshot-start url='https://fastcomments.com/auth/my-account/api-secret/add'; cacheBuster = 'v3'; selector = '.content .account-block'; alt='טופס סוד API חדש עם הדומיין המשויך מוגדר ל-localhost והמפתח בשם Testing'; title='הוספת מפתח API לבדיקות'; actions=[{type: 'set-value', selector: 'select[name=domain]', value: 'localhost'}, {type: 'set-value', selector: 'input[name=name]', value: 'Testing'}] app-screenshot-end]

תחת "Associate with domain" - בחרו את הדומיין "localhost" שלכם.

**הערה: לחלופין, תוכלו להשתמש בסוד API אחד עבור כל פעילות הבדיקות וסביבות ה-staging. פשוט הוסיפו סוד API עבור "All Domains", ותנו לו שם כמו "test".**

ודאו שיש לכם סוד API מוגדר עבור הדומיין/דומיינים שלכם בפרודקשן. אירועים עבור כל שאר הדומיינים ישתמשו בסוד ה-wildcard (הבדיקות).

### שלב 3: הוסיפו את ה-Webhook שלכם

בזמן הרצת ngrok או כלי דומה, הגדירו את הערך עבור "localhost" [כאן](https://fastcomments.com/auth/my-account/manage-data/webhooks).

[app-screenshot-start url='https://fastcomments.com/auth/my-account/manage-data/webhooks'; cacheBuster = 'v3'; selector = '.content'; alt='ניהול webhooks עם הדומיין localhost נבחר וכתובת ngrok שהוזנה בנקודת הקצה של יצירת תגובה'; title='הוספת webhook לבדיקות'; actions=[{type: 'wait', selector: 'button[type=submit]'}, {type: 'set-value', selector: '#domain-select', value: 'localhost'}, {type: 'set-value', selector: 'input[name="comment-created-url"]', value: 'http://xxxx-xxxx-xxxx-xxxx.ngrok.io/some-route'}]; app-screenshot-end]

בלחיצה על `Send Test Payload`, נשלח שני אירועי בדיקה כדי לוודא שאתם מאמתים את מפתח ה-API.

לאחר שהאימות עובר, לחצו על `Save`.

### שלב 4: הוסיפו תגובה

כעת תוכלו להוסיף, לערוך או למחוק תגובות, ואמורים לראות אותנו קוראים למכונת הפיתוח המקומית שלכם עם האירועים, תוך שימוש במפתח ה-API לבדיקות שלכם. עשוי להיות עיכוב של עד 30 שניות
עד שהאירועים יגיעו למכונה שלכם.
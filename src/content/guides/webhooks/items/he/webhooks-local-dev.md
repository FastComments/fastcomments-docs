For Local development, use a tool like [ngrok](https://ngrok.com/).

In order to simplify keeping the system secure, local development follows the same process as setting up and securing other environments. 

### שלב 1: הוספת "localhost" לדומיינים בחשבונכם.

Add "localhost" [as a domain here](https://fastcomments.com/auth/my-account/configure-domains).

[app-screenshot-start url='https://fastcomments.com/auth/my-account/configure-domains/new'; cacheBuster = 'v3'; selector = '.content .account-block'; alt='טופס הוספת דומיין בהגדרות החשבון עם localhost מוזן בשדה שמות הדומיינים'; title='הוסף localhost'; actions=[{type: 'set-value', selector: 'input[name=domainNames]', value: 'localhost'}] app-screenshot-end]

### שלב 2: בחירת מפתח API

We're going to be adding webhook configuration for your domain, so we'll need an API key. [You can do that here.](https://fastcomments.com/auth/my-account/api-secret)

[app-screenshot-start url='https://fastcomments.com/auth/my-account/api-secret/add'; cacheBuster = 'v3'; selector = '.content .account-block'; alt='טופס סוד API חדש עם הדומיין המשויך מוגדר ל‑localhost והמפתח נקרא Testing'; title='הוסף מפתח API לבדיקות'; actions=[{type: 'set-value', selector: 'select[name=domain]', value: 'localhost'}, {type: 'set-value', selector: 'input[name=name]', value: 'Testing'}] app-screenshot-end]

Under "Associate with domain" - select your "localhost" domain.

**הערה: לחלופין, ניתן להשתמש בסוד API אחד לכל פעילות בדיקה וסביבות staging. פשוט הוסיפו סוד API עבור "All Domains", ותנו לו שם כמו "test".**

Ensure you have an API Secret defined for your production domain(s). Events for all other domains will use the wildcard (testing) secret.

### שלב 3: הוספת ה‑Webhook שלכם

While running ngrok or similar tool, set the value for "localhost" [here](https://fastcomments.com/auth/my-account/manage-data/webhooks).

[app-screenshot-start url='https://fastcomments.com/auth/my-account/manage-data/webhooks'; cacheBuster = 'v3'; selector = '.content'; alt='ממשק ניהול Webhooks עם הדומיין localhost נבחר וכתובת ngrok מוזנת בנקודת הקצה של יצירת תגובה'; title='הוסף Webhook לבדיקות'; actions=[{type: 'wait', selector: 'button[type=submit]'}, {type: 'set-value', selector: '#domain-select', value: 'localhost'}, {type: 'set-value', selector: 'input[name="comment-created-url"]', value: 'http://xxxx-xxxx-xxxx-xxxx.ngrok.io/some-route'}]; app-screenshot-end]

When clicking `Send Test Payload`, we will send two test events to check that you validate the API key.

Once it validates, hit `Save`.

### שלב 4: הוספת תגובה

Now you can add, edit, or delete comments and should see us call your local development machine with the events, using your testing API key. There may be up to 30 seconds delay for the events to reach your machine.

---
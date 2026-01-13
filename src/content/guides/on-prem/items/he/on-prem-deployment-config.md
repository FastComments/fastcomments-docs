FastComments משתמשת במשתני סביבה להגדרות. הרשימה הבאה מפרטת את כל המשתנים הנתמכים שרלוונטיים לפריסה מקומית (On-Prem).


| Variable                       | Default                     | Info                                                                                                                                               | Required | Examples or Valid Values                              |
|--------------------------------|-----------------------------|----------------------------------------------------------------------------------------------------------------------------------------------------|----------|-------------------------------------------------------|
| NODE_ENV                       |                             | סוג הסביבה.                                                                                                                                       | כן       | production, dev                                       |
| MONGO_URI                      |                             | URI לחיבור למסד הנתונים.                                                                                                                           | כן       |                                                       |
| MONGO_ENABLE_SSL               | false                       | מאפשר שימוש ב-SSL לחיבור למסד הנתונים.                                                                                                            | לא       | true, false                                           |
| MONGO_ENABLE_SSL_VALIDATE      | false                       | מאפשר אימות התעודה מול תעודת הרשות (CA) בעת חיבור ל-Mongo.                                                                                       | לא       | true, false                                           |
| MONGO_SSL_CA                   |                             | קובץ PEM של ה-CA ל-SSL של Mongo.                                                                                                                  | לא       | /path/to/some-cert.pem                                |
| ADMIN_NOTIFICATIONS_EMAIL      |                             | דוא״ל שאליו יישלחו התראות מערכת חשובות.                                                                                                            | לא       | admin-group@bigcorp.com                               |
| IP_HASH_SALT                   |                             | מלח (salt) להאשטת כתובות IP.                                                                                                                      | כן       |                                                       |
| SESSION_SECRET                 |                             | המפתח המשמש לחתימת סשנים.                                                                                                                         | כן       |                                                       |
| SESSION_STORE_SECRET           |                             | המפתח המשמש לחתימה/האשת (hash) של סשנים באחסון. חייב להיות שונה מ-SESSION_SECRET.                                                                 | כן       |                                                       |
| HOSTNAME                       |                             | שם המארח שבו FastComments פרוס (לוח ניהול וכו'). לא צריך לכלול פורט או פרוטוקול.                                                                  | כן       | example.com                                           |
| HOST_ADDR                      |                             | URI נגיש שבו FastComments פרוס (לוח ניהול וכו').                                                                                                   | כן       | https://example.com                                   |
| EMAIL_CONFIG_PATH              |                             | נתיב במערכת הקבצים המקומית שבו ממוקמת תצורת הדוא״ל (SMTP, התאמות דומיין/ספקים וכו׳).                                                               | כן       | /my/config.json                                       |
| EMAIL_DEFAULT_FROM_NAME        | FastComments Robot          | כותרת "שם השולח" בהודעות דוא״ל.                                                                                                                   | לא       | My Company Name                                       |
| EMAIL_DEFAULT_FOOTER_LOGO      | /images/logo-32-2020-01.png | לוגו בפוטר של הדוא״ל.                                                                                                                             | לא       | https://exmaple.com/footer.png                        |
| EMAIL_DEFAULT_TRANSPORT        |                             | החלפה ל-"defaultTransport" בתוך EMAIL_CONFIG_PATH. שימושי לפריסת אותו קובץ קונפיג על פני סביבות שונות.                                            | לא       | myTransportName                                       |
| ON_PREM_TENANT_ID              |                             | ה-ID של החשבון שלכם ב-fastcomments.com. משמש להרשמת מפתח הרישיון.                                                                                 | לא       |                                                       |
| ON_PREM_LICENSE_KEY            |                             | מפתח רישיון לפריסה מקומית.                                                                                                                         | לא       |                                                       |
| GIPHY_API_KEY                  |                             | מפתח API ל-Giphy. אם לא צויין, יש ליצור כלל בקונפיג שיבטל את בורר ה-GIF.                                                                         | לא       |                                                       |
| GIPHY_DEFAULT_RATING           | pg                          | משמש באינטגרציה עם Giphy. ניתן גם להחליף באמצעות חוקי התאמת וידג'ט.                                                                                | לא       | g, pg, pg-13, r                                       |
| OPENAI_SECRET_KEY              |                             | משמש לתכונות מבוססות OpenAI כגון זיהוי ספאם אופציונלי מבוסס GPT.                                                                                   | לא       |                                                       |
| CDN_HOST_ADDR                  |                             | שם המארח ממנו יתאחזרו נכסים (assets). ברירת המחדל היא הערך של HOSTNAME.                                                                          | לא       | example.com                                           |
| LARGE_FILE_HOST_ADDR           |                             | שם המארח ממנו ייטענו קבצים גדולים (כמו ייצוא). ברירת המחדל היא הערך של CDN_HOST_ADDR.                                                            | לא       | example.com                                           |
| LARGE_FILE_LOCATION_TYPE       | local_disk                  | היכן מאוחסנים קבצים גדולים, כגון ייצוא.                                                                                                           | לא       | local_disk, s3                                        |
| FROM_EMAIL_HOST                |                             | שם המארח שממנו ישלחו הודעות דוא״ל.                                                                                                                | לא       | example.com                                           |
| COOKIE_ID                      | fastcomments.sid            | שם העוגייה של fastcomments.                                                                                                                         | לא       |                                                       |
| COOKIE_HOSTNAME                | .fastcomments.com           | הערך של שדה 'hostname' בעוגייה. מומלץ להקדים בנקודה.                                                                                               | לא       | .example.com                                          |
| S3_ACCESS_KEY                  |                             | משמש להעלאות קבצים של משתמשים, אווטארים וכו'. אם לא מוגדר, ברירת המחדל היא מערכת הקבצים המקומית.                                                  | לא       |                                                       |
| S3_SECRET_KEY                  |                             | משמש להעלאות קבצים של משתמשים, אווטארים וכו'.                                                                                                      | לא       |                                                       |
| S3_REGION                      |                             | אזור (region) לשימוש בהעלאות S3.                                                                                                                   | לא       |                                                       |
| S3_BUCKET                      |                             | דלי (bucket) לשימוש בהעלאות קבצים.                                                                                                                 | לא       |                                                       |
| S3_HOST                        |                             | ה-host של שירות S3.                                                                                                                                | לא       |                                                       |
| CACHE_DIR                      |                             | מיקום לשמירת מטמון אופציונלי לא מקוון, לשימוש כאשר DB אינו זמין. מתרענן תקופתית עם 100 השרשורים המובילים של תגובות.                              | לא       |                                                       |
| BACKUP_DIR                     |                             | מיקום לאחסון נתונים לשימוש כאשר DB אינו זמין. אם תגובה נשלחת כשה-DB אינו זמין היא תישמר כאן ותטופל מאוחר יותר.                                     | לא       |                                                       |

שים לב שכל המשתנים הקשורים לדומיין משתמשים בסיומת `_HOST` או `_ADDR`. ההבדל הוא:

- `_HOST`: `example.com`
- `_ADDR`: `https://example.com`

המשתנה `EMAIL_CONFIG_PATH` צריך להכיל נתיב לקובץ JSON בפורמט הדוגמה הבא:

[inline-code-attrs-start title = 'תצורת דוא"ל'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{
    "defaultDKIM": {
        "domainName": "mycompany.org",
        "keySelector": "2024",
        "privateKey": "-----BEGIN PRIVATE KEY-----\nABCDEFG\n-----END PRIVATE KEY-----"
    },
    "providerTransports": {
        "yahoo.com": "specialTransport"
    },
    "defaultTransport": "mailgun",
    "transports": {
        "mailgun": {
            "host": "smtp.mailgun.org",
            "port": 587,
            "secure": false,
            "auth": {
                "user": "admin@somewhere.com",
                "pass": "password"
            },
            "tls": {
                "ciphers": "SSLv3"
            }
        },
        "specialTransport": {
            "host": "smtp.someplace.org",
            "port": 587,
            "secure": false,
            "auth": {
                "user": "admin@example.com",
                "pass": "password"
            },
            "tls": {
                "ciphers": "SSLv3"
            }
        }
    }
}
[inline-code-end]

בדוגמה שלמעלה הגדרנו טרנספורט דוא״ל ברירת מחדל `SMTP` בשם `mailgun`. כמו כן הגדרנו טרנספורט מיוחד שאותו אנו משתמשים ספציפית עבור כתובות של `@yahoo.com`. במצבים מסוימים רצוי להשתמש בספק ספציפי או בכתובת IP שולחת עבור דומיין כדי לכוונן את מסירת ההודעות. זה אופציונלי.

### DocumentDB

בעת חיבור ל-`DocumentDB` תרצו להגדיר `MONGO_ENABLE_SSL=true MONGO_SSL_CA=/some/path.pem` כדי להיות תואמים להגדרות ברירת המחדל.

---
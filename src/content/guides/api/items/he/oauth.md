FastComments הוא שרת אימות OAuth 2.1. אפליקציה יכולה לקבל אסימון הקשור לחשבון FastComments אחד ולהשתמש בו בכל נקודת קצה במדריך זה במקום מפתח API. כך מתחבר אפליקציית Zapier, שרת MCP ושילובים של צד שלישי אחרים.

אסימונים ניתנים דרך זרימת קוד האימות עם PKCE. אין אישורי לקוח או מתן מרומז.

### גילוי

כתובות נקודות הקצה, מענקים נתמכים ושיטות אימות מתפרסמים בכתובת המטא‑נתונים הסטנדרטית:

[inline-code-attrs-start title = 'מטא-נתונים של שרת האימות'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
https://fastcomments.com/.well-known/oauth-authorization-server
[inline-code-end]

הנקודות קצה שהיא מתארת:

[inline-code-attrs-start title = 'נקודות קצה של OAuth'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
GET  https://fastcomments.com/oauth/authorize
POST https://fastcomments.com/oauth/token
POST https://fastcomments.com/oauth/revoke
POST https://fastcomments.com/oauth/register
[inline-code-end]

חשבונות באיזור האיחוד האירופי משתמשים ב‑`https://eu.fastcomments.com` כמנפיק, עם אותם הנתיבים.

### רישום לקוח

לקוח צריך `client_id` ו‑`redirect_uri` רשום לפני שהוא יכול להתחיל את הזרימה. ישנן שתי דרכים לקבל אחד:

- **רישום דינמי של לקוח.** `POST /oauth/register` עם גוף JSON לפי RFC 7591 (`redirect_uris`, `client_name`, `client_uri`, `logo_uri`, `token_endpoint_auth_method`). התגובה מכילה את `client_id` וללקוחות סודיים, את `client_secret`. הרישום אינו מאומת ומוגבל בקצב לפי IP.
- **מסמך מטא‑נתונים של מזהה לקוח.** הלקוח משתמש בכתובת `https` שהוא שולט בה כמזהה `client_id`. FastComments מושך את הכתובת וקורא את אותם שדות מטא‑נתונים ממנה. אין צורך בקריאת רישום.

אפליקציות שותף המופיעות בלוח הבקרה של FastComments, כגון Zapier, נרשמות ישירות על ידי FastComments. פנה לתמיכה אם אתה בונה רשימת שוק וצריך לקוח של צד ראשון.

### היקפים

[inline-code-attrs-start title = 'היקפים'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
read   GET and HEAD requests
write  every other method
[inline-code-end]

בקשה שלא מבקשת היקף מקבלת את שני ההיקפים. המשתמש רואה את ההיקפים המבוקשים בעמוד ההסכמה. בקשה להיקף שאינו אחד משניים אלו נכשלת עם `invalid_scope`.

### שלב 1 - בקשת אימות

שלח את דפדפן המשתמש לנקודת הקצה של האימות. PKCE עם שיטת `S256` נדרש לכל לקוח.

[inline-code-attrs-start title = 'בקשת אימות'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
GET https://fastcomments.com/oauth/authorize
    ?response_type=code
    &client_id=YOUR_CLIENT_ID
    &redirect_uri=https://example.com/oauth/callback
    &scope=read%20write
    &state=RANDOM_STATE
    &code_challenge=BASE64URL_SHA256_OF_VERIFIER
    &code_challenge_method=S256
[inline-code-end]

[inline-code-attrs-start title = 'פרמטרים של בקשת אימות'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface OAuthAuthorizeQueryParams {
    response_type: 'code'
    client_id: string
    /** חייב להתאים בדיוק לאחת מכתובות ה‑redirect הרשומות של הלקוח. **/
    redirect_uri: string
    /** מופרדים ברווח. השמט כדי לבקש את שני ההיקפים. **/
    scope?: 'read' | 'write' | 'read write'
    /** נשלח ללא שינוי ב‑redirect. השתמש בו כדי לקשר את הקריאה חזרה למפגש שהתחיל את הזרימה. **/
    state?: string
    /** base64url(sha256(code_verifier)). **/
    code_challenge: string
    code_challenge_method: 'S256'
    /** מחוון משאב RFC 8707 אופציונלי. אם נשלח, יש לשלוח את אותו ערך לנקודת הקצה של האסימון. **/
    resource?: string
}
[inline-code-end]

המשתמש נכנס ל‑FastComments אם נדרש ורואה עמוד הסכמה שמציג את שם האפליקציה שלך, החשבון שאליו היא תתחבר, וההיקפים המבוקשים. על המשתמש להחזיק בהרשאת **API Admin** על החשבון; כל משתמש אחר רואה שגיאת הרשאה במקום טופס ההסכמה. אישור מפנה את הדפדפן ל‑`redirect_uri` שלך עם `code` ו‑`state`. דחייה מפנה עם `error=access_denied`.

קוד האימות תקף למשך 10 דקות וניתן להחליף אותו פעם אחת. החלפה שנייה של אותו קוד מבטלת כל אסימון שהחלפה הראשונה יצרה.

### שלב 2 - בקשת אסימון

החלף את הקוד לאסימונים. הגוף מקודד כטופס. לקוחות סודיים מאמתים עם `client_secret_basic` (HTTP Basic) או `client_secret_post` (סוד בגוף). לקוחות ציבוריים שולחים רק `client_id`.

[inline-code-attrs-start title = 'דוגמת cURL לבקשת אסימון'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/oauth/token' \
  --header 'Content-Type: application/x-www-form-urlencoded' \
  --data 'grant_type=authorization_code' \
  --data 'client_id=YOUR_CLIENT_ID' \
  --data 'client_secret=YOUR_CLIENT_SECRET' \
  --data 'code=fcac_...' \
  --data 'code_verifier=YOUR_PKCE_VERIFIER' \
  --data 'redirect_uri=https://example.com/oauth/callback'
[inline-code-end]

[inline-code-attrs-start title = 'גוף בקשת אסימון (authorization_code)'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface OAuthTokenRequestAuthorizationCode {
    grant_type: 'authorization_code'
    client_id: string
    /** ללקוחות סודיים בלבד. ניתן לשלוח כאימות HTTP Basic במקום זאת. **/
    client_secret?: string
    code: string
    code_verifier: string
    /** חייב להתאים לבקשת האימות שנשלחה. **/
    redirect_uri?: string
    resource?: string
}
[inline-code-end]

[inline-code-attrs-start title = 'מבנה תגובת אסימון'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface OAuthTokenResponse {
    /** מתחיל ב‑fcat_. תקף לשעה אחת. **/
    access_token: string
    token_type: 'bearer'
    /** שניות עד שפג תוקף אסימון הגישה. 3600. **/
    expires_in: number
    /** מתחיל ב‑fcrt_. תקף ל‑30 יום מההנפקה. **/
    refresh_token: string
    /** היקפים שהוענקו מופרדים ברווח. **/
    scope: string
}
[inline-code-end]

השגיאות תואמות ל‑RFC 6749: גוף JSON עם `error` ו‑`error_description`, HTTP 400 עבור `invalid_request`, `invalid_grant`, `invalid_scope`, `invalid_target` ו‑`unsupported_grant_type`, HTTP 401 עבור `invalid_client`, HTTP 429 כאשר יש הגבלה בקצב.

### שלב 3 - קריאת ה‑API

שלח את אסימון הגישה כאסימון bearer. השוכר (tenant) נרמז מהאסימון, ולכן `tenantId` הוא אופציונלי. כאשר נמסר, הוא חייב להתאים לאסימון אחרת הבקשה תיכשל.

[inline-code-attrs-start title = 'דוגמת cURL לאסימון Bearer'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/me' \
  --header 'Authorization: Bearer fcat_...'
[inline-code-end]

`GET /api/v1/me` מחזיר את השוכר, המשתמש המאשר, וההיקפים שהוענקו, מה שהופך אותה לקריאה המתאימה לבדיקת חיבור. בקשה עם אסימון שפג תוקפו או שבוטל מחזירה HTTP 401. בקשה שהמתודה שלה דורשת היקף שהאסימון אינו מחזיק בו מחזירה HTTP 403.

### שלב 4 - רענון

[inline-code-attrs-start title = 'דוגמת cURL לבקשת רענון'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/oauth/token' \
  --header 'Content-Type: application/x-www-form-urlencoded' \
  --data 'grant_type=refresh_token' \
  --data 'client_id=YOUR_CLIENT_ID' \
  --data 'client_secret=YOUR_CLIENT_SECRET' \
  --data 'refresh_token=fcrt_...'
[inline-code-end]

[inline-code-attrs-start title = 'גוף בקשת אסימון (refresh_token)'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface OAuthTokenRequestRefreshToken {
    grant_type: 'refresh_token'
    client_id: string
    client_secret?: string
    refresh_token: string
    /** אופציונלי. מצמצם לתת‑קבוצה של ההיקפים שהוענקו במקור. **/
    scope?: string
    resource?: string
}
[inline-code-end]

התגובה בעלת המבנה זהה להחלפת הקוד. אסימוני רענון מתחלפים: כל רענון מחזיר `refresh_token` חדש ומבטל את הקודם לאחר חלון חסד של 30 שניות לבקשות מקבילות. הצגת אסימון רענון שהסתובב לפני יותר מ‑30 שניות מתייחסת כהעתקה חוזרת ומבטלת את כל המענק. אפליקציות שותף שנרשמו על ידי FastComments פטורות מהחלפה ומקבלות את אותו אסימון רענון עם תוקף מורחב בעוד 30 יום.

רענון גם בודק שהמשתמש המאשר עדיין מחזיק בהרשאת API Admin על החשבון. אם לא, המענק מבוטל והתגובה היא `invalid_grant`.

### ביטול

[inline-code-attrs-start title = 'דוגמת cURL לבקשת ביטול'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/oauth/revoke' \
  --header 'Content-Type: application/x-www-form-urlencoded' \
  --data 'client_id=YOUR_CLIENT_ID' \
  --data 'client_secret=YOUR_CLIENT_SECRET' \
  --data 'token=fcrt_...' \
  --data 'token_type_hint=refresh_token'
[inline-code-end]

ביטול אסימון רענון מבטל כל אסימון גישה שהונפק מאותו מענק. ביטול אסימון גישה מבטל רק את האסימון הזה. נקודת הקצה מחזירה HTTP 200 עם אובייקט JSON ריק בין אם האסימון נמצא ובין אם לא, לפי RFC 7009.

משתמשים יכולים גם לבטל חיבור מ‑**אפליקציות מחוברות** בלוח הבקרה של FastComments. כל אסימון עבור אותה אפליקציה מפסיק לעבוד מיידית.

---
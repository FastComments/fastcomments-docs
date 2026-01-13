[api-resource-header-start name = 'SSOUser'; route = 'POST /api/v1/sso-users'; creditsCost = 1; api-resource-header-end]

נתיב זה מספק יצירה של משתמש SSO יחיד.

ניסיון ליצור שני משתמשים עם אותו מזהה יגרום לשגיאה.

[inline-code-attrs-start title = 'דוגמת cURL ליצירת SSOUser'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/sso-users?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"id": "my-user-id",
	"username": "fordperfect",
	"displayName": "Ford Perfect",
	"email": "fordperfect@galaxy.com",
    "groupIds": ["some-optional-group-id"]
}'
[inline-code-end]

בדוגמה זו אנחנו מציינים `groupIds` לבקרת גישה, אבל זה אופציונלי.

[inline-code-attrs-start title = 'מבנה בקשת יצירת SSOUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface SSOUserPostQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'מבנה תגובת יצירת SSOUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface SSOUserPostResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'empty-request' | 'invalid-input' | 'missing-id' | 'user-exists'
    /** Included on failure. **/
    reason?: string
    user?: SSOUser; // We return the created user on success.
}
[inline-code-end]

#### הערת אינטגרציה

נתונים שמועברים על ידי ה-API יכולים להידרס פשוט על ידי העברת מטען HMAC של משתמש SSO שונה. לדוגמה, אם
אתה מגדיר שם משתמש דרך ה-API, אבל אז מעביר שם אחר דרך זרימת ה-SSO בטעינת העמוד, אנחנו נעדכן אוטומטית
את שם המשתמש שלו.

אנחנו לא נעדכן פרמטרי משתמש בזרימה זו אלא אם כן אתה מציין אותם במפורש או מגדיר אותם ל-null (לא undefined).

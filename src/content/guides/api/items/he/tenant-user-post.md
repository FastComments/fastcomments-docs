[api-resource-header-start name = 'TenantUser'; route = 'POST /api/v1/tenant-users'; creditsCost = 1; api-resource-header-end]

נתיב זה מספק את היכולת להוסיף `TenantUser` יחיד.

ליצירת `TenantUser` יש את ההגבלות הבאות:

- נדרש `username`.
- נדרש `email`.
- ה-`signUpDate` לא יכול להיות בעתיד.
- ה-`locale` חייב להיות ברשימת [לוקלים נתמכים](https://github.com/FastComments/fastcomments-typescript/blob/main/src/constants.ts#L1).
- ה-`username` חייב להיות ייחודי בכל FastComments.com. אם זו בעיה, אנו מציעים להשתמש ב-SSO במקום זאת.
- ה-`email` חייב להיות ייחודי בכל FastComments.com. אם זו בעיה, אנו מציעים להשתמש ב-SSO במקום זאת.
- לא ניתן ליצור יותר משתמשי שוכר מהמוגדר תחת `maxTenantUsers` בחבילה שלך.

אנחנו יכולים ליצור `TenantUser` כדלקמן

[inline-code-attrs-start title = 'דוגמת cURL ליצירת TenantUser'; type = 'bash'; useDemoTenantUser = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/tenants?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
    "username": "Some Name",
	"email": "someone@someone.com"
}'
[inline-code-end]

[inline-code-attrs-start title = 'מבנה בקשת יצירת TenantUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantUserPostQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'מבנה תגובת יצירת TenantUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantUserPostResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'username-required' | 'email-required' | 'sign-up-date-in-future' | 'unsupported-locale' | 'unauthorized' | 'username-taken' | 'email-taken' | 'tenant-user-limit-reached'
    /** Included on failure. **/
    reason?: string
    tenantUser?: TenantUser; // We return the complete created tenant user on success.
}
[inline-code-end]

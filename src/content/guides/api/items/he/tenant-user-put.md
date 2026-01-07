[api-resource-header-start name = 'TenantUser'; route = 'PUT /api/v1/tenant-users/:id'; creditsCost = 1; api-resource-header-end]

נתיב זה מספק את היכולת להחליף `TenantUser` יחיד.

להחלפת `TenantUser` יש את ההגבלות הבאות:

- ה-`signUpDate` לא יכול להיות בעתיד.
- ה-`locale` חייב להיות ברשימת [לוקלים נתמכים](https://github.com/FastComments/fastcomments-typescript/blob/main/src/constants.ts#L1).
- ה-`username` חייב להיות ייחודי בכל FastComments.com. אם זו בעיה, אנו מציעים להשתמש ב-SSO במקום זאת.
- ה-`email` חייב להיות ייחודי בכל FastComments.com. אם זו בעיה, אנו מציעים להשתמש ב-SSO במקום זאת.
- לא ניתן לעדכן את ה-`tenantId` של משתמש.

אנחנו יכולים ליצור `TenantUser` כדלקמן

[inline-code-attrs-start title = 'דוגמת cURL להחלפת TenantUser'; type = 'bash'; useDemoTenantUser = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request PUT \
  --url 'https://fastcomments.com/api/v1/tenant-users/xyz?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
    "username": "Some Name",
	"email": "someone@someone.com"
}'
[inline-code-end]

[inline-code-attrs-start title = 'מבנה בקשת החלפת TenantUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantUserPutQueryParams {
    tenantId: string
    API_KEY: string
    /** When email or username is changed, you can set this to true to also update the user's comments. This will double the credit cost. **/
    updateComments: 'true' | 'false'
}
[inline-code-end]

[inline-code-attrs-start title = 'מבנה תגובת החלפת TenantUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantUserPutResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'sign-up-date-in-future' | 'unsupported-locale' | 'unauthorized' | 'username-taken' | 'email-taken' | 'unauthorized' | 'no-package' | 'invalid-package' | 'tenant-user-limit-reached' | 'user-does-not-exist'
    /** Included on failure. **/
    reason?: string
}
[inline-code-end]

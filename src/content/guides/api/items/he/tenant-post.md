[api-resource-header-start name = 'Tenant'; route = 'POST /api/v1/tenants'; creditsCost = 1; api-resource-header-end]

נתיב זה מספק את היכולת להוסיף `Tenant` יחיד.

ליצירת `Tenant` יש את ההגבלות הבאות:

- נדרש `name`.
- נדרש `domainConfiguration`.
- הערכים הבאים לא ניתנים לספק בעת יצירת `Tenant`:
  - `hasFlexPricing`
  - `lastBillingIssueReminderDate`
  - `flexLastBilledAmount`
- ה-`signUpDate` לא יכול להיות בעתיד.
- ה-`name` לא יכול להיות ארוך מ-`200 תווים`.
- ה-`email` לא יכול להיות ארוך מ-`300 תווים`.
- ה-`email` חייב להיות ייחודי בכל שוכרי FastComments.com.
- לא ניתן ליצור שוכרים אם לשוכר האב אין `TenantPackage` תקף מוגדר.
  - אם השוכר שלך נוצר דרך FastComments.com, זו לא אמורה להיות בעיה.
- לא ניתן ליצור יותר שוכרים מהמוגדר תחת `maxWhiteLabeledTenants` בחבילה שלך.
- חייב לציין את פרמטר השאילתה `tenantId` שהוא המזהה של `שוכר האב` שלך עם תיוג לבן מופעל.

אנחנו יכולים ליצור `Tenant` עם רק כמה פרמטרים:

[inline-code-attrs-start title = 'דוגמת cURL ליצירת Tenant'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/tenants?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
    "name": "Some Name",
	"email": "someone@someone.com",
    "domainConfiguration": [ { "domain": "somedomain.com" } ]
}'
[inline-code-end]

[inline-code-attrs-start title = 'מבנה בקשת יצירת Tenant'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantPostQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'מבנה תגובת יצירת Tenant'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantPostResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'unexpected-param' | 'not-found' | 'unexpected-param' | 'sign-up-date-in-future' | 'payment-frequency-invalid' | 'cannot-change-payment-frequency' | 'name-invalid' | 'email-invalid' | 'email-taken' | 'no-package' | 'invalid-package' | 'unauthorized' | 'tenant-limit-reached' | 'cannot-move-tenant' | 'cannot-change-package' | 'invalid-billing-info'
    /** Included on failure. **/
    reason?: string
    tenant?: Tenant; // We return the complete created tenant on success.
}
[inline-code-end]

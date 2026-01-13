[api-resource-header-start name = 'Tenant'; route = 'PATCH /api/v1/tenants/:id'; creditsCost = 1; api-resource-header-end]

נקודת קצה API זו מספקת את היכולת לעדכן `Tenant` לפי `id`.

לעדכון `Tenant` יש את ההגבלות הבאות:

- הערכים הבאים לא ניתנים לעדכון:
  - `hasFlexPricing`
  - `lastBillingIssueReminderDate`
  - `flexLastBilledAmount`
  - `managedByTenantId`
- ה-`signUpDate` לא יכול להיות בעתיד.
- ה-`name` לא יכול להיות ארוך מ-`200 תווים`.
- ה-`email` לא יכול להיות ארוך מ-`300 תווים`.
- ה-`email` חייב להיות ייחודי בכל שוכרי FastComments.com.
- כאשר מגדירים `billingInfoValid` ל-`true`, יש לספק `billingInfo` באותה בקשה.
- לא ניתן לעדכן את ה-`packageId` המשויך לשוכר שלך עצמו.
- לא ניתן לעדכן את ה-`paymentFrequency` המשויך לשוכר שלך עצמו.

[inline-code-attrs-start title = 'דוגמת cURL ל-PATCH Tenant'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request PATCH \
  --url 'https://fastcomments.com/api/v1/tenants/xyz?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"name": "Some New Name",
}'
[inline-code-end]

[inline-code-attrs-start title = 'מבנה בקשת PATCH Tenant'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantPatchQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'מבנה תגובת PATCH Tenant'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface TenantPatchResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'unexpected-param' | 'not-found' | 'unexpected-param' | 'sign-up-date-in-future' | 'payment-frequency-invalid' | 'cannot-change-payment-frequency' | 'name-invalid' | 'email-invalid' | 'email-taken' | 'no-package' | 'invalid-package' | 'unauthorized' | 'tenant-limit-reached' | 'cannot-move-tenant' | 'cannot-change-package' | 'invalid-billing-info';
    /** Included on failure. **/
    reason?: string
}
[inline-code-end]

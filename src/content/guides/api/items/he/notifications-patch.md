[api-resource-header-start name = 'Notification'; route = 'PATCH /api/v1/notifications/:id'; creditsCost = 1; api-resource-header-end]

נקודת קצה API זו מספקת את היכולת לעדכן `Notification` לפי `id`.

לעדכון `Notification` יש את ההגבלות הבאות:

- אתה יכול לעדכן רק את השדות הבאים:
  - `viewed`
  - `optedOut`

[inline-code-attrs-start title = 'דוגמת cURL ל-PATCH Notification'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request PATCH \
  --url 'https://fastcomments.com/api/v1/notifications/xyz?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"viewed": false,
}'
[inline-code-end]

[inline-code-attrs-start title = 'מבנה בקשת PATCH Notification'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface NotificationPatchQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'מבנה תגובת PATCH Notification'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface NotificationPatchResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'unexpected-param' | 'unauthorized' | 'not-found';
    /** Included on failure. **/
    reason?: string
}
[inline-code-end]

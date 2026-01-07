[api-resource-header-start name = 'EmailTemplate'; route = 'POST /api/v1/email-templates/render'; creditsCost = 1; api-resource-header-end]

נקודת קצה API זו מספקת את היכולת להציג תצוגה מקדימה של תבניות אימייל.

[inline-code-attrs-start title = 'דוגמת cURL מינימלית ל-POST תצוגה מקדימה של EmailTemplate'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/email-templates/render?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
    "emailTemplateId": "comment-user-mention",
    "displayName": "I'm a custom template.",
    "ejs": "This is an @mention notification! My name is <%= comment.commenterName %>."
}'
[inline-code-end]

[inline-code-attrs-start title = 'מבנה בקשת POST EmailTemplate'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface EmailTemplatePostQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'מבנה תגובת POST EmailTemplate'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface EmailTemplatePostResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'unauthorized' | 'unexpected-param' | 'does-not-render';
    /** Included on failure. **/
    reason?: string
    /** The rendered HTML on success. **/
    html?: string
}
[inline-code-end]

[api-resource-header-start name = 'EmailTemplate'; route = 'POST /api/v1/email-templates/render'; creditsCost = 1; api-resource-header-end]

Тази API крайна точка предоставя възможност за предварителен преглед на имейл шаблони.

[inline-code-attrs-start title = 'Пример за минимален POST за предварителен преглед на EmailTemplate с cURL'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
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

[inline-code-attrs-start title = 'Структура на заявката за POST на EmailTemplate'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface EmailTemplatePostQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Структура на отговора за POST на EmailTemplate'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

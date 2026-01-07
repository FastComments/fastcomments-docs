[api-resource-header-start name = 'EmailTemplate'; route = 'POST /api/v1/email-templates'; creditsCost = 1; api-resource-header-end]

This API endpoint provides the ability to create email templates.

Notes:

- You can't have multiple templates with the same `emailTemplateId` with the same domain.
- But you can have a wildcard template (`domain` = `*` and a domain specific template for the same `emailTemplateId`).
- Specifying `domain` is only relevant if you have different domains, or want to use specific templates for testing (`domain` set to `localhost` etc).
- If you do specify `domain` it must match a `DomainConfig`. On error a list of valid domains is provided.
- The template syntax is EJS and is rendered with a 500ms timeout. P99 for rendering is <5ms, so if you hit 500ms something is wrong.
- **Your template must render with your given `testData`** to save. Render errors are aggregated and reported on in the dashboard (soon available via API). 

The minimum data required to add a template is as follows:

[inline-code-attrs-start title = 'Minimum EmailTemplate POST cURL Example'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/email-templates?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
    "emailTemplateId": "comment-user-mention",
    "displayName": "I'm a custom template.",
    "ejs": "This is an @mention notification! My name is <%= comment.commenterName %>."
}'
[inline-code-end]

You may want to have templates per-site, in which case you define `domain`:

[inline-code-attrs-start title = 'EmailTemplate POST cURL Example'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/email-templates?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
    "emailTemplateId": "comment-user-mention",
    "displayName": "I'm a custom template.",
    "ejs": "This is some email content!",
    "domain": "somespecificsite.com",
}'
[inline-code-end]

[inline-code-attrs-start title = 'EmailTemplate POST Request Structure'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface EmailTemplatePostQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'EmailTemplate POST Response Structure'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface EmailTemplatePostResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'unauthorized' | 'unexpected-param' | 'invalid-email-template-id' | 'domain-invalid' | 'duplicate' | 'does-not-render';
    /** Included on failure. **/
    reason?: string
    /** The created template. **/
    emailTemplate?: EmailTemplate
}
[inline-code-end]

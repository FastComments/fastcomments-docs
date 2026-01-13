## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | Yes |  |

## Response

Returns: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/FlagCommentPublic200Response.ts)

## Example

[inline-code-attrs-start title = 'deleteEmailTemplate Example'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_4b2f6a-4b2f6a2d";
const templateId: string = "email_template_9f8b7c3e";
const result: FlagCommentPublic200Response = await deleteEmailTemplate(tenantId, templateId);
const status: APIStatus | undefined = result?.status
[inline-code-end]
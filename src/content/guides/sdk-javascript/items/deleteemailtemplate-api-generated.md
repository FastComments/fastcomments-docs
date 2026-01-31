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
(async () => {
  const tenantId: string = 'd3f5c8e2-9b1f-4a2b-a3d4-5f67890abcde';
  const incomingTemplateId: string | undefined = undefined; // optional source
  const id: string = incomingTemplateId ?? 'e7a9c1d2-8f3b-47a0-9f1b-2c3d4e5f6789';
  const response: FlagCommentPublic200Response = await deleteEmailTemplate(tenantId, id);
  console.log(response);
})();
[inline-code-end]

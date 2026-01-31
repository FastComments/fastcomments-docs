## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | Yes |  |

## Response

Returns: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/FlagCommentPublic200Response.ts)

## Example

[inline-code-attrs-start title = 'deleteQuestionConfig Example'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_3a9f2c4b';
const id: string = 'qc_8f7b6c1a';
const confirmDeletion: boolean | undefined = true;

if (confirmDeletion) {
  const result: FlagCommentPublic200Response = await deleteQuestionConfig(tenantId, id);
}
[inline-code-end]

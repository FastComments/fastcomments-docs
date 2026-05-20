## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tag | string | Yes |  |
| tenantId | string | No |  |
| deleteHashTagRequest | DeleteHashTagRequest | No |  |

## Response

Returns: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/FlagCommentPublic200Response.ts)

## Example

[inline-code-attrs-start title = 'deleteHashTag Example'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tag: string = "outdated-policy";
const tenantId: string = "acme_corp_42";
const deleteHashTagRequest: DeleteHashTagRequest = {};
const result: FlagCommentPublic200Response = await deleteHashTag(tag, tenantId, deleteHashTagRequest);
[inline-code-end]

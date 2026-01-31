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
const tag: string = 'breaking-news';
const tenantId: string = 'media-corp-123';
const deleteRequest: DeleteHashTagRequest = {
  reason: 'duplicate_tag',
  requestedBy: 'moderator@news-site.com',
  removeAssociatedComments: true,
  requestedAt: new Date().toISOString()
};
const response: FlagCommentPublic200Response = await deleteHashTag(tag, tenantId, deleteRequest);
[inline-code-end]

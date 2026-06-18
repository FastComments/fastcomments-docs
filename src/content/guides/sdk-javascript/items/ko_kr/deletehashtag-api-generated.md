---
## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tag | string | 예 |  |
| tenantId | string | 아니요 |  |
| deleteHashTagRequest | DeleteHashTagRequest | 아니요 |  |

## Response

반환: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/FlagCommentPublic200Response.ts)

## Example

[inline-code-attrs-start title = 'deleteHashTag 예제'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tag: string = "breaking-news";
const tenantId: string = "tenant_72a1";
const deleteHashTagRequest: DeleteHashTagRequest = {
  reason: "consolidate-duplicates",
  requestedBy: "moderator@dailypress.com",
  forceDelete: true
};
const result: FlagCommentPublic200Response = await deleteHashTag(tag, tenantId, deleteHashTagRequest);
[inline-code-end]

---
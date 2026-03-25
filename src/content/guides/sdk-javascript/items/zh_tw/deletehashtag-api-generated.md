---
## 參數

| 名稱 | 類型 | 必填 | 說明 |
|------|------|----------|-------------|
| tag | string | 是 |  |
| tenantId | string | 否 |  |
| deleteHashTagRequest | DeleteHashTagRequest | 否 |  |

## 回應

回傳: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/FlagCommentPublic200Response.ts)

## 範例

[inline-code-attrs-start title = 'deleteHashTag 範例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tag: string = "breaking-news";
const tenantId: string = "tenant_12345";
const deleteRequest: DeleteHashTagRequest = { initiatedBy: "moderator@newsorg.com", purgeAllOccurrences: true };
const result: FlagCommentPublic200Response = await deleteHashTag(tag, tenantId, deleteRequest);
[inline-code-end]

---
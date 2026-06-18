## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tag | string | はい |  |
| tenantId | string | いいえ |  |
| deleteHashTagRequest | DeleteHashTagRequest | いいえ |  |

## レスポンス

返却値: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/FlagCommentPublic200Response.ts)

## 例

[inline-code-attrs-start title = 'deleteHashTag の例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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
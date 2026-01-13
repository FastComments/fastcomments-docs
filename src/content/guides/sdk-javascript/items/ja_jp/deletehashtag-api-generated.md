## パラメータ

| Name | Type | 必須 | 説明 |
|------|------|------|-------------|
| tag | string | はい |  |
| tenantId | string | いいえ |  |
| deleteHashTagRequest | DeleteHashTagRequest | いいえ |  |

## レスポンス

戻り値: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/FlagCommentPublic200Response.ts)

## 例

[inline-code-attrs-start title = 'deleteHashTag の例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tag: string = 'breaking-news';
const tenantId: string = 'tenant_42';
const deleteReq: DeleteHashTagRequest = { removedBy: 'moderator_jane', reason: 'off-topic for this community', deleteAssociatedComments: true } as DeleteHashTagRequest;
const result: FlagCommentPublic200Response = await deleteHashTag(tag, tenantId, deleteReq);
[inline-code-end]

---
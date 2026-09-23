## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenantId | string | はい |  |
| commentId | string | はい |  |
| editKey | string | いいえ |  |
| sso | string | いいえ |  |

## レスポンス

返り値: [`PublicAPIGetCommentTextResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/PublicAPIGetCommentTextResponse.ts)

## 例

[inline-code-attrs-start title = 'getCommentText の例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "acme-corp-tenant";
const commentId: string = "comment-987654321";
const editKey: string = "edit-key-abc123";
const sso: string = "sso-token-xyz789";

const commentWithEdit: PublicAPIGetCommentTextResponse = await getCommentText(
  tenantId,
  commentId,
  editKey,
  sso
);

const commentBasic: PublicAPIGetCommentTextResponse = await getCommentText(
  tenantId,
  commentId
);
[inline-code-end]

---
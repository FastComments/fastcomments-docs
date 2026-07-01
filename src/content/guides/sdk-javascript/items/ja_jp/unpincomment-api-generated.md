## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenantId | string | はい |  |
| commentId | string | はい |  |
| broadcastId | string | はい |  |
| sso | string | いいえ |  |

## レスポンス

返り値: [`UnPinCommentResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/UnPinCommentResponse.ts)

## 例

[inline-code-attrs-start title = 'unPinComment の例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = "tenant-001"
  const commentId: string = "comment-123"
  const broadcastId: string = "broadcast-456"
  const sso: string = "sso-token-xyz"

  const resultWithSso: UnPinCommentResponse = await unPinComment(tenantId, commentId, broadcastId, sso)
  const resultWithoutSso: UnPinCommentResponse = await unPinComment(tenantId, commentId, broadcastId)

  console.log(resultWithSso, resultWithoutSso)
})()
[inline-code-end]

---
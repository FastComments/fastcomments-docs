## Paramètres

| Nom | Type | Obligatoire | Description |
|------|------|----------|-------------|
| tenantId | string | Oui |  |
| commentId | string | Oui |  |
| broadcastId | string | Oui |  |
| sso | string | Non |  |

## Réponse

Retourne : [`UnPinCommentResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/UnPinCommentResponse.ts)

## Exemple

[inline-code-attrs-start title = 'unPinComment Exemple'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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
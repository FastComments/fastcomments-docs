## Paramètres

| Name | Type | Obligatoire | Description |
|------|------|------------|-------------|
| tenantId | string | Oui |  |
| id | string | Non |  |
| unBlockFromCommentParams | UnBlockFromCommentParams | Non |  |
| userId | string | Non |  |
| anonUserId | string | Non |  |

## Réponse

Renvoie : [`Option[UnblockSuccess]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_unblock_success.nim)

## Exemple

[inline-code-attrs-start title = 'Exemple : unBlockUserFromComment'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.unBlockUserFromComment(
  tenantId = "my-tenant-123",
  id = "comment-9f3b2a",
  unBlockFromCommentParams = UnBlockFromCommentParams(),
  userId = "user-1024",
  anonUserId = "anon-77b"
)

if response.isSome:
  let unblockResult = response.get()
  echo unblockResult
else:
  echo "Unblock failed"
[inline-code-end]

---
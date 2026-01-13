## Parametri

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Sì |  |
| id | string | No |  |
| editKey | string | No |  |

## Risposta

Restituisce: [`Option[DeleteCommentVote_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_delete_comment_vote200response.nim)

## Esempio

[inline-code-attrs-start title = 'Esempio di deleteVote'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.deleteVote(tenantId = "my-tenant-123", id = "", editKey = "")
if response.isSome:
  let deleted = response.get()
  discard deleted
[inline-code-end]

---
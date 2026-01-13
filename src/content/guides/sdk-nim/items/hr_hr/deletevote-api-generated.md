## Parametri

| Naziv | Tip | Obvezno | Opis |
|------|------|----------|-------------|
| tenantId | string | Da |  |
| id | string | Ne |  |
| editKey | string | Ne |  |

## Odgovor

Vraća: [`Option[DeleteCommentVote_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_delete_comment_vote200response.nim)

## Primjer

[inline-code-attrs-start title = 'Primjer deleteVote'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.deleteVote(tenantId = "my-tenant-123", id = "", editKey = "")
if response.isSome:
  let deleted = response.get()
  discard deleted
[inline-code-end]

---
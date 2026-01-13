## Parametry

| Nazwa | Typ | Wymagane | Opis |
|------|------|----------|-------------|
| tenantId | string | Tak |  |
| id | string | Nie |  |
| deleteComments | string | Nie |  |
| commentDeleteMode | string | Nie |  |

## Odpowiedź

Zwraca: [`Option[FlagCommentPublic_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_flag_comment_public200response.nim)

## Przykład

[inline-code-attrs-start title = 'Przykład deleteTenantUser'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.deleteTenantUser(tenantId = "my-tenant-123", id = "user-456", deleteComments = "", commentDeleteMode = "")
if response.isSome:
  let flagResp = response.get()
  echo flagResp
[inline-code-end]

---
## Parametri

| Nome | Tipo | Richiesto | Descrizione |
|------|------|----------|-------------|
| tenantId | string | Sì |  |
| id | string | No |  |

## Risposta

Restituisce: [`Option[FlagCommentPublic_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_flag_comment_public200response.nim)

## Esempio

[inline-code-attrs-start title = 'Esempio di deleteTenantPackage'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.deleteTenantPackage(tenantId = "my-tenant-123", id = "pkg-789")
if response.isSome:
  let deleted = response.get()
  echo "Successfully deleted tenant package"
[inline-code-end]
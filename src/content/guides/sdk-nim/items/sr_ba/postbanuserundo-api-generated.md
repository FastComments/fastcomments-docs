## Parametri

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|------|
| tenantId | string | Yes |  |
| banUserUndoParams | BanUserUndoParams | No |  |
| sso | string = "" | No |  |

## Odgovor

Vraća: [`Option[APIEmptyResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_api_empty_response.nim)

## Primjer

[inline-code-attrs-start title = 'postBanUserUndo primjer'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (apiResp, httpResp) = client.postBanUserUndo(
  tenantId = "my-tenant-123",
  banUserUndoParams = BanUserUndoParams(userId = "user-456"),
  sso = ""
)

if apiResp.isSome:
  let _ = apiResp.get()
[inline-code-end]
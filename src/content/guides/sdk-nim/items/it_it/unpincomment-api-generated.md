## Parametri

| Name | Type | Obbligatorio | Descrizione |
|------|------|--------------|-------------|
| tenantId | string | Sì |  |
| commentId | string | Sì |  |
| broadcastId | string | No |  |
| sso | string | No |  |

## Risposta

Restituisce: [`Option[PinComment_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_pin_comment200response.nim)

## Esempio

[inline-code-attrs-start title = 'Esempio di unPinComment'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.unPinComment(tenantId = "my-tenant-123", commentId = "cmt-9f8b7a6", broadcastId = "", sso = "")
if response.isSome:
  let pinResp = response.get()
  echo "Unpinned comment successfully"
else:
  echo "Failed to unpin comment; HTTP response: ", httpResponse
[inline-code-end]

---
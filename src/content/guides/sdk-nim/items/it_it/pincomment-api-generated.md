## Parametri

| Nome | Tipo | Obbligatorio | Descrizione |
|------|------|--------------|-------------|
| tenantId | string | Sì |  |
| commentId | string | Sì |  |
| broadcastId | string | No |  |
| sso | string | No |  |

## Risposta

Restituisce: [`Option[ChangeCommentPinStatusResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_change_comment_pin_status_response.nim)

## Esempio

[inline-code-attrs-start title = 'Esempio di pinComment'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.pinComment(tenantId = "my-tenant-123", commentId = "cmt-98765", broadcastId = "", sso = "")
if response.isSome:
  let pinnedResp = response.get()
  echo "Pin status updated for comment cmt-98765"
else:
  echo "No response received"
[inline-code-end]

---
## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|--------------|---------------|
| tenantId | string | Ja |  |
| options | GetApiCommentsOptions | Nein |  |

## Antwort

Rückgabe: [`Option[ModerationAPIGetCommentsResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_moderation_api_get_comments_response.nim)

## Beispiel

[inline-code-attrs-start title = 'getApiComments Beispiel'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (maybeResp, httpResp) = client.getApiComments(tenantId = "my-tenant-123", options = GetApiCommentsOptions())
if maybeResp.isSome:
  let resp = maybeResp.get()
  # Weiterverarbeitung kann mit `resp` durchgeführt werden
[inline-code-end]

---
## Parametri

| Ime | Tip | Obavezno | Opis |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| options | GetApiCommentsOptions | No |  |

## Odgovor

Vraća: [`Option[ModerationAPIGetCommentsResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_moderation_api_get_comments_response.nim)

## Primer

[inline-code-attrs-start title = 'getApiComments Primer'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (maybeResp, httpResp) = client.getApiComments(tenantId = "my-tenant-123", options = GetApiCommentsOptions())
if maybeResp.isSome:
  let resp = maybeResp.get()
  # dalja obrada se može izvršiti uz `resp`
[inline-code-end]
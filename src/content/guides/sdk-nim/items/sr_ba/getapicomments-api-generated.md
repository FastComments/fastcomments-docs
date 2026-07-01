## Parametri

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|------|
| tenantId | string | Da |  |
| options | GetApiCommentsOptions | Ne |  |

## Odgovor

Vraća: [`Option[ModerationAPIGetCommentsResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_moderation_api_get_comments_response.nim)

## Primjer

[inline-code-attrs-start title = 'getApiComments Primjer'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (maybeResp, httpResp) = client.getApiComments(tenantId = "my-tenant-123", options = GetApiCommentsOptions())
if maybeResp.isSome:
  let resp = maybeResp.get()
  # daljnja obrada se može izvršiti sa `resp`
[inline-code-end]

---
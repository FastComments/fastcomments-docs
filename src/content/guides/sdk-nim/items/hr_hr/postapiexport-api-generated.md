## Parametri

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|-------------|
| textSearch | string | Ne |  |
| byIPFromComment | string | Ne |  |
| filters | string | Ne |  |
| searchFilters | string | Ne |  |
| sorts | string | Ne |  |
| sso | string | Ne |  |

## Odgovor

Vraća: [`Option[ModerationExportResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_moderation_export_response.nim)

## Primjer

[inline-code-attrs-start title = 'postApiExport Primjer'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.postApiExport(
  textSearch = "offensive language and spam",
  byIPFromComment = "203.0.113.45",
  filters = "{\"status\":\"pending\",\"severity\":\"high\"}",
  searchFilters = "authorEmail:editor@news-site.com",
  sorts = "-createdAt",
  sso = "sso-session-token-9f8b7c"
)
if response.isSome:
  let exportResp = response.get()
  echo "Moderation export received:", exportResp
else:
  echo "No export returned, HTTP status:", httpResponse.status.code
[inline-code-end]
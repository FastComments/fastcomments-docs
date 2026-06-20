---
## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|----------|-------------|
| textSearch | string | Nein |  |
| byIPFromComment | string | Nein |  |
| filter | string | Nein |  |
| searchFilters | string | Nein |  |
| demo | bool | Nein |  |
| sso | string | Nein |  |

## Antwort

Gibt zurück: [`Option[ModerationAPICountCommentsResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_moderation_api_count_comments_response.nim)

## Beispiel

[inline-code-attrs-start title = 'getCount Beispiel'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getCount(
  textSearch = "climate change",
  byIPFromComment = "203.0.113.5",
  filter = "status:approved",
  searchFilters = "author:john.doe@example.com;tag:opinion",
  demo = false,
  sso = "sso_eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9"
)
if response.isSome:
  let countResp = response.get()
  discard countResp
  echo "Count response received"
else:
  echo "No count data returned"
[inline-code-end]

---
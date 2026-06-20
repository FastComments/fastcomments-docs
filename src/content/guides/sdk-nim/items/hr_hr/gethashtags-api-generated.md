---
## Parametri

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|-------------|
| tenantId | string | Da |  |
| page | float64 | Ne |  |

## Odgovor

Vraća: [`Option[GetHashTagsResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_hash_tags_response.nim)

## Primjer

[inline-code-attrs-start title = 'getHashTags Primjer'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getHashTags(tenantId = "news-portal-987", page = 2.0)
if response.isSome:
  let tagsResp = response.get()
  echo "Received hashtags response"
else:
  echo "No hashtags returned"
[inline-code-end]

---
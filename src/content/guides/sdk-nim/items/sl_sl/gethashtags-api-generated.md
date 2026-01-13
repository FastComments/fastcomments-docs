---
## Parametri

| Ime | Tip | Obvezno | Opis |
|------|------|----------|-------------|
| tenantId | string | Da |  |
| page | float64 | Ne |  |

## Odgovor

Vrne: [`Option[GetHashTags_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_hash_tags200response.nim)

## Primer

[inline-code-attrs-start title = 'Primer getHashTags'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getHashTags(tenantId = "my-tenant-123", page = 1.0)
if response.isSome:
  let tags = response.get()
  for t in tags:
    echo t
else:
  echo "no hashtags found"
[inline-code-end]

---
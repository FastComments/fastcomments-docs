## Paramètres

| Nom | Type | Obligatoire | Description |
|------|------|----------|-------------|
| tenantId | string | Oui |  |
| page | float64 | Non |  |

## Réponse

Renvoie: [`Option[GetHashTags_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_hash_tags200response.nim)

## Exemple

[inline-code-attrs-start title = 'Exemple de getHashTags'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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
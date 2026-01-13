## Paramètres

| Nom | Type | Obligatoire | Description |
|------|------|----------|-------------|
| tag | string | Non |  |
| tenantId | string | Oui |  |
| updateHashTagBody | UpdateHashTagBody | Non |  |

## Réponse

Retourne: [`Option[PatchHashTag_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_patch_hash_tag200response.nim)

## Exemple

[inline-code-attrs-start title = 'Exemple de patchHashTag'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.patchHashTag(tag = "politics", tenantId = "my-tenant-123", updateHashTagBody = UpdateHashTagBody())

if response.isSome:
  let updated = response.get()
  echo "Hashtag updated successfully"
else:
  echo "Failed to update hashtag, status:", httpResponse.status
[inline-code-end]

---
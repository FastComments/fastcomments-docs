Lister les pages d'un locataire. Utilisé par le client de bureau FChat pour remplir sa liste de salles.
Requiert que `enableFChat` soit true dans la configuration personnalisée résolue pour chaque page.
Les pages nécessitant SSO sont filtrées en fonction de l'accès aux groupes de l'utilisateur demandeur.

## Paramètres

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Oui |  |
| cursor | string | Non |  |
| limit | int | Non |  |
| q | string | Non |  |
| sortBy | PagesSortBy | Non |  |
| hasComments | bool | Non |  |

## Réponse

Retourne : [`Option[GetPublicPagesResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_public_pages_response.nim)

## Exemple

[inline-code-attrs-start title = 'Exemple getPagesPublic'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getPagesPublic(
  tenantId = "my-tenant-123",
  cursor = "",
  limit = 0,
  q = "",
  sortBy = PagesSortBy(0),
  hasComments = false
)

if response.isSome:
  let pages = response.get()
  echo "Retrieved public pages: ", $pages
else:
  echo "No pages returned, HTTP status: ", $httpResponse.status
[inline-code-end]
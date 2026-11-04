Liste les pages d'un tenant. Utilisé par le client de bureau FChat pour remplir sa liste de salons.
Nécessite que `enableFChat` soit à true dans la configuration personnalisée résolue pour chaque page.
Les pages nécessitant une authentification SSO sont filtrées en fonction des accès de groupe de l'utilisateur à l'origine de la requête.

## Paramètres

| Nom | Type | Requis | Description |
|------|------|----------|-------------|
| tenantId | string | Oui |  |
| options | GetPagesPublicOptions | Non |  |

## Réponse

Retourne : [`Option[GetPublicPagesResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_public_pages_response.nim)

## Exemple

[inline-code-attrs-start title = 'Exemple de getPagesPublic'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getPagesPublic(tenantId = "my-tenant-123", options = GetPagesPublicOptions())
if response.isSome:
  let pages = response.get()
  echo pages
[inline-code-end]
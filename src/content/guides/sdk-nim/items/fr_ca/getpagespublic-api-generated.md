Lister les pages pour un locataire. Utilisé par le client de bureau FChat pour remplir sa liste de salles.  
Nécessite que `enableFChat` soit vrai dans la configuration personnalisée résolue pour chaque page.  
Les pages qui nécessitent SSO sont filtrées en fonction de l’accès aux groupes de l’utilisateur demandeur.

## Paramètres

| Nom | Type | Obligatoire | Description |
|------|------|------------|-------------|
| tenantId | string | Oui |  |
| options | GetPagesPublicOptions | Non |  |

## Réponse

Retourne : [`Option[GetPublicPagesResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_public_pages_response.nim)

## Exemple

[inline-code-attrs-start title = 'Exemple getPagesPublic'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getPagesPublic(tenantId = "my-tenant-123", options = GetPagesPublicOptions())
if response.isSome:
  let pages = response.get()
  echo pages
[inline-code-end]
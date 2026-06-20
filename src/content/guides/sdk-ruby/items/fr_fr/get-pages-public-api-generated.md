Lister les pages pour un tenant. Utilisé par le client de bureau FChat pour remplir sa liste de salons.
Nécessite que `enableFChat` soit true dans la configuration personnalisée résolue pour chaque page.
Les pages qui nécessitent un SSO sont filtrées en fonction de l'accès par groupe de l'utilisateur demandeur.

## Paramètres

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Oui |  |
| cursor | string | query | Non | Curseur opaque de pagination renvoyé comme `nextCursor` par une requête précédente. Lié au même `sortBy`. |
| limit | integer | query | Non | 1..200, par défaut 50 |
| q | string | query | Non | Filtre optionnel de préfixe de titre insensible à la casse. |
| sortBy | string | query | Non | Ordre de tri. `updatedAt` (par défaut, les plus récents en premier), `commentCount` (le plus de commentaires en premier) ou `title` (alphabétique). |
| hasComments | boolean | query | Non | Si true, ne renvoie que les pages ayant au moins un commentaire. |

## Réponse

Renvoie : [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/get_public_pages_response.rb)

## Exemple

[inline-code-attrs-start title = 'Exemple get_pages_public'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::PublicApi.new
tenant_id = 'tenant_id_example' # String | 
opts = {
  cursor: 'cursor_example', # String | Curseur opaque de pagination renvoyé comme `nextCursor` par une requête précédente. Lié au même `sortBy`.
  limit: 56, # Integer | 1..200, par défaut 50
  q: 'q_example', # String | Filtre optionnel de préfixe de titre insensible à la casse.
  sort_by: FastCommentsClient::PagesSortBy::UPDATED_AT, # PagesSortBy | Ordre de tri. `updatedAt` (par défaut, les plus récents en premier), `commentCount` (le plus de commentaires en premier), ou `title` (alphabétique).
  has_comments: true # Boolean | Si true, ne renvoie que les pages avec au moins un commentaire.
}

begin
  
  result = api_instance.get_pages_public(tenant_id, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling PublicApi->get_pages_public: #{e}"
end
[inline-code-end]
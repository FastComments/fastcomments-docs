Liste des pages pour un locataire. Utilisé par le client de bureau FChat pour remplir sa liste de salons.  
Nécessite que `enableFChat` soit vrai dans la configuration personnalisée résolue pour chaque page.  
Les pages qui nécessitent SSO sont filtrées en fonction des droits de groupe de l'utilisateur demandeur.

## Parameters

| Nom | Type | Emplacement | Obligatoire | Description |
|------|------|------------|-------------|-------------|
| tenantId | string | path | Oui |  |
| cursor | string | query | Non | Curseur de pagination opaque renvoyé comme `nextCursor` lors d'une requête précédente. Lié au même `sortBy`. |
| limit | integer | query | Non | 1..200, défaut 50 |
| q | string | query | Non | Filtre de préfixe de titre insensible à la casse, optionnel. |
| sortBy | string | query | Non | Ordre de tri. `updatedAt` (défaut, le plus récent d'abord), `commentCount` (le plus de commentaires d'abord), ou `title` (alphabétique). |
| hasComments | boolean | query | Non | Si vrai, ne retourner que les pages contenant au moins un commentaire. |

## Response

Returns: `GetPublicPagesResponse`

## Example

[inline-code-attrs-start title = 'Exemple getPagesPublic'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = PublicApi();
final tenantId = tenantId_example; // String | 
final cursor = cursor_example; // String | Curseur de pagination opaque renvoyé comme `nextCursor` lors d'une requête précédente. Lié au même `sortBy`.
final limit = 56; // int | 1..200, default 50
final q = q_example; // String | Filtre de préfixe de titre insensible à la casse, optionnel.
final sortBy = ; // PagesSortBy | Ordre de tri. `updatedAt` (défaut, le plus récent d'abord), `commentCount` (le plus de commentaires d'abord), ou `title` (alphabétique).
final hasComments = true; // bool | Si vrai, ne retourner que les pages contenant au moins un commentaire.

try {
    final result = api_instance.getPagesPublic(tenantId, GetPagesPublicOptions(cursor: cursor, limit: limit, q: q, sortBy: sortBy, hasComments: hasComments));
    print(result);
} catch (e) {
    print('Exception when calling PublicApi->getPagesPublic: $e\n');
}
[inline-code-end]

---
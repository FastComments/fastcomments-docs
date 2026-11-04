Informations utilisateur en masse pour un tenant. Étant donné des userIds, renvoie les informations d'affichage de User / SSOUser.
Utilisé par le widget de commentaires pour enrichir les utilisateurs qui viennent d'apparaître via un événement de présence.
Pas de contexte de page : la confidentialité est appliquée de manière uniforme (les profils privés sont masqués).

## Paramètres

| Nom | Type | Emplacement | Requis | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Oui |  |
| ids | string | query | Oui | userIds délimités par des virgules. |

## Réponse

Renvoie : `PageUsersInfoResponse`

## Exemple

[inline-code-attrs-start title = 'Exemple de getUsersInfo'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = PublicApi();
final tenantId = tenantId_example; // String | 
final ids = ids_example; // String | userIds délimités par des virgules.

try {
    final result = api_instance.getUsersInfo(tenantId, ids);
    print(result);
} catch (e) {
    print('Exception when calling PublicApi->getUsersInfo: $e\n');
}
[inline-code-end]
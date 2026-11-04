Commentateurs passés sur la page qui ne sont PAS actuellement en ligne. Triés par displayName.
À utiliser après avoir épuisé /users/online pour afficher une section « Membres ».
Pagination par curseur sur commenterName : le serveur parcourt l'index partiel {tenantId, urlId, commenterName}
depuis afterName vers l'avant via $gt, sans coût de $skip.

## Paramètres

| Nom | Type | Emplacement | Requis | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Oui |  |
| urlId | string | query | Oui | Identifiant d'URL de la page (nettoyé côté serveur). |
| afterName | string | query | Non | Curseur : passez nextAfterName de la réponse précédente. |
| afterUserId | string | query | Non | Départage du curseur : passez nextAfterUserId de la réponse précédente. Requis lorsque afterName est défini pour éviter que les entrées avec le même nom ne soient perdues. |

## Réponse

Renvoie : `PageUsersOfflineResponse`

## Exemple

[inline-code-attrs-start title = 'Exemple de getOfflineUsers'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = PublicApi();
final tenantId = tenantId_example; // String | 
final urlId = urlId_example; // String | Identifiant d'URL de la page (nettoyé côté serveur).
final afterName = afterName_example; // String | Curseur : passez nextAfterName de la réponse précédente.
final afterUserId = afterUserId_example; // String | Départage du curseur : passez nextAfterUserId de la réponse précédente. Requis lorsque afterName est défini pour éviter que les entrées avec le même nom ne soient perdues.

try {
    final result = api_instance.getOfflineUsers(tenantId, urlId, GetOfflineUsersOptions(afterName: afterName, afterUserId: afterUserId));
    print(result);
} catch (e) {
    print('Exception when calling PublicApi->getOfflineUsers: $e\n');
}
[inline-code-end]
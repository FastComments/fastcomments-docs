Visiteurs actuellement en ligne d’une page : les personnes dont la session websocket est abonnée à la page en ce moment.  
Renvoie anonCount + totalCount (abonnés au niveau de la salle, y compris les spectateurs anonymes que nous n’énumérons pas).

## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| urlId | string | query | Yes | Identifiant d’URL de la page (nettoyé côté serveur). |
| afterName | string | query | No | Curseur : fournir nextAfterName de la réponse précédente. |
| afterUserId | string | query | No | Déterminateur de tie du curseur : fournir nextAfterUserId de la réponse précédente. Requis lorsque afterName est défini afin que les égalités de nom ne suppriment pas d’entrées. |

## Response

Returns: `PageUsersOnlineResponse`

## Example

[inline-code-attrs-start title = 'Exemple getOnlineUsers'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = PublicApi();
final tenantId = tenantId_example; // String | 
final urlId = urlId_example; // String | Identifiant d’URL de la page (nettoyé côté serveur).
final afterName = afterName_example; // String | Curseur : fournir nextAfterName de la réponse précédente.
final afterUserId = afterUserId_example; // String | Déterminateur de tie du curseur : fournir nextAfterUserId de la réponse précédente. Requis lorsque afterName est défini afin que les égalités de nom ne suppriment pas d’entrées.

try {
    final result = api_instance.getOnlineUsers(tenantId, urlId, GetOnlineUsersOptions(afterName: afterName, afterUserId: afterUserId));
    print(result);
} catch (e) {
    print('Exception when calling PublicApi->getOnlineUsers: $e\n');
}
[inline-code-end]
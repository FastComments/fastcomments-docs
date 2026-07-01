Currently-online viewers of a page : personnes dont la session websocket est abonnée à la page en ce moment.  
Renvoie anonCount + totalCount (abonnés à l’ensemble de la salle, y compris les visionneurs anonymes que nous n’énumérons pas).

## Paramètres

| Nom | Type | Emplacement | Obligatoire | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Oui |  |
| urlId | string | query | Oui | Identifiant d\'URL de page (nettoyé côté serveur). |
| afterName | string | query | Non | Curseur : transmettez nextAfterName depuis la réponse précédente. |
| afterUserId | string | query | Non | Curseur de résolution d\'égalité : transmettez nextAfterUserId depuis la réponse précédente. Requis lorsque afterName est défini afin que les égalités de nom ne suppriment pas d\'entrées. |

## Réponse

Renvoie : `PageUsersOnlineResponse`

## Exemple

[inline-code-attrs-start title = 'Exemple getOnlineUsers'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = PublicApi();
final tenantId = tenantId_example; // String | 
final urlId = urlId_example; // String | Identifiant d\'URL de page (nettoyé côté serveur).
final afterName = afterName_example; // String | Curseur : transmettez nextAfterName depuis la réponse précédente.
final afterUserId = afterUserId_example; // String | Curseur de résolution d\'égalité : transmettez nextAfterUserId depuis la réponse précédente. Requis lorsque afterName est défini afin que les égalités de nom ne suppriment pas d\'entrées.

try {
    final result = api_instance.getOnlineUsers(tenantId, urlId, GetOnlineUsersOptions(afterName: afterName, afterUserId: afterUserId));
    print(result);
} catch (e) {
    print('Exception when calling PublicApi->getOnlineUsers: $e\n');
}
[inline-code-end]
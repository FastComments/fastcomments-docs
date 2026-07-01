Past commenters on the page who are NOT currently online. Sorted by displayName.  
Use this after exhausting /users/online to render a "Members" section.  
Cursor pagination on commenterName: server walks the partial {tenantId, urlId, commenterName} index from afterName forward via $gt, no $skip cost.

## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| urlId | string | query | Yes | Identifiant d'URL de la page (nettoyé côté serveur). |
| afterName | string | query | No | Curseur : passez nextAfterName depuis la réponse précédente. |
| afterUserId | string | query | No | Débrayage de curseur : passez nextAfterUserId depuis la réponse précédente. Obligatoire lorsque afterName est défini afin que les égalités de nom ne suppriment pas d'entrées. |

## Response

Returns: `PageUsersOfflineResponse`

## Example

[inline-code-attrs-start title = 'getOfflineUsers Exemple'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = PublicApi();
final tenantId = tenantId_example; // String | 
final urlId = urlId_example; // String | Identifiant d'URL de la page (nettoyé côté serveur).
final afterName = afterName_example; // String | Curseur : passez nextAfterName depuis la réponse précédente.
final afterUserId = afterUserId_example; // String | Débrayage de curseur : passez nextAfterUserId depuis la réponse précédente. Obligatoire lorsque afterName est défini afin que les égalités de nom ne suppriment pas d'entrées.

try {
    final result = api_instance.getOfflineUsers(tenantId, urlId, GetOfflineUsersOptions(afterName: afterName, afterUserId: afterUserId));
    print(result);
} catch (e) {
    print('Exception when calling PublicApi->getOfflineUsers: $e\n');
}
[inline-code-end]
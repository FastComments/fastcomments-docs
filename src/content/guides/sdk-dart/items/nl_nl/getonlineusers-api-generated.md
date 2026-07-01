Momenteel online kijkers van een pagina: personen waarvan de websocketsessie momenteel op de pagina is geabonneerd.  
Retourneert anonCount + totalCount (kamerbrede abonnees, inclusief anonieme kijkers die we niet opsommen).

## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| urlId | string | query | Yes | Pagina URL-identificatie (server-side opgeschoond). |
| afterName | string | query | No | Cursor: geef nextAfterName door van de vorige respons. |
| afterUserId | string | query | No | Cursor tie‑breaker: geef nextAfterUserId door van de vorige respons. Vereist wanneer afterName is ingesteld zodat naam‑ties geen entries laten vallen. |

## Response

Retourneert: `PageUsersOnlineResponse`

## Example

[inline-code-attrs-start title = 'getOnlineUsers Voorbeeld'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = PublicApi();
final tenantId = tenantId_example; // String | 
final urlId = urlId_example; // String | Pagina URL-identificatie (server-side opgeschoond).
final afterName = afterName_example; // String | Cursor: geef nextAfterName door van de vorige respons.
final afterUserId = afterUserId_example; // String | Cursor tie‑breaker: geef nextAfterUserId door van de vorige respons. Vereist wanneer afterName is ingesteld zodat naam‑ties geen entries laten vallen.

try {
    final result = api_instance.getOnlineUsers(tenantId, urlId, GetOnlineUsersOptions(afterName: afterName, afterUserId: afterUserId));
    print(result);
} catch (e) {
    print('Exception when calling PublicApi->getOnlineUsers: $e\n');
}
[inline-code-end]
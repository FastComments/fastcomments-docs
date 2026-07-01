Currently-online viewers of a page: people whose websocket session is subscribed to the page right now.  
Restituisce anonCount + totalCount (abbonati a livello di stanza, inclusi visualizzatori anonimi che non elenchiamo).

## Parametri

| Nome | Tipo | Posizione | Obbligatorio | Descrizione |
|------|------|-----------|--------------|-------------|
| tenantId | string | path | Yes |  |
| urlId | string | query | Yes | Identificatore URL della pagina (pulito lato server). |
| afterName | string | query | No | Cursore: passa nextAfterName dalla risposta precedente. |
| afterUserId | string | query | No | Risolutore di pareggio per il cursore: passa nextAfterUserId dalla risposta precedente. Richiesto quando afterName è impostato affinché i pareggi di nome non eliminino voci. |

## Risposta

Restituisce: `PageUsersOnlineResponse`

## Esempio

[inline-code-attrs-start title = 'Esempio getOnlineUsers'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = PublicApi();
final tenantId = tenantId_example; // String |
final urlId = urlId_example; // String | Identificatore URL della pagina (pulito lato server).
final afterName = afterName_example; // String | Cursore: passa nextAfterName dalla risposta precedente.
final afterUserId = afterUserId_example; // String | Risolutore di pareggio per il cursore: passa nextAfterUserId dalla risposta precedente. Richiesto quando afterName è impostato affinché i pareggi di nome non eliminino voci.

try {
    final result = api_instance.getOnlineUsers(tenantId, urlId, GetOnlineUsersOptions(afterName: afterName, afterUserId: afterUserId));
    print(result);
} catch (e) {
    print('Exception when calling PublicApi->getOnlineUsers: $e\n');
}
[inline-code-end]

---
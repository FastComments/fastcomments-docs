Commentatori passati sulla pagina attualmente NON online. Ordinati per displayName.
Usare dopo aver esaurito /users/online per renderizzare una sezione "Members".
Paginazione con cursore su commenterName: il server percorre l'indice parziale {tenantId, urlId, commenterName}
da afterName in avanti tramite $gt, senza costo $skip.

## Parametri

| Nome | Tipo | Posizione | Obbligatorio | Descrizione |
|------|------|----------|----------|-------------|
| tenantId | string | path | Sì |  |
| urlId | string | query | Sì | Identificatore URL della pagina (pulito lato server). |
| afterName | string | query | No | Cursore: passare nextAfterName dalla risposta precedente. |
| afterUserId | string | query | No | Tiebreaker del cursore: passare nextAfterUserId dalla risposta precedente. Obbligatorio quando afterName è impostato, così che le voci con lo stesso nome non vengano perse. |

## Risposta

Restituisce: `PageUsersOfflineResponse`

## Esempio

[inline-code-attrs-start title = 'Esempio di getOfflineUsers'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = PublicApi();
final tenantId = tenantId_example; // String | 
final urlId = urlId_example; // String | Identificatore URL della pagina (pulito lato server).
final afterName = afterName_example; // String | Cursore: passare nextAfterName dalla risposta precedente.
final afterUserId = afterUserId_example; // String | Tiebreaker del cursore: passare nextAfterUserId dalla risposta precedente. Obbligatorio quando afterName è impostato, così che le voci con lo stesso nome non vengano perse.

try {
    final result = api_instance.getOfflineUsers(tenantId, urlId, GetOfflineUsersOptions(afterName: afterName, afterUserId: afterUserId));
    print(result);
} catch (e) {
    print('Exception when calling PublicApi->getOfflineUsers: $e\n');
}
[inline-code-end]
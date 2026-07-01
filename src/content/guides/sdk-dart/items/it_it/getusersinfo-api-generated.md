Informazioni di massa sugli utenti per un tenant. Dati gli userIds, restituisce le informazioni di visualizzazione da User / SSOUser.  
Utilizzato dal widget dei commenti per arricchire gli utenti che sono appena apparsi tramite un evento di presenza.  
Nessun contesto di pagina: la privacy è applicata in modo uniforme (i profili privati sono mascherati).

## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| ids | string | query | Yes | UserIds delimitati da virgola. |

## Response

Returns: `PageUsersInfoResponse`

## Example

[inline-code-attrs-start title = 'Esempio getUsersInfo'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = PublicApi();
final tenantId = tenantId_example; // String | 
final ids = ids_example; // String | Comma-delimited userIds.

try {
    final result = api_instance.getUsersInfo(tenantId, ids);
    print(result);
} catch (e) {
    print('Exception when calling PublicApi->getUsersInfo: $e\n');
}
[inline-code-end]
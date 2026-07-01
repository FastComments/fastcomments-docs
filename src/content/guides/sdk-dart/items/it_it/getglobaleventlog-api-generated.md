req  
tenantId  
urlId  
userIdWS  

## Parametri

| Nome | Tipo | Posizione | Obbligatorio | Descrizione |
|------|------|-----------|--------------|-------------|
| tenantId | string | path | Sì |  |
| urlId | string | query | Sì |  |
| userIdWS | string | query | Sì |  |
| startTime | integer | query | Sì |  |
| endTime | integer | query | No |  |

## Risposta

Restituisce: `GetEventLogResponse`

## Esempio

[inline-code-attrs-start title = 'getGlobalEventLog Esempio'; type = ''; isFunctional = false; inline-code-attrs-end]  
[inline-code-start]  
import 'package:fastcomments_dart/api.dart';

final api_instance = PublicApi();
final tenantId = tenantId_example; // String | 
final urlId = urlId_example; // String | 
final userIdWS = userIdWS_example; // String | 
final startTime = 789; // int | 
final endTime = 789; // int | 

try {
    final result = api_instance.getGlobalEventLog(tenantId, urlId, userIdWS, startTime, endTime);
    print(result);
} catch (e) {
    print('Exception when calling PublicApi->getGlobalEventLog: $e\n');
}
[inline-code-end]
---
req
tenantId
urlId
userIdWS

## Parameters

| Naam | Type | Locatie | Vereist | Beschrijving |
|------|------|----------|----------|---------------|
| tenantId | string | path | Ja |  |
| urlId | string | query | Ja |  |
| userIdWS | string | query | Ja |  |
| startTime | integer | query | Ja |  |
| endTime | integer | query | Nee |  |

## Respons

Retourneert: `GetEventLogResponse`

## Voorbeeld

[inline-code-attrs-start title = 'getEventLog Voorbeeld'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = PublicApi();
final tenantId = tenantId_example; // String | 
final urlId = urlId_example; // String | 
final userIdWS = userIdWS_example; // String | 
final startTime = 789; // int | 
final endTime = 789; // int | 

try {
    final result = api_instance.getEventLog(tenantId, urlId, userIdWS, startTime, endTime);
    print(result);
} catch (e) {
    print('Exception when calling PublicApi->getEventLog: $e\n');
}
[inline-code-end]

---
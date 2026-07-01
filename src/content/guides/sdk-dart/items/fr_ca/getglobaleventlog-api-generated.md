req
tenantId
urlId
userIdWS

## Paramètres

| Nom | Type | Emplacement | Obligatoire | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Oui |  |
| urlId | string | query | Oui |  |
| userIdWS | string | query | Oui |  |
| startTime | integer | query | Oui |  |
| endTime | integer | query | Non |  |

## Réponse

Retourne : `GetEventLogResponse`

## Exemple

[inline-code-attrs-start title = 'Exemple getGlobalEventLog'; type = ''; isFunctional = false; inline-code-attrs-end]
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

---
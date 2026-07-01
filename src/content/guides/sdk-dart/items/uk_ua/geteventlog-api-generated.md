запит
tenantId
urlId
userIdWS

## Параметри

| Назва      | Тип     | Розташування | Обов'язково | Опис |
|------------|---------|--------------|-------------|------|
| tenantId   | string  | path         | Так         |      |
| urlId      | string  | query        | Так         |      |
| userIdWS   | string  | query        | Так         |      |
| startTime  | integer | query        | Так         |      |
| endTime    | integer | query        | Ні          |      |

## Відповідь

Повертає: `GetEventLogResponse`

## Приклад

[inline-code-attrs-start title = 'Приклад getEventLog'; type = ''; isFunctional = false; inline-code-attrs-end]
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
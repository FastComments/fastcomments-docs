## Параметри

| Ім'я | Тип | Розташування | Обов'язково | Опис |
|------|------|--------------|-------------|------|
| tenantId | string | path | Так |  |
| urlId | string | query | Так |  |
| id | string | query | Так |  |

## Відповідь

Повертає: `CreateV1PageReact`

## Приклад

[inline-code-attrs-start title = 'deleteV2PageReact Приклад'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = PublicApi();
final tenantId = tenantId_example; // String | 
final urlId = urlId_example; // String | 
final id = id_example; // String | 

try {
    final result = api_instance.deleteV2PageReact(tenantId, urlId, id);
    print(result);
} catch (e) {
    print('Exception when calling PublicApi->deleteV2PageReact: $e\n');
}
[inline-code-end]
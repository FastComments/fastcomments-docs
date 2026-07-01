## Параметри

| Назва | Тип | Розташування | Обов'язковий | Опис |
|------|------|--------------|--------------|------|
| tenantId | string | query | Так |  |
| id | string | path | Так |  |
| sure | string | query | Ні |  |

## Відповідь

Повертає: `APIEmptyResponse`

## Приклад

[inline-code-attrs-start title = 'Приклад deleteTenant'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO Налаштувати авторизацію API ключа: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// розкоментуйте нижче, щоб налаштувати префікс (наприклад, Bearer) для API ключа, за потреби
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final id = id_example; // String | 
final sure = sure_example; // String | 

try {
    final result = api_instance.deleteTenant(tenantId, id, sure);
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->deleteTenant: $e\n');
}
[inline-code-end]
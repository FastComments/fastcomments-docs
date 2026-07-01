## Параметри

| Назва | Тип | Розташування | Обов'язковий | Опис |
|------|------|--------------|--------------|------|
| tenantId | string | query | Yes |  |
| id | string | path | Yes |  |
| userId | string | query | No |  |

## Відповідь

Повертає: `APIEmptyResponse`

## Приклад

[inline-code-attrs-start title = 'Приклад updateNotification'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO Налаштувати авторизацію за допомогою API ключа: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// розкоментуйте нижче, щоб встановити префікс (наприклад, Bearer) для API ключа, якщо потрібно
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final id = id_example; // String | 
final updateNotificationBody = UpdateNotificationBody(); // UpdateNotificationBody | 
final userId = userId_example; // String | 

try {
    final result = api_instance.updateNotification(tenantId, id, updateNotificationBody, userId);
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->updateNotification: $e\n');
}
[inline-code-end]
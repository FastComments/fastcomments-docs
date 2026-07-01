---
## Параметри

| Назва | Тип | Розташування | Необхідно | Опис |
|------|------|--------------|-----------|------|
| tenantId | string | query | Так |  |
| userId | string | query | Ні |  |
| urlId | string | query | Ні |  |
| fromCommentId | string | query | Ні |  |
| viewed | boolean | query | Ні |  |
| type | string | query | Ні |  |
| skip | number | query | Ні |  |

## Відповідь

Повертає: `GetNotificationsResponse`

## Приклад

[inline-code-attrs-start title = 'getNotifications Приклад'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO Налаштуйте авторизацію ключа API: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// розкоментуйте нижче, щоб налаштувати префікс (e.g. Bearer) для API key, якщо потрібно
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final userId = userId_example; // String | 
final urlId = urlId_example; // String | 
final fromCommentId = fromCommentId_example; // String | 
final viewed = true; // bool | 
final type = type_example; // String | 
final skip = 1.2; // double | 

try {
    final result = api_instance.getNotifications(tenantId, GetNotificationsOptions(userId: userId, urlId: urlId, fromCommentId: fromCommentId, viewed: viewed, type: type, skip: skip));
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->getNotifications: $e\n');
}
[inline-code-end]

---
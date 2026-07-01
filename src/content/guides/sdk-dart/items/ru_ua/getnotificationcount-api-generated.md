## Параметри

| Назва | Тип | Розташування | Обов’язково | Опис |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| userId | string | query | No |  |
| urlId | string | query | No |  |
| fromCommentId | string | query | No |  |
| viewed | boolean | query | No |  |
| type | string | query | No |  |

## Відповідь

Повертає: `GetNotificationCountResponse`

## Приклад

[inline-code-attrs-start title = 'Пример getNotificationCount'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO Налаштувати авторизацію API-ключа: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// розкоментуйте нижче, щоб встановити префікс (наприклад, Bearer) для API-ключа, якщо потрібно
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final userId = userId_example; // String | 
final urlId = urlId_example; // String | 
final fromCommentId = fromCommentId_example; // String | 
final viewed = true; // bool | 
final type = type_example; // String | 

try {
    final result = api_instance.getNotificationCount(tenantId, GetNotificationCountOptions(userId: userId, urlId: urlId, fromCommentId: fromCommentId, viewed: viewed, type: type));
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->getNotificationCount: $e\n');
}
[inline-code-end]
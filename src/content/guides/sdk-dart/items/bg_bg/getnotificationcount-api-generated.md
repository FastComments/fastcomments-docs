## Параметри

| Име | Тип | Местоположение | Задължително | Описание |
|------|------|----------------|--------------|----------|
| tenantId | string | query | Yes |  |
| userId | string | query | No |  |
| urlId | string | query | No |  |
| fromCommentId | string | query | No |  |
| viewed | boolean | query | No |  |
| type | string | query | No |  |

## Отговор

Връща: `GetNotificationCountResponse`

## Пример

[inline-code-attrs-start title = 'Пример за getNotificationCount'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO Конфигуриране на упълномощаване за API ключ: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// разкоментирайте по-долу, за да зададете префикс (например Bearer) за API ключ, ако е необходимо
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
## Параметри

| Име | Тип | Местоположение | Задължително | Описание |
|------|------|----------------|--------------|----------|
| tenantId | string | query | Да |  |
| urlId | string | query | Да |  |
| userId | string | query | Не |  |
| anonUserId | string | query | Не |  |

## Отговор

Връща: `GetVotesForUserResponse`

## Пример

[inline-code-attrs-start title = 'Пример за getVotesForUser'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO Конфигурирайте упълномощаване с API ключ: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// Откоментирайте по-долу, за да зададете префикс (например Bearer) за API ключ, ако е необходимо
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final urlId = urlId_example; // String | 
final userId = userId_example; // String | 
final anonUserId = anonUserId_example; // String | 

try {
    final result = api_instance.getVotesForUser(tenantId, urlId, GetVotesForUserOptions(userId: userId, anonUserId: anonUserId));
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->getVotesForUser: $e\n');
}
[inline-code-end]
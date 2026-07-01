req
tenantId
afterId

## Параметры

| Имя | Тип | Расположение | Обязательно | Описание |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| afterId | string | query | No |  |
| limit | integer | query | No |  |
| tags | array | query | No |  |

## Ответ

Возвращает: `GetFeedPostsResponse`

## Пример

[inline-code-attrs-start title = 'Пример getFeedPosts'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO Настроить авторизацию с помощью API-ключа: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// раскомментировать ниже, чтобы настроить префикс (например, Bearer) для API-ключа, если необходимо
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final afterId = afterId_example; // String | 
final limit = 56; // int | 
final tags = []; // List<String> | 

try {
    final result = api_instance.getFeedPosts(tenantId, GetFeedPostsOptions(afterId: afterId, limit: limit, tags: tags));
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->getFeedPosts: $e\n');
}
[inline-code-end]
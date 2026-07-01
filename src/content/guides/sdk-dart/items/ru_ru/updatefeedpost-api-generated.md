## Параметры

| Имя | Тип | Расположение | Обязательно | Описание |
|------|------|----------|----------|-------------|
| tenantId | string | query | Да |  |
| id | string | path | Да |  |

## Ответ

Returns: `APIEmptyResponse`

## Пример

[inline-code-attrs-start title = 'Пример updateFeedPost'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO Настройте авторизацию ключа API: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// раскомментируйте ниже, чтобы установить префикс (например, Bearer) для ключа API, если необходимо
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final id = id_example; // String | 
final feedPost = FeedPost(); // FeedPost | 

try {
    final result = api_instance.updateFeedPost(tenantId, id, feedPost);
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->updateFeedPost: $e\n');
}
[inline-code-end]
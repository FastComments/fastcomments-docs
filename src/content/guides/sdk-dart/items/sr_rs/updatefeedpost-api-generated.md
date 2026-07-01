## Параметри

| Име | Тип | Локација | Обавезно | Опис |
|------|------|----------|----------|-------------|
| tenantId | string | query | Да |  |
| id | string | path | Да |  |

## Одговор

Враћа: `APIEmptyResponse`

## Пример

[inline-code-attrs-start title = 'Primer updateFeedPost'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO Конфигуриши ауторизацију API кључа: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// otkomentarišite ispod da postavite prefiks (npr. Bearer) za API кључ, ako je potrebno

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
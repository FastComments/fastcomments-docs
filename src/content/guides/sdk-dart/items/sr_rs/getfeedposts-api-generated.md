zahtev
tenantId
afterId

## Параметри

| Naziv | Tip | Lokacija | Obavezno | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | query | Da |  |
| afterId | string | query | Ne |  |
| limit | integer | query | Ne |  |
| tags | array | query | Ne |  |

## Одговор

Враћа: `GetFeedPostsResponse`

## Пример

[inline-code-attrs-start title = 'Primer getFeedPosts'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO Поставите ауторизацију API кључа: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// одкоментаришите доле да бисте подесили префикс (нпр. Bearer) за API кључ, ако је потребно
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
    print('Izuzetak pri pozivanju DefaultApi->getFeedPosts: $e\n');
}
[inline-code-end]
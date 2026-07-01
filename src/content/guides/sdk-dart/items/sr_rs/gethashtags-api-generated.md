## Параметри

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Да |  |
| page | number | query | Не |  |

## Одговор

Враћа: `GetHashTagsResponse`

## Пример

[inline-code-attrs-start title = 'Пример getHashTags'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO Конфигуриши ауторизацију API кључа: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// одкоментариши доле да подесиш префикс (нпр. Bearer) за API кључ, ако је потребно
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final page = 1.2; // double | 

try {
    final result = api_instance.getHashTags(tenantId, page);
    print(result);
} catch (e) {
    print('Изузетак при позивању DefaultApi->getHashTags: $e\n');
}
[inline-code-end]
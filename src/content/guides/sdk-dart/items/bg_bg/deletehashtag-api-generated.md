## Параметри

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| tag | string | path | Yes |  |

## Отговор

Връща: `APIEmptyResponse`

## Пример

[inline-code-attrs-start title = 'deleteHashTag Пример'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO Конфигурирайте упълномощаване на API ключ: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// разкоментирайте по-долу, за да настроите префикс (e.g. Bearer) за API ключ, ако е необходимо
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final tag = tag_example; // String | 
final deleteHashTagRequestBody = DeleteHashTagRequestBody(); // DeleteHashTagRequestBody | 

try {
    final result = api_instance.deleteHashTag(tenantId, tag, deleteHashTagRequestBody);
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->deleteHashTag: $e\n');
}
[inline-code-end]

---
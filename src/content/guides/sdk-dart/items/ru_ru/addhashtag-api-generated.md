## Параметры

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |

## Ответ

Возвращает: `CreateHashTagResponse`

## Пример

[inline-code-attrs-start title = 'Пример addHashTag'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO Настройте авторизацию API-ключа: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// раскомментируйте ниже, чтобы задать префикс (например, Bearer) для API-ключа, если необходимо
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final createHashTagBody = CreateHashTagBody(); // CreateHashTagBody | 

try {
    final result = api_instance.addHashTag(tenantId, createHashTagBody);
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->addHashTag: $e\n');
}
[inline-code-end]
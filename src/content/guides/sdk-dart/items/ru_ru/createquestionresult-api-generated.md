## Parameters

| Name | Type | Location | Required | Описание |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |

## Response

Возвращает: `CreateQuestionResultResponse`

## Example

[inline-code-attrs-start title = 'createQuestionResult Пример'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO Настройте авторизацию ключа API: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// раскомментировать ниже, чтобы установить префикс (например, Bearer) для ключа API, при необходимости
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final createQuestionResultBody = CreateQuestionResultBody(); // CreateQuestionResultBody | 

try {
    final result = api_instance.createQuestionResult(tenantId, createQuestionResultBody);
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->createQuestionResult: $e\n');
}
[inline-code-end]
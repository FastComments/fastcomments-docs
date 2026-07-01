## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| id | string | path | Yes |  |

## Response

Returns: `APIEmptyResponse`

## Example

[inline-code-attrs-start title = 'Пример updateQuestionResult'; type = ''; isFunctional = false; inline-code-attrs-end]
```dart
import 'package:fastcomments_dart/api.dart';
// TODO Настройте авторизацию API ключа: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// раскоментируйте ниже, чтобы установить префикс (например, Bearer) для API ключа, если необходимо
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final id = id_example; // String | 
final updateQuestionResultBody = UpdateQuestionResultBody(); // UpdateQuestionResultBody | 

try {
    final result = api_instance.updateQuestionResult(tenantId, id, updateQuestionResultBody);
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->updateQuestionResult: $e\n');
}
```
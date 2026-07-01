## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| id | string | path | Yes |  |

## Response

Returns: `APIEmptyResponse`

## Example

[inline-code-attrs-start title = 'Приклад updateEmailTemplate'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO Налаштуйте авторизацію за допомогою API ключа: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// розкоментуйте нижче, щоб налаштувати префікс (наприклад, Bearer) для API ключа, якщо потрібно
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final id = id_example; // String | 
final updateEmailTemplateBody = UpdateEmailTemplateBody(); // UpdateEmailTemplateBody | 

try {
    final result = api_instance.updateEmailTemplate(tenantId, id, updateEmailTemplateBody);
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->updateEmailTemplate: $e\n');
}
[inline-code-end]
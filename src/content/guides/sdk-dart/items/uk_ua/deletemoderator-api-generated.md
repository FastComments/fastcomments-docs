## Parameters

| Ім'я | Тип | Розташування | Обов'язково | Опис |
|------|------|--------------|-------------|------|
| tenantId | string | query | Yes |  |
| id | string | path | Yes |  |
| sendEmail | string | query | No |  |

## Response

Повертає: `APIEmptyResponse`

## Example

[inline-code-attrs-start title = 'deleteModerator Приклад'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO: налаштуйте авторизацію API ключа: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// Розкоментуйте нижче, щоб налаштувати префікс (наприклад, Bearer) для API ключа, за потреби
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final id = id_example; // String | 
final sendEmail = sendEmail_example; // String | 

try {
    final result = api_instance.deleteModerator(tenantId, id, sendEmail);
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->deleteModerator: $e\n');
}
[inline-code-end]
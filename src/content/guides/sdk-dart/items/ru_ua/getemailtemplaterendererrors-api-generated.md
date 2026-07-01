## Параметри

| Назва | Тип | Розташування | Обов’язковий | Опис |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| id | string | path | Yes |  |
| skip | number | query | No |  |

## Відповідь

Повертає: `GetEmailTemplateRenderErrorsResponse`

## Приклад

[inline-code-attrs-start title = 'getEmailTemplateRenderErrors Приклад'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO Налаштувати авторизацію за допомогою API ключа: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// розкоментуйте нижче, щоб налаштувати префікс (наприк., Bearer) для API ключа, за потреби
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final id = id_example; // String | 
final skip = 1.2; // double | 

try {
    final result = api_instance.getEmailTemplateRenderErrors(tenantId, id, skip);
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->getEmailTemplateRenderErrors: $e\n');
}
[inline-code-end]
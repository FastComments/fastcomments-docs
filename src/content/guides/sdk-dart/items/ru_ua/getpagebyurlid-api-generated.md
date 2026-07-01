## Параметри

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Так |  |
| urlId | string | query | Так |  |

## Відповідь

Повертає: `GetPageByURLIdAPIResponse`

## Приклад

[inline-code-attrs-start title = 'Приклад getPageByURLId'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO Налаштувати авторизацію ключа API: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// розкоментуйте нижче, щоб встановити префікс (наприклад, Bearer) для ключа API, якщо потрібно
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final urlId = urlId_example; // String | 

try {
    final result = api_instance.getPageByURLId(tenantId, urlId);
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->getPageByURLId: $e\n');
}
[inline-code-end]

---
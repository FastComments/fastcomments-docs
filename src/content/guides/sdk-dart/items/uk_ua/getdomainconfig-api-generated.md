## Параметри

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| domain | string | path | Yes |  |

## Відповідь

Повертає: `GetDomainConfigResponse`

## Приклад

[inline-code-attrs-start title = 'Приклад getDomainConfig'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO Налаштуйте авторизацію API-ключа: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// розкоментуйте нижче, щоб налаштувати префікс (наприклад, Bearer) для API-ключа, якщо потрібно
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final domain = domain_example; // String | 

try {
    final result = api_instance.getDomainConfig(tenantId, domain);
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->getDomainConfig: $e\n');
}
[inline-code-end]

---
## Параметры

| Имя | Тип | Местоположение | Обязательно | Описание |
|------|------|----------------|-------------|----------|
| tenantId | string | query | Yes |  |
| domain | string | path | Yes |  |

## Ответ

Возвращает: `GetDomainConfigResponse`

## Пример

[inline-code-attrs-start title = 'Пример getDomainConfig'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO Настроить авторизацию ключа API: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// раскомментировать ниже, чтобы установить префикс (например, Bearer) для ключа API, если необходимо
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
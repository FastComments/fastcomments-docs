## Параметры

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| domainToUpdate | string | path | Yes |  |

## Ответ

Returns: `PutDomainConfigResponse`

## Пример

[inline-code-attrs-start title = 'Пример putDomainConfig'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO Настроить авторизацию API-ключа: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// раскомментировать ниже, чтобы настроить префикс (например, Bearer) для API-ключа, при необходимости
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final domainToUpdate = domainToUpdate_example; // String | 
final updateDomainConfigParams = UpdateDomainConfigParams(); // UpdateDomainConfigParams | 

try {
    final result = api_instance.putDomainConfig(tenantId, domainToUpdate, updateDomainConfigParams);
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->putDomainConfig: $e\n');
}
[inline-code-end]
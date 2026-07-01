## Параметри

| Име | Тип | Местоположение | Задължително | Описание |
|------|------|----------|----------|-------------|
| tenantId | string | query | Да |  |
| domainToUpdate | string | path | Да |  |

## Отговор

Връща: `PutDomainConfigResponse`

## Пример

[inline-code-attrs-start title = 'Пример за putDomainConfig'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO Конфигурирайте оторизация за API ключ: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// разкоментирайте по-долу, за да настроите префикс (например Bearer) за API ключ, ако е необходимо
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
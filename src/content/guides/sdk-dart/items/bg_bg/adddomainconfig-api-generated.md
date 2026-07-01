## Параметри

| Име | Тип | Местоположение | Задължително | Описание |
|------|------|----------------|-------------|----------|
| tenantId | string | query | Да |  |

## Отговор

Връща: `AddDomainConfigResponse`

## Пример

[inline-code-attrs-start title = 'addDomainConfig Пример'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO Конфигурирайте удостоверяване за API ключ: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// премахнете коментара по-долу, за да зададете префикс (напр. Bearer) за API ключа, ако е необходимо
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final addDomainConfigParams = AddDomainConfigParams(); // AddDomainConfigParams | 

try {
    final result = api_instance.addDomainConfig(tenantId, addDomainConfigParams);
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->addDomainConfig: $e\n');
}
[inline-code-end]
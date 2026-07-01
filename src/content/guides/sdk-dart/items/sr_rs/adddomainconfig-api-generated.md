## Parameters

| Име | Тип | Локација | Обавезно | Опис |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |

## Response

Враћа: `AddDomainConfigResponse`

## Пример

[inline-code-attrs-start title = 'addDomainConfig Primer'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO Подесите ауторизацију API кључа: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// откоментаришите испод да поставите префикс (нпр. Bearer) за API кључ, ако је потребно
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
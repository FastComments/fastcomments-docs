---
## Параметри

| Име | Тип | Локација | Обавезно | Опис |
|------|------|----------|----------|-------------|
| tenantId | string | query | Да |  |
| email | string | path | Да |  |

## Одговор

Враћа: `GetSSOUserByEmailAPIResponse`

## Пример

[inline-code-attrs-start title = 'Primer getSSOUserByEmail'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO Конфигуришите ауторизацију API кључа: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// одкоментаришите испод да подесите префикс (нпр. Bearer) за API кључ, ако је потребно
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final email = email_example; // String | 

try {
    final result = api_instance.getSSOUserByEmail(tenantId, email);
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->getSSOUserByEmail: $e\n');
}
[inline-code-end]

---
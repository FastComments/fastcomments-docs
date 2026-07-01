## Параметри

| Име | Тип | Место | Обавезно | Опис |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| urlId | string | query | Yes |  |

## Одговор

Враћа: `GetPageByURLIdAPIResponse`

## Пример

[inline-code-attrs-start title = 'Primer getPageByURLId'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO Конфигуришите ауторизацију API кључа: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// uncomment below to setup prefix (e.g. Bearer) for API key, if needed
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
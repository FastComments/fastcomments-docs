## Параметри

| Име | Тип | Локација | Обавезно | Опис |
|------|------|----------|----------|-------|
| tenantId | string | query | Да |  |

## Одговор

Returns: `GetPagesAPIResponse`

## Пример

[inline-code-attrs-start title = 'getPages Пример'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO Конфигуришите ауторизацију API кључа: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// uklonite komentar ispod da postavite prefiks (npr. Bearer) za API кључ, ako je potrebno
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 

try {
    final result = api_instance.getPages(tenantId);
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->getPages: $e\n');
}
[inline-code-end]
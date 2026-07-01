## Параметри

| Назив | Тип | Локација | Обавезно | Опис |
|------|------|----------|----------|------|
| tenantId | string | query | Yes |  |
| id | string | path | Yes |  |
| redirectURL | string | query | No |  |

## Одговор

Враћа: `APIEmptyResponse`

## Пример

[inline-code-attrs-start title = 'sendLoginLink Пример'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO Конфигуришите ауторизацију API кључа: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// одкоментаришите доле да поставите префикс (нпр. Bearer) за API кључ, ако је потребно
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final id = id_example; // String | 
final redirectURL = redirectURL_example; // String | 

try {
    final result = api_instance.sendLoginLink(tenantId, id, redirectURL);
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->sendLoginLink: $e\n');
}
[inline-code-end]
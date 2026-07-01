## Параметры

| Имя | Тип | Местоположение | Обязательно | Описание |
|------|------|----------------|--------------|----------|
| tenantId | string | query | Yes |  |

## Ответ

Возвращает: `CreateModeratorResponse`

## Пример

[inline-code-attrs-start title = 'Пример createModerator'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO: настройте авторизацию API‑ключа: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// раскомментировать ниже, чтобы установить префикс (например, Bearer) для API‑ключа, если необходимо
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final createModeratorBody = CreateModeratorBody(); // CreateModeratorBody | 

try {
    final result = api_instance.createModerator(tenantId, createModeratorBody);
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->createModerator: $e\n');
}
[inline-code-end]
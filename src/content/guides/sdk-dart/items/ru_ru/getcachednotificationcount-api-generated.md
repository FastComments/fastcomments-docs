## Параметры

| Имя | Тип | Расположение | Обязательно | Описание |
|------|------|--------------|--------------|----------|
| tenantId | string | query | Да |  |
| id | string | path | Да |  |

## Ответ

Возвращает: `GetCachedNotificationCountResponse`

## Пример

[inline-code-attrs-start title = 'Пример getCachedNotificationCount'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO Настроить авторизацию с API ключом: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// раскомментировать ниже, чтобы установить префикс (например, Bearer) для API ключа, если необходимо
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final id = id_example; // String | 

try {
    final result = api_instance.getCachedNotificationCount(tenantId, id);
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->getCachedNotificationCount: $e\n');
}
[inline-code-end]
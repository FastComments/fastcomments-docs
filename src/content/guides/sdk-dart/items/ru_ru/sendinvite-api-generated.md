## Параметры

| Имя | Тип | Местоположение | Обязательно | Описание |
|------|------|----------------|------------|-----------|
| tenantId | string | query | Да |  |
| id | string | path | Да |  |
| fromName | string | query | Да |  |

## Ответ

Возвращает: `APIEmptyResponse`

## Пример

[inline-code-attrs-start title = 'Пример sendInvite'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO Настройте авторизацию API-ключа: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// раскомментируйте ниже, чтобы установить префикс (например, Bearer) для API-ключа, если необходимо
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final id = id_example; // String | 
final fromName = fromName_example; // String | 

try {
    final result = api_instance.sendInvite(tenantId, id, fromName);
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->sendInvite: $e\n');
}
[inline-code-end]
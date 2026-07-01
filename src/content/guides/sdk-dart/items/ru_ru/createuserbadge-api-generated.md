## Параметры

| Имя | Тип | Расположение | Обязательно | Описание |
|------|------|----------|----------|-------------|
| tenantId | string | query | Да |  |

## Ответ

Возвращает: `APICreateUserBadgeResponse`

## Пример

[inline-code-attrs-start title = 'Пример createUserBadge'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO Настройте авторизацию API-ключа: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// раскомментируйте ниже, чтобы установить префикс (например, Bearer) для API-ключа, если необходимо
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final createUserBadgeParams = CreateUserBadgeParams(); // CreateUserBadgeParams | 

try {
    final result = api_instance.createUserBadge(tenantId, createUserBadgeParams);
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->createUserBadge: $e\n');
}
[inline-code-end]
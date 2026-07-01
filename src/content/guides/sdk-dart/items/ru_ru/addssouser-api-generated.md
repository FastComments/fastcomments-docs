## Параметры

| Имя | Тип | Место | Обязательно | Описание |
|------|------|----------|----------|-------------|
| tenantId | string | query | Да |  |

## Ответ

Возвращает: `AddSSOUserAPIResponse`

## Пример

[inline-code-attrs-start title = 'Пример addSSOUser'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO Настроить авторизацию API-ключа: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// раскомментировать ниже, чтобы установить префикс (например, Bearer) для API-ключа, если необходимо
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final createAPISSOUserData = CreateAPISSOUserData(); // CreateAPISSOUserData | 

try {
    final result = api_instance.addSSOUser(tenantId, createAPISSOUserData);
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->addSSOUser: $e\n');
}
[inline-code-end]
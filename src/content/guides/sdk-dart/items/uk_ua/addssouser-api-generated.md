## Параметри

| Назва | Тип | Розташування | Обов’язково | Опис |
|------|------|----------|----------|-------------|
| tenantId | string | query | Так |  |

## Відповідь

Returns: `AddSSOUserAPIResponse`

## Приклад

[inline-code-attrs-start title = 'addSSOUser Приклад'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO Налаштувати авторизацію ключа API: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// розкоментуйте нижче, щоб налаштувати префікс (наприклад, Bearer) для ключа API, якщо потрібно
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
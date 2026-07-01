## Параметри

| Назва | Тип | Розташування | Обов'язково | Опис |
|------|------|--------------|-------------|------|
| tenantId | string | query | Yes |  |
| id | string | path | Yes |  |
| deleteComments | boolean | query | No |  |
| commentDeleteMode | string | query | No |  |

## Відповідь

Повертає: `DeleteSSOUserAPIResponse`

## Приклад

[inline-code-attrs-start title = 'Приклад deleteSSOUser'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO Налаштуйте авторизацію ключа API: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// розкоментуйте нижче, щоб встановити префікс (наприклад, Bearer) для ключа API, якщо потрібно
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String |
final id = id_example; // String |
final deleteComments = true; // bool |
final commentDeleteMode = commentDeleteMode_example; // String |

try {
    final result = api_instance.deleteSSOUser(tenantId, id, DeleteSSOUserOptions(deleteComments: deleteComments, commentDeleteMode: commentDeleteMode));
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->deleteSSOUser: $e\n');
}
[inline-code-end]
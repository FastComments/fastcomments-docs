## Параметри

| Назва | Тип | Розташування | Обов'язково | Опис |
|------|------|----------|----------|-------------|
| tenantId | string | query | Так |  |
| id | string | path | Так |  |
| deleteComments | string | query | Ні |  |
| commentDeleteMode | string | query | Ні |  |

## Відповідь

Повертає: `APIEmptyResponse`

## Приклад

[inline-code-attrs-start title = 'Приклад deleteTenantUser'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO Налаштувати авторизацію ключа API: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// розкоментуйте нижче, щоб налаштувати префікс (наприклад Bearer) для ключа API, якщо потрібно
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final id = id_example; // String | 
final deleteComments = deleteComments_example; // String | 
final commentDeleteMode = commentDeleteMode_example; // String | 

try {
    final result = api_instance.deleteTenantUser(tenantId, id, DeleteTenantUserOptions(deleteComments: deleteComments, commentDeleteMode: commentDeleteMode));
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->deleteTenantUser: $e\n');
}
[inline-code-end]
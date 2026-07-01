## Параметри

| Ім'я | Тип | Розташування | Обов'язково | Опис |
|------|------|--------------|--------------|------|
| tenantId | string | query | Yes |  |
| id | string | path | Yes |  |

## Відповідь

Повертає: `PatchPageAPIResponse`

## Приклад

[inline-code-attrs-start title = 'patchPage Приклад'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO Налаштувати авторизацію ключа API: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// розкоментуйте нижче, щоб встановити префікс (наприклад, Bearer) для ключа API, якщо потрібно
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final id = id_example; // String | 
final updateAPIPageData = UpdateAPIPageData(); // UpdateAPIPageData | 

try {
    final result = api_instance.patchPage(tenantId, id, updateAPIPageData);
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->patchPage: $e\n');
}
[inline-code-end]
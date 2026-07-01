---
## Параметри

| Назва | Тип | Розташування | Обов'язковий | Опис |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| id | string | path | Yes |  |

## Відповідь

Повертає: `APIEmptyResponse`

## Приклад

[inline-code-attrs-start title = 'Приклад deleteEmailTemplate'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO Налаштуйте авторизацію API-ключа: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// раскоментуйте нижче, щоб налаштувати префікс (наприклад, Bearer) для API-ключа, якщо потрібно
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final id = id_example; // String | 

try {
    final result = api_instance.deleteEmailTemplate(tenantId, id);
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->deleteEmailTemplate: $e\n');
}
[inline-code-end]

---
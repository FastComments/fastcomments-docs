## Параметри

| Назва | Тип | Розташування | Обов’язковий | Опис |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| id | string | path | Yes |  |

## Відповідь

Повертає: `GetCachedNotificationCountResponse`

## Приклад

[inline-code-attrs-start title = 'getCachedNotificationCount Приклад'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO Налаштуйте авторизацію за допомогою ключа API: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// розкоментуйте нижче, щоб налаштувати префікс (наприклад, Bearer) для ключа API, якщо потрібно
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

---
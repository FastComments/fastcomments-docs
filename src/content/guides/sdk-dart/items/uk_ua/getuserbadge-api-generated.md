## Параметри

| Назва | Тип | Розташування | Обов’язково | Опис |
|------|------|--------------|--------------|------|
| tenantId | string | query | Yes |  |
| id | string | path | Yes |  |

## Відповідь

Повертає: `APIGetUserBadgeResponse`

## Приклад

[inline-code-attrs-start title = 'Приклад getUserBadge'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO Налаштуйте авторизацію за допомогою API ключа: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// розкоментуйте нижче, щоб встановити префікс (наприклад, Bearer) для API ключа, якщо потрібно
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final id = id_example; // String | 

try {
    final result = api_instance.getUserBadge(tenantId, id);
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->getUserBadge: $e\n');
}
[inline-code-end]
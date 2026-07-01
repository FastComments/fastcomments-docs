## Параметри

| Ім'я | Тип | Розташування | Обов'язково | Опис |
|------|------|--------------|-------------|------|
| tenantId | string | query | Так |  |
| id | string | path | Так |  |

## Відповідь

Повертає: `APIGetUserBadgeProgressResponse`

## Приклад

[inline-code-attrs-start title = 'Приклад getUserBadgeProgressById'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO Налаштувати авторизацію за допомогою API ключа: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// розкоментуйте нижче, щоб налаштувати префікс (наприклад, Bearer) для API ключа, якщо потрібно

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final id = id_example; // String | 

try {
    final result = api_instance.getUserBadgeProgressById(tenantId, id);
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->getUserBadgeProgressById: $e\n');
}
[inline-code-end]
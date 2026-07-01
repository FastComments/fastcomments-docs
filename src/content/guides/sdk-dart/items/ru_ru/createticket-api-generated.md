## Параметры

| Имя | Тип | Расположение | Обязательно | Описание |
|------|------|----------|----------|-------------|
| tenantId | string | query | Да |  |
| userId | string | query | Да |  |

## Ответ

Возвращает: `CreateTicketResponse`

## Пример

[inline-code-attrs-start title = 'createTicket Пример'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO Настройте авторизацию API-ключа: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// раскомментируйте ниже, чтобы установить префикс (например, Bearer) для API‑ключа, если необходимо
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final userId = userId_example; // String | 
final createTicketBody = CreateTicketBody(); // CreateTicketBody | 

try {
    final result = api_instance.createTicket(tenantId, userId, createTicketBody);
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->createTicket: $e\n');
}
[inline-code-end]
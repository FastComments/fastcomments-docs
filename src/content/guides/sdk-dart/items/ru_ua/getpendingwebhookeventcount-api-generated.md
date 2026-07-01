## Параметри

| Назва | Тип | Розташування | Обов'язково | Опис |
|------|------|----------|----------|-------------|
| tenantId | string | query | Так |  |
| commentId | string | query | Ні |  |
| externalId | string | query | Ні |  |
| eventType | string | query | Ні |  |
| type | string | query | Ні |  |
| domain | string | query | Ні |  |
| attemptCountGT | number | query | Ні |  |

## Відповідь

Повертає: `GetPendingWebhookEventCountResponse`

## Приклад

[inline-code-attrs-start title = 'getPendingWebhookEventCount Приклад'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO Настроить авторизацию ключа API: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// розкоментуйте нижче, щоб налаштувати префікс (наприклад, Bearer) для ключа API, якщо потрібно
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final commentId = commentId_example; // String | 
final externalId = externalId_example; // String | 
final eventType = eventType_example; // String | 
final type = type_example; // String | 
final domain = domain_example; // String | 
final attemptCountGT = 1.2; // double | 

try {
    final result = api_instance.getPendingWebhookEventCount(tenantId, GetPendingWebhookEventCountOptions(commentId: commentId, externalId: externalId, eventType: eventType, type: type, domain: domain, attemptCountGT: attemptCountGT));
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->getPendingWebhookEventCount: $e\n');
}
[inline-code-end]
## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| userId | string | query | No |  |
| state | number | query | No |  |
| skip | number | query | No |  |
| limit | number | query | No |  |

## Response

Връща: `GetTicketsResponse`

## Example

[inline-code-attrs-start title = 'Пример за getTickets'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO Конфигурирайте упълномощаване на API ключ: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// разкоментирайте по-долу, за да настроите префикс (например Bearer) за API ключ, ако е необходимо
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final userId = userId_example; // String | 
final state = 1.2; // double | 
final skip = 1.2; // double | 
final limit = 1.2; // double | 

try {
    final result = api_instance.getTickets(tenantId, GetTicketsOptions(userId: userId, state: state, skip: skip, limit: limit));
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->getTickets: $e\n');
}
[inline-code-end]
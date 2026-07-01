## Параметри

| Име | Тип | Местоположение | Задължително | Описание |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| userId | string | query | Yes |  |
| id | string | path | Yes |  |

## Отговор

Връща: `ChangeTicketStateResponse`

## Пример

[inline-code-attrs-start title = 'Пример за changeTicketState'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO Конфигуриране на API ключ за упълномощаване: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// разкоментирайте по-долу, за да настроите префикс (например Bearer) за API ключа, ако е необходимо
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final userId = userId_example; // String | 
final id = id_example; // String | 
final changeTicketStateBody = ChangeTicketStateBody(); // ChangeTicketStateBody | 

try {
    final result = api_instance.changeTicketState(tenantId, userId, id, changeTicketStateBody);
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->changeTicketState: $e\n');
}
[inline-code-end]

---
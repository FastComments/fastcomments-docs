## Параметри

| Име | Тип | Локација | Обавезно | Опис |
|------|------|----------|----------|-------------|
| tenantId | string | query | Да |  |
| userId | string | query | Не |  |
| urlId | string | query | Не |  |
| fromCommentId | string | query | Не |  |
| viewed | boolean | query | Не |  |
| type | string | query | Не |  |
| skip | number | query | Не |  |

## Одговор

Враћа: `GetNotificationsResponse`

## Пример

[inline-code-attrs-start title = 'Primer getNotifications'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO Konfigurišite autorizaciju API ključa: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// odkomentarišite ispod da postavite prefiks (npr. Bearer) za API ključ, ako je potrebno
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final userId = userId_example; // String | 
final urlId = urlId_example; // String | 
final fromCommentId = fromCommentId_example; // String | 
final viewed = true; // bool | 
final type = type_example; // String | 
final skip = 1.2; // double | 

try {
    final result = api_instance.getNotifications(tenantId, GetNotificationsOptions(userId: userId, urlId: urlId, fromCommentId: fromCommentId, viewed: viewed, type: type, skip: skip));
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->getNotifications: $e\n');
}
[inline-code-end]
## Parametri

| Naziv | Tip | Lokacija | Obavezno | Opis |
|------|------|----------|----------|------|
| tenantId | string | query | Yes |  |
| userId | string | query | No |  |
| urlId | string | query | No |  |
| fromCommentId | string | query | No |  |
| viewed | boolean | query | No |  |
| type | string | query | No |  |

## Odgovor

Vraća: `GetNotificationCountResponse`

## Primer

[inline-code-attrs-start title = 'getNotificationCount Primer'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO Konfigurišite autorizaciju API ključa: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// otkomentarišite ispod da postavite prefiks (npr. Bearer) za API ključ, po potrebi
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final userId = userId_example; // String | 
final urlId = urlId_example; // String | 
final fromCommentId = fromCommentId_example; // String | 
final viewed = true; // bool | 
final type = type_example; // String | 

try {
    final result = api_instance.getNotificationCount(tenantId, GetNotificationCountOptions(userId: userId, urlId: urlId, fromCommentId: fromCommentId, viewed: viewed, type: type));
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->getNotificationCount: $e\n');
}
[inline-code-end]
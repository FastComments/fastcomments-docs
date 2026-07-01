## Parametri

| Naziv | Tip | Lokacija | Obavezno | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| isLive | boolean | query | No |  |
| doSpamCheck | boolean | query | No |  |
| sendEmails | boolean | query | No |  |
| populateNotifications | boolean | query | No |  |

## Odgovor

Returns: `APISaveCommentResponse`

## Primer

[inline-code-attrs-start title = 'saveComment Primer'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO Konfigurišite autorizaciju API ključa: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// odkomentarišite ispod da postavite prefiks (npr. Bearer) za API ključ, ako je potrebno
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final createCommentParams = CreateCommentParams(); // CreateCommentParams | 
final isLive = true; // bool | 
final doSpamCheck = true; // bool | 
final sendEmails = true; // bool | 
final populateNotifications = true; // bool | 

try {
    final result = api_instance.saveComment(tenantId, createCommentParams, SaveCommentOptions(isLive: isLive, doSpamCheck: doSpamCheck, sendEmails: sendEmails, populateNotifications: populateNotifications));
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->saveComment: $e\n');
}
[inline-code-end]
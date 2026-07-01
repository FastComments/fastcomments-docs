## Parametri

| Naziv | Tip | Lokacija | Obavezno | Opis |
|------|------|----------|----------|------|
| tenantId | string | query | Da |  |
| isLive | boolean | query | Ne |  |
| doSpamCheck | boolean | query | Ne |  |
| sendEmails | boolean | query | Ne |  |
| populateNotifications | boolean | query | Ne |  |

## Odgovor

Vraća: `SaveCommentsBulkResponse`

## Primer

[inline-code-attrs-start title = 'saveCommentsBulk Primer'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO Konfigurišite autorizaciju API ključa: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// otkomentarišite dolje da postavite prefiks (npr. Bearer) za API ključ, ako je potrebno
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final createCommentParams = [List<CreateCommentParams>()]; // List<CreateCommentParams> | 
final isLive = true; // bool | 
final doSpamCheck = true; // bool | 
final sendEmails = true; // bool | 
final populateNotifications = true; // bool | 

try {
    final result = api_instance.saveCommentsBulk(tenantId, createCommentParams, SaveCommentsBulkOptions(isLive: isLive, doSpamCheck: doSpamCheck, sendEmails: sendEmails, populateNotifications: populateNotifications));
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->saveCommentsBulk: $e\n');
}
[inline-code-end]
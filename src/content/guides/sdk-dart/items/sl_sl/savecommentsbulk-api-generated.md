## Parametri

| Ime | Vrsta | Lokacija | Obvezno | Opis |
|------|------|----------|----------|------|
| tenantId | string | query | Da |  |
| isLive | boolean | query | Ne |  |
| doSpamCheck | boolean | query | Ne |  |
| sendEmails | boolean | query | Ne |  |
| populateNotifications | boolean | query | Ne |  |

## Odgovor

Vrne: `SaveCommentsBulkResponse`

## Primer

[inline-code-attrs-start title = 'saveCommentsBulk Primer'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO Nastavite overitev API ključa: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// odkomentirajte spodaj, da nastavite predpono (npr. Bearer) za API ključ, po potrebi
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
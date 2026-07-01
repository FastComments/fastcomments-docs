## Parametry

| Nazwa | Typ | Lokalizacja | Wymagane | Opis |
|------|------|----------|----------|------|
| tenantId | string | query | Yes |  |
| isLive | boolean | query | No |  |
| doSpamCheck | boolean | query | No |  |
| sendEmails | boolean | query | No |  |
| populateNotifications | boolean | query | No |  |

## Odpowiedź

Zwraca: `SaveCommentsBulkResponse`

## Przykład

[inline-code-attrs-start title = 'saveCommentsBulk Przykład'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO Skonfiguruj autoryzację kluczem API: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// odkomentuj poniżej, aby ustawić prefiks (np. Bearer) dla klucza API, jeśli potrzebny
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
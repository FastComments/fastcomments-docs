## Parametri

| Naziv | Tip | Lokacija | Obavezno | Opis |
|------|------|----------|----------|------|
| tenantId | string | query | Yes |  |
| urlId | string | query | Yes |  |
| userId | string | query | No |  |
| anonUserId | string | query | No |  |

## Odgovor

Vraća: `GetVotesForUserResponse`

## Primjer

[inline-code-attrs-start title = 'Primjer getVotesForUser'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO Konfigurirajte autorizaciju API ključa: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// otkomentirajte dolje da biste postavili prefiks (npr. Bearer) za API ključ, ako je potrebno
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final urlId = urlId_example; // String | 
final userId = userId_example; // String | 
final anonUserId = anonUserId_example; // String | 

try {
    final result = api_instance.getVotesForUser(tenantId, urlId, GetVotesForUserOptions(userId: userId, anonUserId: anonUserId));
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->getVotesForUser: $e\n');
}
[inline-code-end]
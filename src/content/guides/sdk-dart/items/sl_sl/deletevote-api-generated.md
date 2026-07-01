## Parameters

| Ime | Vrsta | Lokacija | Obvezno | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | poizvedba | Da |  |
| id | string | pot | Da |  |
| editKey | string | poizvedba | Ne |  |

## Odgovor

Vrne: `VoteDeleteResponse`

## Primer

[inline-code-attrs-start title = 'deleteVote Primer'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO Konfigurirajte avtorizacijo API ključa: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// odkomentirajte spodaj za nastavitev predpone (npr. Bearer) za API ključ, če je potrebno
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final id = id_example; // String | 
final editKey = editKey_example; // String | 

try {
    final result = api_instance.deleteVote(tenantId, id, editKey);
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->deleteVote: $e\n');
}
[inline-code-end]

---
## Parametri

| Naziv | Tip | Lokacija | Obavezno | Opis |
|------|------|----------|----------|------|
| tenantId | string | query | Yes |  |
| id | string | path | Yes |  |

## Odgovor

Vraća: `APIEmptyResponse`

## Primjer

[inline-code-attrs-start title = 'updateModerator Primjer'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO Konfiguriraj autorizaciju API ključa: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// otkomentiraj dolje za postavljanje prefiksa (npr. Bearer) za API ključ, ako je potrebno
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final id = id_example; // String | 
final updateModeratorBody = UpdateModeratorBody(); // UpdateModeratorBody | 

try {
    final result = api_instance.updateModerator(tenantId, id, updateModeratorBody);
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->updateModerator: $e\n');
}
[inline-code-end]
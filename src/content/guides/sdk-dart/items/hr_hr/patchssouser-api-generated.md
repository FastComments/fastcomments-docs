## Parametri

| Naziv | Tip | Lokacija | Obavezno | Opis |
|------|------|----------|----------|------|
| tenantId | string | query | Da |  |
| id | string | path | Da |  |
| updateComments | boolean | query | Ne |  |

## Odgovor

Vraća: `PatchSSOUserAPIResponse`

## Primjer

[inline-code-attrs-start title = 'Primjer patchSSOUser'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO Konfigurirajte autorizaciju API ključa: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// otkomentirajte dolje za postavljanje prefiksa (npr. Bearer) za API ključ, po potrebi
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final id = id_example; // String | 
final updateAPISSOUserData = UpdateAPISSOUserData(); // UpdateAPISSOUserData | 
final updateComments = true; // bool | 

try {
    final result = api_instance.patchSSOUser(tenantId, id, updateAPISSOUserData, updateComments);
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->patchSSOUser: $e\n');
}
[inline-code-end]
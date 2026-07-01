## Parametri

| Ime | Vrsta | Lokacija | Potrebno | Opis |
|------|------|----------|----------|------|
| tenantId | string | query | Yes |  |

## Odgovor

Vrne: `CreateModeratorResponse`

## Primer

[inline-code-attrs-start title = 'createModerator Primer'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO Nastavi avtorizacijo API ključa: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// odkomentiraj spodaj za nastavitev predpone (npr. Bearer) za API ključ, če je potrebno
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final createModeratorBody = CreateModeratorBody(); // CreateModeratorBody | 

try {
    final result = api_instance.createModerator(tenantId, createModeratorBody);
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->createModerator: $e\n');
}
[inline-code-end]
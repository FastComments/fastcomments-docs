## Parametri

| Ime | Tip | Lokacija | Zahtevano | Opis |
|------|------|----------|-----------|------|
| tenantId | string | query | Yes |  |
| id | string | path | Yes |  |
| errorId | string | path | Yes |  |

## Odgovor

Vrne: `APIEmptyResponse`

## Primer

[inline-code-attrs-start title = 'Primer deleteEmailTemplateRenderError'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO Konfigurirajte avtentikacijo API ključa: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// odkomentirajte spodaj za nastavitev predpone (npr. Bearer) za API ključ, po potrebi
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final id = id_example; // String | 
final errorId = errorId_example; // String | 

try {
    final result = api_instance.deleteEmailTemplateRenderError(tenantId, id, errorId);
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->deleteEmailTemplateRenderError: $e\n');
}
[inline-code-end]
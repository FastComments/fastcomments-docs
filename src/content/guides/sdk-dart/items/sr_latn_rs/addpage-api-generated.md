## Parametri

| Naziv | Tip | Lokacija | Obavezno | Opis |
|------|------|----------|----------|------|
| tenantId | string | query | Yes |  |

## Odgovor

Vraća: `AddPageAPIResponse`

## Primer

[inline-code-attrs-start title = 'addPage Primer'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO Konfigurišite autorizaciju API ključa: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// uklonite komentar ispod da postavite prefiks (npr. Bearer) za API ključ, ako je potrebno
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final createAPIPageData = CreateAPIPageData(); // CreateAPIPageData | 

try {
    final result = api_instance.addPage(tenantId, createAPIPageData);
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->addPage: $e\n');
}
[inline-code-end]

---
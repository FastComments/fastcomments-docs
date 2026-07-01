## Parametri

| Ime | Tip | Lokacija | Obavezno | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| limit | number | query | No |  |
| skip | number | query | No |  |
| order | string | query | No |  |
| after | number | query | No |  |
| before | number | query | No |  |

## Odgovor

Vraća: `GetAuditLogsResponse`

## Primer

[inline-code-attrs-start title = 'Primer getAuditLogs'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO Konfigurirajte autorizaciju API ključa: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// otkomentarišite dolje da postavite prefiks (npr. Bearer) za API ključ, ako je potrebno
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final limit = 1.2; // double | 
final skip = 1.2; // double | 
final order = ; // SORTDIR | 
final after = 1.2; // double | 
final before = 1.2; // double | 

try {
    final result = api_instance.getAuditLogs(tenantId, GetAuditLogsOptions(limit: limit, skip: skip, order: order, after: after, before: before));
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->getAuditLogs: $e\n');
}
[inline-code-end]
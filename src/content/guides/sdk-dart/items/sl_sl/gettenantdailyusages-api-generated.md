## Parameters

| Ime | Tip | Lokacija | Obvezno | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | query | Da |  |
| yearNumber | number | query | Ne |  |
| monthNumber | number | query | Ne |  |
| dayNumber | number | query | Ne |  |
| skip | number | query | Ne |  |

## Odgovor

Vrne: `GetTenantDailyUsagesResponse`

## Primer

[inline-code-attrs-start title = 'getTenantDailyUsages Primer'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO Nastavite avtorizacijo API ključa: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// odkomentirajte spodaj, da nastavite predpono (npr. Bearer) za API ključ, če je potrebno
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final yearNumber = 1.2; // double | 
final monthNumber = 1.2; // double | 
final dayNumber = 1.2; // double | 
final skip = 1.2; // double | 

try {
    final result = api_instance.getTenantDailyUsages(tenantId, GetTenantDailyUsagesOptions(yearNumber: yearNumber, monthNumber: monthNumber, dayNumber: dayNumber, skip: skip));
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->getTenantDailyUsages: $e\n');
}
[inline-code-end]
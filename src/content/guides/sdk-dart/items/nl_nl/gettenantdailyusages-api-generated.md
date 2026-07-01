## Parameters

| Naam | Type | Locatie | Verplicht | Beschrijving |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ja |  |
| yearNumber | number | query | Nee |  |
| monthNumber | number | query | Nee |  |
| dayNumber | number | query | Nee |  |
| skip | number | query | Nee |  |

## Respons

Retourneert: `GetTenantDailyUsagesResponse`

## Voorbeeld

[inline-code-attrs-start title = 'getTenantDailyUsages Voorbeeld'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO Configureer API-sleutel autorisatie: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// verwijder de ondercommentaar om prefix in te stellen (bijv. Bearer) voor API-sleutel, indien nodig
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
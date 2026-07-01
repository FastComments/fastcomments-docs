## Παράμετροι

| Όνομα | Τύπος | Τοποθεσία | Απαιτείται | Περιγραφή |
|------|------|----------|------------|-----------|
| tenantId | string | query | Ναι |  |
| yearNumber | number | query | Όχι |  |
| monthNumber | number | query | Όχι |  |
| dayNumber | number | query | Όχι |  |
| skip | number | query | Όχι |  |

## Απόκριση

Επιστρέφει: `GetTenantDailyUsagesResponse`

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα getTenantDailyUsages'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO Διαμόρφωση εξουσιοδότησης κλειδιού API: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// uncomment below to setup prefix (e.g. Bearer) for API key, if needed
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
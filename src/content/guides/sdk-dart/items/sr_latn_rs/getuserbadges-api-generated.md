## Parametri

| Naziv | Tip | Lokacija | Obavezno | Opis |
|------|------|----------|----------|------|
| tenantId | string | query | Da |  |
| userId | string | query | Ne |  |
| badgeId | string | query | Ne |  |
| type | number | query | Ne |  |
| displayedOnComments | boolean | query | Ne |  |
| limit | number | query | Ne |  |
| skip | number | query | Ne |  |

## Odgovor

Vraća: `APIGetUserBadgesResponse`

## Primer

[inline-code-attrs-start title = 'getUserBadges Primer'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO Konfigurišite autorizaciju API ključa: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// odkomentarišite dole da podesite prefiks (npr. Bearer) za API ključ, ako je potrebno
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final userId = userId_example; // String | 
final badgeId = badgeId_example; // String | 
final type = 1.2; // double | 
final displayedOnComments = true; // bool | 
final limit = 1.2; // double | 
final skip = 1.2; // double | 

try {
    final result = api_instance.getUserBadges(tenantId, GetUserBadgesOptions(userId: userId, badgeId: badgeId, type: type, displayedOnComments: displayedOnComments, limit: limit, skip: skip));
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->getUserBadges: $e\n');
}
[inline-code-end]
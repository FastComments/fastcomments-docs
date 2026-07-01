## Parameter

| Name | Typ | Location | Erforderlich | Beschreibung |
|------|------|----------|--------------|---------------|
| tenantId | string | query | Ja |  |
| userId | string | query | Nein |  |
| limit | number | query | Nein |  |
| skip | number | query | Nein |  |

## Antwort

Returns: `APIGetUserBadgeProgressListResponse`

## Beispiel

[inline-code-attrs-start title = 'Beispiel für getUserBadgeProgressList'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO API-Schlüssel-Autorisierung konfigurieren: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// Entkommentieren Sie die folgende Zeile, um das Präfix (z. B. Bearer) für den API-Schlüssel einzurichten, falls erforderlich
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final userId = userId_example; // String | 
final limit = 1.2; // double | 
final skip = 1.2; // double | 

try {
    final result = api_instance.getUserBadgeProgressList(tenantId, GetUserBadgeProgressListOptions(userId: userId, limit: limit, skip: skip));
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->getUserBadgeProgressList: $e\n');
}
[inline-code-end]
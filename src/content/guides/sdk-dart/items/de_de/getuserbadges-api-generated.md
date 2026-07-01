## Parameter

| Name | Typ | Ort | Erforderlich | Beschreibung |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ja |  |
| userId | string | query | Nein |  |
| badgeId | string | query | Nein |  |
| type | number | query | Nein |  |
| displayedOnComments | boolean | query | Nein |  |
| limit | number | query | Nein |  |
| skip | number | query | Nein |  |

## Antwort

Rückgabe: `APIGetUserBadgesResponse`

## Beispiel

[inline-code-attrs-start title = 'getUserBadges Beispiel'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO API-Schlüssel-Authentifizierung konfigurieren: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// Entfernen Sie das Kommentarzeichen unten, um das Präfix (z. B. Bearer) für den API-Schlüssel einzurichten, falls benötigt
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
## Parametre

| Navn | Type | Placering | Krævet | Beskrivelse |
|------|------|----------|--------|-------------|
| tenantId | string | query | Ja |  |
| sso | string | query | Nej |  |

## Svar

Returnerer: `APIModerateGetUserBanPreferencesResponse`

## Eksempel

[inline-code-attrs-start title = 'getUserBanPreference Eksempel'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = ModerationApi();
final tenantId = tenantId_example; // String | 
final sso = sso_example; // String | 

try {
    final result = api_instance.getUserBanPreference(tenantId, sso);
    print(result);
} catch (e) {
    print('Exception when calling ModerationApi->getUserBanPreference: $e\n');
}
[inline-code-end]
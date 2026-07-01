## Parameters

| Naam | Type | Locatie | Vereist | Beschrijving |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ja |  |
| value | string | query | Nee |  |
| sso | string | query | Nee |  |

## Respons

Retourneert: `ModerationSiteSearchResponse`

## Voorbeeld

[inline-code-attrs-start title = 'getSearchSites Voorbeeld'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = ModerationApi();
final tenantId = tenantId_example; // String | 
final value = value_example; // String | 
final sso = sso_example; // String | 

try {
    final result = api_instance.getSearchSites(tenantId, GetSearchSitesOptions(value: value, sso: sso));
    print(result);
} catch (e) {
    print('Exception when calling ModerationApi->getSearchSites: $e\n');
}
[inline-code-end]
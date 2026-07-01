## Parametri

| Nome | Tipo | Posizione | Obbligatorio | Descrizione |
|------|------|----------|--------------|-------------|
| tenantId | string | query | Sì |  |
| value | string | query | No |  |
| sso | string | query | No |  |

## Risposta

Restituisce: `ModerationSiteSearchResponse`

## Esempio

[inline-code-attrs-start title = 'Esempio getSearchSites'; type = ''; isFunctional = false; inline-code-attrs-end]
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

---
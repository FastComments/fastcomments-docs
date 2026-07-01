## Parametri

| Naziv | Tip | Lokacija | Obavezno | Opis |
|------|------|----------|----------|------|
| tenantId | string | query | Da |  |
| sso | string | query | Ne |  |

## Odgovor

Vraća: `GetTenantManualBadgesResponse`

## Primjer

[inline-code-attrs-start title = 'Primjer getManualBadges'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = ModerationApi();
final tenantId = tenantId_example; // String | 
final sso = sso_example; // String | 

try {
    final result = api_instance.getManualBadges(tenantId, sso);
    print(result);
} catch (e) {
    print('Exception when calling ModerationApi->getManualBadges: $e\n');
}
[inline-code-end]

---
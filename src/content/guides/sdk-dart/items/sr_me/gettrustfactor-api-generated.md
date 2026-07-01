## Parametri

| Ime | Tip | Lokacija | Obavezno | Opis |
|------|------|----------|----------|------|
| tenantId | string | upit | Da |  |
| userId | string | upit | Ne |  |
| sso | string | upit | Ne |  |

## Odgovor

Vraća: `GetUserTrustFactorResponse`

## Primjer

[inline-code-attrs-start title = 'Primjer getTrustFactor'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = ModerationApi();
final tenantId = tenantId_example; // String | 
final userId = userId_example; // String | 
final sso = sso_example; // String | 

try {
    final result = api_instance.getTrustFactor(tenantId, GetTrustFactorOptions(userId: userId, sso: sso));
    print(result);
} catch (e) {
    print('Exception when calling ModerationApi->getTrustFactor: $e\n');
}
[inline-code-end]
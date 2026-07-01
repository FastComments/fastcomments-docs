## Parametri

| Naziv | Tip | Lokacija | Obavezno | Opis |
|------|------|----------|----------|------|
| tenantId | string | query | Da |  |
| userId | string | query | Ne |  |
| trustFactor | string | query | Ne |  |
| sso | string | query | Ne |  |

## Odgovor

Returns: `SetUserTrustFactorResponse`

## Primjer

[inline-code-attrs-start title = 'setTrustFactor Primjer'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = ModerationApi();
final tenantId = tenantId_example; // String | 
final userId = userId_example; // String | 
final trustFactor = trustFactor_example; // String | 
final sso = sso_example; // String | 

try {
    final result = api_instance.setTrustFactor(tenantId, SetTrustFactorOptions(userId: userId, trustFactor: trustFactor, sso: sso));
    print(result);
} catch (e) {
    print('Exception when calling ModerationApi->setTrustFactor: $e\n');
}
[inline-code-end]
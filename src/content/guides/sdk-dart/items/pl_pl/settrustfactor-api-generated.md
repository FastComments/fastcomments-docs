## Parameters

| Nazwa | Typ | Lokalizacja | Wymagane | Opis |
|------|------|------------|----------|------|
| tenantId | string | query | Tak |  |
| userId | string | query | Nie |  |
| trustFactor | string | query | Nie |  |
| sso | string | query | Nie |  |

## Odpowiedź

Zwraca: `SetUserTrustFactorResponse`

## Przykład

[inline-code-attrs-start title = 'Przykład setTrustFactor'; type = ''; isFunctional = false; inline-code-attrs-end]
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
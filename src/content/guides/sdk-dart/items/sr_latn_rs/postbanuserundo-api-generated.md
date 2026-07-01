## Parametri

| Naziv | Tip | Lokacija | Obavezno | Opis |
|------|------|----------|----------|------|
| tenantId | string | query | Da |  |
| sso | string | query | Ne |  |

## Odgovor

Vraća: `APIEmptyResponse`

## Primer

[inline-code-attrs-start title = 'Primer postBanUserUndo'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = ModerationApi();
final tenantId = tenantId_example; // String | 
final banUserUndoParams = BanUserUndoParams(); // BanUserUndoParams | 
final sso = sso_example; // String | 

try {
    final result = api_instance.postBanUserUndo(tenantId, banUserUndoParams, sso);
    print(result);
} catch (e) {
    print('Exception when calling ModerationApi->postBanUserUndo: $e\n');
}
[inline-code-end]
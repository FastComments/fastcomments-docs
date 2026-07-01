## Paramètres

| Nom | Type | Emplacement | Obligatoire | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| sso | string | query | No |  |

## Réponse

Retourne : `APIEmptyResponse`

## Exemple

[inline-code-attrs-start title = 'postBanUserUndo Exemple'; type = ''; isFunctional = false; inline-code-attrs-end]
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
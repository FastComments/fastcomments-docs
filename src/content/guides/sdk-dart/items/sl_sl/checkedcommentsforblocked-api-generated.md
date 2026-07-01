## Parametri

| Ime | Tip | Lokacija | Obvezno | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | query | Da |  |
| commentIds | string | query | Da | Seznam ID-jev komentarjev, ločenih z vejicami. |
| sso | string | query | Ne |  |

## Odgovor

Vrne: `CheckBlockedCommentsResponse`

## Primer

[inline-code-attrs-start title = 'checkedCommentsForBlocked Primer'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = PublicApi();
final tenantId = tenantId_example; // String | 
final commentIds = commentIds_example; // String | Seznam ID-jev komentarjev, ločenih z vejicami.
final sso = sSO_example; // String | 

try {
    final result = api_instance.checkedCommentsForBlocked(tenantId, commentIds, sso);
    print(result);
} catch (e) {
    print('Exception when calling PublicApi->checkedCommentsForBlocked: $e\n');
}
[inline-code-end]

---
## Parametry

| Nazwa | Typ | Lokalizacja | Wymagane | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| commentIds | string | query | Yes | Lista identyfikatorów komentarzy oddzielonych przecinkami. |
| sso | string | query | No |  |

## Odpowiedź

Zwraca: `CheckBlockedCommentsResponse`

## Przykład

[inline-code-attrs-start title = 'checkedCommentsForBlocked Przykład'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = PublicApi();
final tenantId = tenantId_example; // String | 
final commentIds = commentIds_example; // String | Lista identyfikatorów komentarzy oddzielonych przecinkami.
final sso = sso_example; // String | 

try {
    final result = api_instance.checkedCommentsForBlocked(tenantId, commentIds, sso);
    print(result);
} catch (e) {
    print('Wyjątek podczas wywoływania PublicApi->checkedCommentsForBlocked: $e\n');
}
[inline-code-end]
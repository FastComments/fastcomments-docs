## Παράμετροι

| Όνομα | Τύπος | Τοποθεσία | Απαιτείται | Περιγραφή |
|------|------|----------|------------|------------|
| tenantId | string | query | Ναι |  |
| commentIds | string | query | Ναι | Μια λίστα αναγνωριστικών σχολίων χωρισμένη με κόμματα. |
| sso | string | query | Όχι |  |

## Απάντηση

Returns: `CheckBlockedCommentsResponse`

## Παράδειγμα

[inline-code-attrs-start title = 'checkedCommentsForBlocked Παράδειγμα'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = PublicApi();
final tenantId = tenantId_example; // String | 
final commentIds = commentIds_example; // String | Μια λίστα αναγνωριστικών σχολίων χωρισμένη με κόμματα.
final sso = sso_example; // String | 

try {
    final result = api_instance.checkedCommentsForBlocked(tenantId, commentIds, sso);
    print(result);
} catch (e) {
    print('Exception when calling PublicApi->checkedCommentsForBlocked: $e\n');
}
[inline-code-end]
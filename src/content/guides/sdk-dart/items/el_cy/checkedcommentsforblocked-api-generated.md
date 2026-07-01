## Παράμετροι

| Όνομα | Τύπος | Θέση | Απαιτείται | Περιγραφή |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| commentIds | string | query | Yes | Μία λίστα διαχωρισμένη με κόμματα των αναγνωριστικών σχολίων. |
| sso | string | query | No |  |

## Απόκριση

Returns: `CheckBlockedCommentsResponse`

## Παράδειγμα

[inline-code-attrs-start title = 'checkedCommentsForBlocked Παράδειγμα'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = PublicApi();
final tenantId = tenantId_example; // String | 
final commentIds = commentIds_example; // String | Μία λίστα διαχωρισμένη με κόμματα των αναγνωριστικών σχολίων.
final sso = sso_example; // String | 

try {
    final result = api_instance.checkedCommentsForBlocked(tenantId, commentIds, sso);
    print(result);
} catch (e) {
    print('Exception when calling PublicApi->checkedCommentsForBlocked: $e\n');
}
[inline-code-end]
## Παράμετροι

| Όνομα | Τύπος | Θέση | Απαιτείται | Περιγραφή |
|------|------|----------|----------|-------------|
| tenantId | string | path | Ναι |  |
| urlId | string | query | Ναι |  |

## Απόκριση

Επιστρέφει: `CreateV1PageReact`

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα deleteV1PageReact'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = PublicApi();
final tenantId = tenantId_example; // String | 
final urlId = urlId_example; // String | 

try {
    final result = api_instance.deleteV1PageReact(tenantId, urlId);
    print(result);
} catch (e) {
    print('Exception when calling PublicApi->deleteV1PageReact: $e\n');
}
[inline-code-end]

---
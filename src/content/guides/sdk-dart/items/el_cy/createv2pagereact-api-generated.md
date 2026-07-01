## Παράμετροι

| Όνομα | Τύπος | Τοποθεσία | Απαιτείται | Περιγραφή |
|------|------|----------|------------|-------------|
| tenantId | string | path | Ναι |  |
| urlId | string | query | Ναι |  |
| id | string | query | Ναι |  |
| title | string | query | Όχι |  |

## Απόκριση

Επιστρέφει: `CreateV1PageReact`

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα createV2PageReact'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = PublicApi();
final tenantId = tenantId_example; // String | 
final urlId = urlId_example; // String | 
final id = id_example; // String | 
final title = title_example; // String | 

try {
    final result = api_instance.createV2PageReact(tenantId, urlId, id, title);
    print(result);
} catch (e) {
    print('Exception when calling PublicApi->createV2PageReact: $e\n');
}
[inline-code-end]
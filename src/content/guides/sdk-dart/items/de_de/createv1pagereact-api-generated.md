## Parameter

| Name | Typ | Ort | Erforderlich | Beschreibung |
|------|------|-----|--------------|---------------|
| tenantId | string | path | Ja |  |
| urlId | string | query | Ja |  |
| title | string | query | Nein |  |

## Antwort

Rückgabe: `CreateV1PageReact`

## Beispiel

[inline-code-attrs-start title = 'createV1PageReact Beispiel'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = PublicApi();
final tenantId = tenantId_example; // String | 
final urlId = urlId_example; // String | 
final title = title_example; // String | 

try {
    final result = api_instance.createV1PageReact(tenantId, urlId, title);
    print(result);
} catch (e) {
    print('Exception when calling PublicApi->createV1PageReact: $e\n');
}
[inline-code-end]
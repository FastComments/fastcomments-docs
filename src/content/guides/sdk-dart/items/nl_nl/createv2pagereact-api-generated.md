## Parameters

| Naam | Type | Locatie | Verplicht | Beschrijving |
|------|------|----------|-----------|--------------|
| tenantId | string | path | Ja |  |
| urlId | string | query | Ja |  |
| id | string | query | Ja |  |
| title | string | query | Nee |  |

## Respons

Retourneert: `CreateV1PageReact`

## Voorbeeld

[inline-code-attrs-start title = 'createV2PageReact Voorbeeld'; type = ''; isFunctional = false; inline-code-attrs-end]
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
    print('Uitzondering bij het aanroepen van PublicApi->createV2PageReact: $e\n');
}
[inline-code-end]
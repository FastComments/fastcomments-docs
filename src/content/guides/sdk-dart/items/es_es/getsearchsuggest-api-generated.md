## Parámetros

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Sí |  |
| text-search | string | query | No |  |
| sso | string | query | No |  |

## Respuesta

Devuelve: `ModerationSuggestResponse`

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo getSearchSuggest'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = ModerationApi();
final tenantId = tenantId_example; // String | 
final textSearch = textSearch_example; // String | 
final sso = sso_example; // String | 

try {
    final result = api_instance.getSearchSuggest(tenantId, GetSearchSuggestOptions(textSearch: textSearch, sso: sso));
    print(result);
} catch (e) {
    print('Exception when calling ModerationApi->getSearchSuggest: $e\n');
}
[inline-code-end]
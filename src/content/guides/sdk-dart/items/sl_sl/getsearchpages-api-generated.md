## Parametri

| Ime | Vrsta | Lokacija | Obvezno | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | query | Da |  |
| value | string | query | Ne |  |
| sso | string | query | Ne |  |

## Odziv

Returns: `ModerationPageSearchResponse`

## Primer

[inline-code-attrs-start title = 'Primer getSearchPages'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = ModerationApi();
final tenantId = tenantId_example; // String | 
final value = value_example; // String | 
final sso = sso_example; // String | 

try {
    final result = api_instance.getSearchPages(tenantId, GetSearchPagesOptions(value: value, sso: sso));
    print(result);
} catch (e) {
    print('Exception when calling ModerationApi->getSearchPages: $e\n');
}
[inline-code-end]
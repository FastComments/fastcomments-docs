## Parametri

| Naziv | Tip | Lokacija | Obavezno | Opis |
|------|------|----------|----------|------|
| tenantId | string | query | Yes |  |
| value | string | query | No |  |
| sso | string | query | No |  |

## Odgovor

Vraća: `ModerationPageSearchResponse`

## Primjer

[inline-code-attrs-start title = 'Primjer getSearchPages'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = ModerationApi();
final tenantId = tenantId_example; // String | 
final value = value_example; // String | 
final sno = sso_example; // String | 

try {
    final result = api_instance.getSearchPages(tenantId, GetSearchPagesOptions(value: value, sso: sno));
    print(result);
} catch (e) {
    print('Exception when calling ModerationApi->getSearchPages: $e\n');
}
[inline-code-end]
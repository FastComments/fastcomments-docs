## Parametri

| Ime | Tip | Lokacija | Obvezno | Opis |
|------|------|----------|----------|-------------|
| namespace | string | path | Yes |  |
| component | string | path | Yes |  |
| locale | string | query | No |  |
| useFullTranslationIds | boolean | query | No |  |

## Odziv

Returns: `GetTranslationsResponse`

## Primer

[inline-code-attrs-start title = 'Primer getTranslations'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = PublicApi();
final namespace = namespace_example; // Niz |
final component = component_example; // Niz |
final locale = locale_example; // Niz |
final useFullTranslationIds = true; // bool | 

try {
    final result = api_instance.getTranslations(namespace, component, GetTranslationsOptions(locale: locale, useFullTranslationIds: useFullTranslationIds));
    print(result);
} catch (e) {
    print('Exception when calling PublicApi->getTranslations: $e\n');
}
[inline-code-end]
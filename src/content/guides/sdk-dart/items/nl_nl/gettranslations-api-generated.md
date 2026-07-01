## Parameters

| Naam | Type | Locatie | Vereist | Beschrijving |
|------|------|----------|----------|---------------|
| namespace | string | path | Ja |  |
| component | string | path | Ja |  |
| locale | string | query | Nee |  |
| useFullTranslationIds | boolean | query | Nee |  |

## Respons

Retourneert: `GetTranslationsResponse`

## Voorbeeld

[inline-code-attrs-start title = 'Voorbeeld van getTranslations'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = PublicApi();
final namespace = namespace_example; // String | 
final component = component_example; // String | 
final locale = locale_example; // String | 
final useFullTranslationIds = true; // bool | 

try {
    final result = api_instance.getTranslations(namespace, component, GetTranslationsOptions(locale: locale, useFullTranslationIds: useFullTranslationIds));
    print(result);
} catch (e) {
    print('Exception when calling PublicApi->getTranslations: $e\n');
}
[inline-code-end]

---
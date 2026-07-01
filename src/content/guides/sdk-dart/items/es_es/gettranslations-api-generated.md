## Parámetros

| Nombre | Tipo | Ubicación | Obligatorio | Descripción |
|------|------|----------|-------------|-------------|
| namespace | string | path | Sí |  |
| component | string | path | Sí |  |
| locale | string | query | No |  |
| useFullTranslationIds | boolean | query | No |  |

## Respuesta

Devuelve: `GetTranslationsResponse`

## Ejemplo

[inline-code-attrs-start title = 'getTranslations Ejemplo'; type = ''; isFunctional = false; inline-code-attrs-end]
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
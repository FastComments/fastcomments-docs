## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| namespace | string | path | Yes |  |
| component | string | path | Yes |  |
| locale | string | query | No |  |
| useFullTranslationIds | boolean | query | No |  |

## Response

Returns: `GetTranslationsResponse`

## Example

[inline-code-attrs-start title = 'Пример за getTranslations'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = PublicApi();
final namespace = namespace_example; // Низ |
final component = component_example; // Низ |
final locale = locale_example; // Низ |
final useFullTranslationIds = true; // булев |

try {
    final result = api_instance.getTranslations(namespace, component, GetTranslationsOptions(locale: locale, useFullTranslationIds: useFullTranslationIds));
    print(result);
} catch (e) {
    print('Exception when calling PublicApi->getTranslations: $e\n');
}
[inline-code-end]
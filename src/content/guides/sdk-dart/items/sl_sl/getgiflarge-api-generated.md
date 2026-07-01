## Parametri

| Ime | Vrsta | Lokacija | Obvezno | Opis |
|------|------|----------|----------|------|
| tenantId | string | path | Da |  |
| largeInternalURLSanitized | string | query | Da |  |

## Odgovor

Returns: `GifGetLargeResponse`

## Primer

[inline-code-attrs-start title = 'Primer getGifLarge'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = PublicApi();
final tenantId = tenantId_example; // String | 
final largeInternalURLSanitized = largeInternalURLSanitized_example; // String | 

try {
    final result = api_instance.getGifLarge(tenantId, largeInternalURLSanitized);
    print(result);
} catch (e) {
    print('Exception when calling PublicApi->getGifLarge: $e\n');
}
[inline-code-end]
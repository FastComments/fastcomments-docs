Naloži in spremeni velikost slike

## Parametri

| Ime | Vrsta | Mesto | Obvezno | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| sizePreset | string | query | No | Vnaprej določena velikost: \"Default\" (1000x1000px) ali \"CrossPlatform\" (ustvari velikosti za priljubljene naprave) |
| urlId | string | query | No | ID strani, iz katere se izvaja naložitev, za konfiguracijo |

## Odgovor

Vrne: `UploadImageResponse`

## Primer

[inline-code-attrs-start title = 'uploadImage Primer'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = PublicApi();
final tenantId = tenantId_example; // String | 
final file = BINARY_DATA_HERE; // MultipartFile | 
final sizePreset = ; // SizePreset | Vnaprej določena velikost: \"Default\" (1000x1000px) ali \"CrossPlatform\" (ustvari velikosti za priljubljene naprave)
final urlId = urlId_example; // String | ID strani, iz katere se izvaja naložitev, za konfiguracijo

try {
    final result = api_instance.uploadImage(tenantId, file, UploadImageOptions(sizePreset: sizePreset, urlId: urlId));
    print(result);
} catch (e) {
    print('Exception when calling PublicApi->uploadImage: $e\n');
}
[inline-code-end]

---
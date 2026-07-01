Upload and resize an image
============================

## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| sizePreset | string | query | No | Preset di dimensione: "Default" (1000x1000px) o "CrossPlatform" (crea dimensioni per dispositivi popolari) |
| urlId | string | query | No | ID della pagina da cui avviene il caricamento, da configurare |

## Response

Returns: `UploadImageResponse`

## Example

[inline-code-attrs-start title = 'Esempio uploadImage'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = PublicApi();
final tenantId = tenantId_example; // String | 
final file = BINARY_DATA_HERE; // MultipartFile | 
final sizePreset = ; // SizePreset | Preset di dimensione: \"Default\" (1000x1000px) o \"CrossPlatform\" (crea dimensioni per dispositivi popolari)
final urlId = urlId_example; // String | ID della pagina da cui avviene il caricamento, da configurare

try {
    final result = api_instance.uploadImage(tenantId, file, UploadImageOptions(sizePreset: sizePreset, urlId: urlId));
    print(result);
} catch (e) {
    print('Exception when calling PublicApi->uploadImage: $e\n');
}
[inline-code-end]
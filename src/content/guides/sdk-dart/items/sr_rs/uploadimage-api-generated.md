Upload and resize an image

## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| sizePreset | string | query | No | Претподешавање величине: \"Default\" (1000x1000px) или \"CrossPlatform\" (прави величине за популарне уређаје) |
| urlId | string | query | No | Идентификатор странице са које се врши отпремање, за конфигурацију |

## Response

Враћа: `UploadImageResponse`

## Example

[inline-code-attrs-start title = 'Пример uploadImage'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = PublicApi();
final tenantId = tenantId_example; // String | 
final file = BINARY_DATA_HERE; // MultipartFile | 
final sizePreset = ; // SizePreset | Претподешавање величине: \"Default\" (1000x1000px) или \"CrossPlatform\" (прави величине за популарне уређаје)
final urlId = urlId_example; // String | Идентификатор странице са које се врши отпремање, за конфигурацију

try {
    final result = api_instance.uploadImage(tenantId, file, UploadImageOptions(sizePreset: sizePreset, urlId: urlId));
    print(result);
} catch (e) {
    print('Exception when calling PublicApi->uploadImage: $e\n');
}
[inline-code-end]
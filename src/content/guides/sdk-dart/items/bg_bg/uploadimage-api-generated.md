Upload and resize an image
==========================

## Parameters

| Име | Тип | Местоположение | Задължително | Описание |
|------|------|----------------|-------------|----------|
| tenantId | string | path | Yes |  |
| sizePreset | string | query | No | Предварително зададен размер: \"Default\" (1000x1000px) или \"CrossPlatform\" (създава размери за популярни устройства) |
| urlId | string | query | No | Идентификатор на страницата, от която се извършва качването, за конфигуриране |

## Response

Връща: `UploadImageResponse`

## Example

[inline-code-attrs-start title = 'Пример за uploadImage'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = PublicApi();
final tenantId = tenantId_example; // String | 
final file = BINARY_DATA_HERE; // MultipartFile | 
final sizePreset = ; // SizePreset | Предварително зададен размер: \"Default\" (1000x1000px) или \"CrossPlatform\" (създава размери за популярни устройства)
final urlId = urlId_example; // String | Идентификатор на страницата, от която се извършва качването, за конфигуриране

try {
    final result = api_instance.uploadImage(tenantId, file, UploadImageOptions(sizePreset: sizePreset, urlId: urlId));
    print(result);
} catch (e) {
    print('Exception when calling PublicApi->uploadImage: $e\n');
}
[inline-code-end]
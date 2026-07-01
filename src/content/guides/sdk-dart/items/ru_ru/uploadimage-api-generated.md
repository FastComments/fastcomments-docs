Upload and resize an image → Загрузка и изменение размера изображения

## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| sizePreset | string | query | No | Предустановленный размер: "Default" (1000x1000px) или "CrossPlatform" (создаёт размеры для популярных устройств) |
| urlId | string | query | No | Идентификатор страницы, из которой происходит загрузка, для конфигурации |

## Response

Возвращает: `UploadImageResponse`

## Example

[inline-code-attrs-start title = 'uploadImage Пример'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = PublicApi();
final tenantId = tenantId_example; // String | 
final file = BINARY_DATA_HERE; // MultipartFile | 
final sizePreset = ; // SizePreset | Предустановленный размер: \"Default\" (1000x1000px) или \"CrossPlatform\" (создаёт размеры для популярных устройств)
final urlId = urlId_example; // String | Идентификатор страницы, из которой происходит загрузка, для конфигурации

try {
    final result = api_instance.uploadImage(tenantId, file, UploadImageOptions(sizePreset: sizePreset, urlId: urlId));
    print(result);
} catch (e) {
    print('Exception when calling PublicApi->uploadImage: $e\n');
}
[inline-code-end]
---
画像のアップロードとリサイズ

## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| sizePreset | string | query | No | サイズプリセット: "Default" (1000x1000px) または "CrossPlatform" (人気デバイス用にサイズを作成) |
| urlId | string | query | No | 設定用のアップロードが行われているページ ID |

## Response

返り値: `UploadImageResponse`

## Example

[inline-code-attrs-start title = 'uploadImage 例'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = PublicApi();
final tenantId = tenantId_example; // String | 
final file = BINARY_DATA_HERE; // MultipartFile | 
final sizePreset = ; // SizePreset | サイズプリセット: \"Default\" (1000x1000px) または \"CrossPlatform\" (人気デバイス用にサイズを作成)
final urlId = urlId_example; // String | 設定用のアップロードが行われているページ ID

try {
    final result = api_instance.uploadImage(tenantId, file, UploadImageOptions(sizePreset: sizePreset, urlId: urlId));
    print(result);
} catch (e) {
    print('Exception when calling PublicApi->uploadImage: $e\n');
}
[inline-code-end]

---
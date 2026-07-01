上傳並調整圖像大小

## 參數

| 名稱 | 類型 | 位置 | 必要 | 描述 |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| sizePreset | string | query | No | 尺寸預設: "Default" (1000x1000像素) 或 "CrossPlatform" (為流行裝置建立尺寸) |
| urlId | string | query | No | 上傳發生的頁面 ID，用於配置 |

## 回應

Returns: `UploadImageResponse`

## 範例

[inline-code-attrs-start title = 'uploadImage 範例'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = PublicApi();
final tenantId = tenantId_example; // String | 
final file = BINARY_DATA_HERE; // MultipartFile | 
final sizePreset = ; // SizePreset | 尺寸預設: "Default" (1000x1000像素) 或 "CrossPlatform" (為流行裝置建立尺寸)
final urlId = urlId_example; // String | 上傳發生的頁面 ID，用於配置

try {
    final result = api_instance.uploadImage(tenantId, file, UploadImageOptions(sizePreset: sizePreset, urlId: urlId));
    print(result);
} catch (e) {
    print('Exception when calling PublicApi->uploadImage: $e\n');
}
[inline-code-end]
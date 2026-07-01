上传并调整图像大小

## 参数

| 名称 | 类型 | 位置 | 必需 | 描述 |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| sizePreset | string | query | No | 大小预设："Default" (1000x1000px) 或 "CrossPlatform" (为流行设备创建尺寸) |
| urlId | string | query | No | 页面 ID，上传发生的来源，以进行配置 |

## 返回

返回： `UploadImageResponse`

## 示例

[inline-code-attrs-start title = 'uploadImage 示例'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = PublicApi();
final tenantId = tenantId_example; // String | 
final file = BINARY_DATA_HERE; // MultipartFile | 
final sizePreset = ; // SizePreset | 大小预设：\"Default\" (1000x1000px) 或 \"CrossPlatform\" (为流行设备创建尺寸)
final urlId = urlId_example; // String | 页面 ID，上传发生的来源，以进行配置

try {
    final result = api_instance.uploadImage(tenantId, file, UploadImageOptions(sizePreset: sizePreset, urlId: urlId));
    print(result);
} catch (e) {
    print('Exception when calling PublicApi->uploadImage: $e\n');
}
[inline-code-end]
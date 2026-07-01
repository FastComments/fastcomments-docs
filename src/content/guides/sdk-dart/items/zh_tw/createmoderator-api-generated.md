## 參數

| 名稱 | 類型 | 位置 | 必填 | 說明 |
|------|------|----------|----------|-------------|
| tenantId | string | query | 是 |  |

## 回應

返回： `CreateModeratorResponse`

## 範例

[inline-code-attrs-start title = 'createModerator 範例'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO 配置 API 金鑰授權: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// 取消下列註解以設定 API 金鑰前綴 (例如 Bearer)，若有需要
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final createModeratorBody = CreateModeratorBody(); // CreateModeratorBody | 

try {
    final result = api_instance.createModerator(tenantId, createModeratorBody);
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->createModerator: $e\n');
}
[inline-code-end]
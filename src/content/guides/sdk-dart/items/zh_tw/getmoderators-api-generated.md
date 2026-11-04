## 參數

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | 是 |  |
| skip | number | query | 否 |  |

## 回應

返回： `GetModeratorsResponse`

## 範例

[inline-code-attrs-start title = 'getModerators 範例'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO 設定 API 金鑰授權: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// 如有需要，取消註解以下以設定 API 金鑰的前綴 (例如 Bearer)
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final skip = 1.2; // double | 

try {
    final result = api_instance.getModerators(tenantId, skip);
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->getModerators: $e\n');
}
[inline-code-end]
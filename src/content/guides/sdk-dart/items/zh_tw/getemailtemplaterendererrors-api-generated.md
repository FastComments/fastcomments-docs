## 參數

| 名稱 | 類型 | 位置 | 必填 | 說明 |
|------|------|----------|------|-------------|
| tenantId | string | query | 是 |  |
| id | string | path | 是 |  |
| skip | number | query | 否 |  |

## 回應

返回： `GetEmailTemplateRenderErrorsResponse`

## 範例

[inline-code-attrs-start title = 'getEmailTemplateRenderErrors 範例'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO 配置 API 金鑰授權: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// 取消註解以下行以設定 API 金鑰的前綴 (例如 Bearer)，如果需要的話
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final id = id_example; // String | 
final skip = 1.2; // double | 

try {
    final result = api_instance.getEmailTemplateRenderErrors(tenantId, id, skip);
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->getEmailTemplateRenderErrors: $e\n');
}
[inline-code-end]
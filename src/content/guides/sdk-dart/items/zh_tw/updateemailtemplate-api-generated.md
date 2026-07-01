## 參數

| 名稱 | 類型 | 位置 | 必填 | 說明 |
|------|------|----------|----------|-------------|
| tenantId | string | 查詢 | 是 |  |
| id | string | 路徑 | 是 |  |

## 回應

返回：`APIEmptyResponse`

## 範例

[inline-code-attrs-start title = 'updateEmailTemplate 範例'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO 配置 API 金鑰授權: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// 取消註解以下以設定前綴 (例如 Bearer) 給 API 金鑰，如有需要
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final id = id_example; // String | 
final updateEmailTemplateBody = UpdateEmailTemplateBody(); // UpdateEmailTemplateBody | 

try {
    final result = api_instance.updateEmailTemplate(tenantId, id, updateEmailTemplateBody);
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->updateEmailTemplate: $e\n');
}
[inline-code-end]
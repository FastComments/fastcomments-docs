## 參數

| 名稱 | 類型 | 位置 | 必需 | 描述 |
|------|------|----------|----------|-------------|
| tenantId | string | query | 是 |  |
| id | string | path | 是 |  |
| sendEmail | string | query | 否 |  |

## 回應

Returns: `APIEmptyResponse`

## 範例

[inline-code-attrs-start title = 'deleteModerator 範例'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO 配置 API 金鑰授權: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// 取消註解以下以設定前綴（例如 Bearer）給 API 金鑰，如有需要
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final id = id_example; // String | 
final sendEmail = sendEmail_example; // String | 

try {
    final result = api_instance.deleteModerator(tenantId, id, sendEmail);
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->deleteModerator: $e\n');
}
[inline-code-end]

---
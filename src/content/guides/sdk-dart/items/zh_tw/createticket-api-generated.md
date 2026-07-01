## 參數

| 名稱 | 類型 | 位置 | 必要 | 描述 |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| userId | string | query | Yes |  |

## 回應

返回: `CreateTicketResponse`

## 範例

[inline-code-attrs-start title = 'createTicket 範例'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO 設定 API 金鑰授權：api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// 取消註解以下以設定前綴（例如 Bearer）給 API 金鑰，如有需要
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final userId = userId_example; // String | 
final createTicketBody = CreateTicketBody(); // CreateTicketBody | 

try {
    final result = api_instance.createTicket(tenantId, userId, createTicketBody);
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->createTicket: $e\n');
}
[inline-code-end]

---
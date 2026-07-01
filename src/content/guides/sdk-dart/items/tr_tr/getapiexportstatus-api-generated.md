## Parameters

| Ad | Tür | Konum | Zorunlu | Açıklama |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| batchJobId | string | query | No |  |
| sso | string | query | No |  |

## Yanıt

Döndürür: `ModerationExportStatusResponse`

## Örnek

[inline-code-attrs-start title = 'getApiExportStatus Örneği'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = ModerationApi();
final tenantId = tenantId_example; // String | 
final batchJobId = batchJobId_example; // String | 
final sso = sso_example; // String | 

try {
    final result = api_instance.getApiExportStatus(tenantId, GetApiExportStatusOptions(batchJobId: batchJobId, sso: sso));
    print(result);
} catch (e) {
    print('Exception when calling ModerationApi->getApiExportStatus: $e\n');
}
[inline-code-end]
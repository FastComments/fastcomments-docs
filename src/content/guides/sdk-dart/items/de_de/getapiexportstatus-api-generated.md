## Parameter

| Name      | Typ    | Ort    | Erforderlich | Beschreibung |
|-----------|--------|--------|--------------|--------------|
| tenantId  | string | query  | Ja           |  |
| batchJobId| string | query  | Nein         |  |
| sso       | string | query  | Nein         |  |

## Antwort

Rückgabe: `ModerationExportStatusResponse`

## Beispiel

[inline-code-attrs-start title = 'getApiExportStatus Beispiel'; type = ''; isFunctional = false; inline-code-attrs-end]
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
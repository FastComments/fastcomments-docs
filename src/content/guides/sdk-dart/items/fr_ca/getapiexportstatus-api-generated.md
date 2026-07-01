## Paramètres

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| batchJobId | string | query | No |  |
| sso | string | query | No |  |

## Réponse

Retourne : `ModerationExportStatusResponse`

## Exemple

[inline-code-attrs-start title = 'Exemple getApiExportStatus'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = ModerationApi();
final tenantId = tenantId_example; // Chaîne | 
final batchJobId = batchJobId_example; // Chaîne | 
final sso = sso_example; // Chaîne | 

try {
    final result = api_instance.getApiExportStatus(tenantId, GetApiExportStatusOptions(batchJobId: batchJobId, sso: sso));
    print(result);
} catch (e) {
    print('Exception lors de l\'appel de ModerationApi->getApiExportStatus: $e\n');
}
[inline-code-end]
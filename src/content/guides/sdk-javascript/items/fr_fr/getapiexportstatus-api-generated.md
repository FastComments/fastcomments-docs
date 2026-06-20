## Paramètres

| Nom | Type | Requis | Description |
|------|------|----------|-------------|
| batchJobId | string | Non |  |
| sso | string | Non |  |

## Réponse

Renvoie: [`ModerationExportStatusResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/ModerationExportStatusResponse.ts)

## Exemple

[inline-code-attrs-start title = 'Exemple de getApiExportStatus'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const statusByBatch: ModerationExportStatusResponse = await getApiExportStatus('export_20260615_84c2');
const statusBySso: ModerationExportStatusResponse = await getApiExportStatus(undefined, 'sso_84c2f1b2');
[inline-code-end]

---
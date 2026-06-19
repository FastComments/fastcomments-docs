## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| id | string | Ja |  |

## Antwort

Gibt zurück: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APIEmptyResponse.ts)

## Beispiel

[inline-code-attrs-start title = 'deleteTenantPackage Beispiel'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'f47ac10b-58cc-4372-a567-0e02b2c3d479';
const packageId: string = 'pkg_prod_delete_2026-06-19';
const onComplete: ((status?: APIStatus) => void) | undefined = undefined;
const response: APIEmptyResponse = await deleteTenantPackage(tenantId, packageId);
onComplete?.();
[inline-code-end]

---
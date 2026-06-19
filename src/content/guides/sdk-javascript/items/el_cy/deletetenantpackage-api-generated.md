## Παράμετροι

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Ναι |  |
| id | string | Ναι |  |

## Απόκριση

Επιστρέφει: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APIEmptyResponse.ts)

## Παράδειγμα

[inline-code-attrs-start title = 'deleteTenantPackage Παράδειγμα'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'f47ac10b-58cc-4372-a567-0e02b2c3d479';
const packageId: string = 'pkg_prod_delete_2026-06-19';
const onComplete: ((status?: APIStatus) => void) | undefined = undefined;
const response: APIEmptyResponse = await deleteTenantPackage(tenantId, packageId);
onComplete?.();
[inline-code-end]

---
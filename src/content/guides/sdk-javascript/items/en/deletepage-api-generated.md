## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | Yes |  |

## Response

Returns: [`DeletePageAPIResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/DeletePageAPIResponse.ts)

## Example

[inline-code-attrs-start title = 'deletePage Example'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function run(deleteImmediately?: boolean): Promise<void> {
  const tenantId: string = 'acme_corp_tenant_01';
  const id: string = 'page_5f8b3a9e';
  const result: DeletePageAPIResponse = await deletePage(tenantId, id);
  const logAction: boolean = deleteImmediately ?? false;
  if (logAction) console.log('Deleted page', id, result);
}
run();
[inline-code-end]

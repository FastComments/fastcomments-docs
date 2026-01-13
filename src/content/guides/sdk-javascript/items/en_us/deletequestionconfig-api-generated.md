## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | Yes |  |

## Response

Returns: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/FlagCommentPublic200Response.ts)

## Example

[inline-code-attrs-start title = 'deleteQuestionConfig Example'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function deleteIfPresent(tenantId: string, id?: string): Promise<FlagCommentPublic200Response | null> {
  if (!id) return null;
  const result: FlagCommentPublic200Response = await deleteQuestionConfig(tenantId, id);
  return result;
}
const tenantId: string = 'tenant_acme_001';
const optionalConfigId: string | undefined = 'qcfg_20260112_01';
(async (): Promise<void> => {
  const deleted: FlagCommentPublic200Response | null = await deleteIfPresent(tenantId, optionalConfigId);
  void deleted;
})();
[inline-code-end]

---
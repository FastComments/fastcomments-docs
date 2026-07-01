## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | Yes |  |

## Response

מחזיר: [`DeleteQuestionResultResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/DeleteQuestionResultResponse.ts)

## Example

[inline-code-attrs-start title = 'deleteQuestionResult דוגמה'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function runDeletion(): Promise<void> {
    const tenantId: string = "tenant-001";
    const resultId: string = "c9f2a5b3-7e6d-4c9a-8b1f-2d3e4f5a6b7c";
    const result: DeleteQuestionResultResponse = await deleteQuestionResult(tenantId, resultId);
    console.log(result);
}

runDeletion();
[inline-code-end]
## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | Yes |  |

## レスポンス

返り値: [`DeleteQuestionConfigResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/DeleteQuestionConfigResponse.ts)

## 例

[inline-code-attrs-start title = 'deleteQuestionConfig の例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function runDeletion(): Promise<void> {
  const tenantId: string = "tenant_8f5a2c9d";
  const configId: string = "questionConfig_4b7e1f";
  const deleteResult: DeleteQuestionConfigResponse = await deleteQuestionConfig(tenantId, configId);
  console.log(deleteResult);
}
void runDeletion();
[inline-code-end]
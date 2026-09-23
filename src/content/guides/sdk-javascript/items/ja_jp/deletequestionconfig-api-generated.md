## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenantId | string | はい |  |
| id | string | はい |  |

## レスポンス

返却: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APIEmptyResponse.ts)

## 例

[inline-code-attrs-start title = 'deleteQuestionConfig の例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function executeDeletion() {
  const tenantId: string = "tenant_12345";
  const configId: string = "config_9876";
  const result: APIEmptyResponse = await deleteQuestionConfig(tenantId, configId);
  console.log(result);
}
executeDeletion();
[inline-code-end]
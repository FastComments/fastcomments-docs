## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenantId | string | はい |  |
| id | string | はい |  |

## レスポンス

戻り値: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APIEmptyResponse.ts)

## 例

[inline-code-attrs-start title = 'deleteQuestionConfig の例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_4f2b3c';
const id: string = 'qcfg_9a8b7c';
const metadataNote: string | undefined = undefined; // オプションのメタデータ（関数では必須ではありません）
const result: APIEmptyResponse = await deleteQuestionConfig(tenantId, id);
metadataNote;
[inline-code-end]

---
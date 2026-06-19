## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenantId | string | はい |  |
| id | string | はい |  |
| fromName | string | はい |  |

## レスポンス

戻り値: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APIEmptyResponse.ts)

## 例

[inline-code-attrs-start title = 'sendInvite の例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'bright-media-12';
const id: string = 'user-8f4d2';
const fromName: string = 'Sofia Park';
const optionalNote: string | undefined = undefined;
const result: APIEmptyResponse = await sendInvite(tenantId, id, fromName);
[inline-code-end]

---
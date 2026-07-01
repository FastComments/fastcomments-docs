## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| urlId | string | Yes |  |
| id | string | Yes |  |

## レスポンス

戻り値: [`DeleteV2PageReactResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/DeleteV2PageReactResponse.ts)

## 例

[inline-code-attrs-start title = 'deleteV2PageReact の例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_12345";
const urlId: string = "page_98765";
const reactionId: string = "react_abcde";

const deleteResult: DeleteV2PageReactResponse = await deleteV2PageReact(tenantId, urlId, reactionId);
[inline-code-end]
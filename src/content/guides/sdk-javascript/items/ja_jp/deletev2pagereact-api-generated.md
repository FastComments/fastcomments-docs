## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| urlId | string | Yes |  |
| id | string | Yes |  |

## レスポンス

戻り値: [`DeleteV2PageReact200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/DeleteV2PageReact200Response.ts)

## 例

[inline-code-attrs-start title = 'deleteV2PageReact の例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_79021";
const urlId: string = "blog/my-first-post";
const id: string = "reaction_9f8b7c";
let includeHistory: boolean | undefined = undefined; // オプションのフラグ。いくつかの呼び出しで使用されます

const result: DeleteV2PageReact200Response = await deleteV2PageReact(tenantId, urlId, id);
console.log(result);
[inline-code-end]
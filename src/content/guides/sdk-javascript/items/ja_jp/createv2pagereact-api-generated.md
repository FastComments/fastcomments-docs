## パラメーター

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenantId | string | はい |  |
| urlId | string | はい |  |
| id | string | はい |  |
| title | string | いいえ |  |

## レスポンス

戻り値: [`CreateV2PageReact200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateV2PageReact200Response.ts)

## 例

[inline-code-attrs-start title = 'createV2PageReact の例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "fastcomments-tenant-72";
const urlId: string = "articles/2026/06/15/product-update";
const id: string = "page-8f3b2a";
const title: string = "Product Update: June 15, 2026";

(async function run(): Promise<void> {
  const response: CreateV2PageReact200Response = await createV2PageReact(tenantId, urlId, id, title);
  console.log(response);
})();
[inline-code-end]

---
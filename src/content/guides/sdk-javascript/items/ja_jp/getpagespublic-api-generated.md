テナントのページ一覧を返します。FChat デスクトップクライアントがルームリストを表示するために使用します。
各ページの解決済みカスタム設定で `enableFChat` が true である必要があります。
SSO を必要とするページは、要求ユーザーのグループアクセスに基づいてフィルタリングされます。

## Parameters

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenantId | string | はい |  |
| cursor | string | いいえ |  |
| limit | number | いいえ |  |
| q | string | いいえ |  |
| sortBy | PagesSortBy | いいえ |  |
| hasComments | boolean | いいえ |  |

## Response

戻り値: [`GetPagesPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetPagesPublic200Response.ts)

## Example

[inline-code-attrs-start title = 'getPagesPublic の例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_8f3b2c';
const cursor: string = 'eyJwYWdlIjoiMTIwIn0';
const limit: number = 25;
const q: string = 'homepage hero';
const hasComments: boolean = true;

const response: GetPagesPublic200Response = await getPagesPublic(
  tenantId,
  cursor,
  limit,
  q,
  undefined,
  hasComments
);
[inline-code-end]

---
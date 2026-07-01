## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenantId | string | はい |  |
| postIds | Array<string> | いいえ |  |
| sso | string | いいえ |  |

## 応答

返却: [`GetUserReactsPublicResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetUserReactsPublicResponse.ts)

## 例

[inline-code-attrs-start title = 'getUserReactsPublic の例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function demo() {
  const tenantId: string = "tenant_12345";
  const postIds: string[] = ["post_1a2b3c", "post_4d5e6f"];
  const ssoToken: string = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...";

  const fullResponse: GetUserReactsPublicResponse = await getUserReactsPublic(tenantId, postIds, ssoToken);
  const minimalResponse: GetUserReactsPublicResponse = await getUserReactsPublic(tenantId);
}

demo();
[inline-code-end]
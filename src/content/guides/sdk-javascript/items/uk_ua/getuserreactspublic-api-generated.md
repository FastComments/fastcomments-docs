## Параметри

| Назва | Тип | Обов’язково | Опис |
|------|------|----------|-------------|
| tenantId | string | Так |  |
| postIds | Array<string> | Ні |  |
| sso | string | Ні |  |

## Відповідь

Повертає: [`GetUserReactsPublicResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetUserReactsPublicResponse.ts)

## Приклад

[inline-code-attrs-start title = 'getUserReactsPublic Приклад'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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
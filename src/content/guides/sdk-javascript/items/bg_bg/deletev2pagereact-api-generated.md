## Параметри

| Име | Тип | Задължително | Описание |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| urlId | string | Да |  |
| id | string | Да |  |
| sso | string | Не |  |

## Отговор

Връща: [`CreateV1PageReact`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateV1PageReact.ts)

## Пример

[inline-code-attrs-start title = 'deleteV2PageReact Пример'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function runDeleteExamples() {
  const tenantId: string = "tenant_12345";
  const urlId: string = "page_98765";
  const commentId: string = "comment_abcde";

  // Извикване без незадължителен sso
  const resultWithoutSso: CreateV1PageReact = await deleteV2PageReact(tenantId, urlId, commentId);

  // Извикване с незадължителен sso
  const ssoToken: string = "sso_token_xyz";
  const resultWithSso: CreateV1PageReact = await deleteV2PageReact(tenantId, urlId, commentId, ssoToken);
}
[inline-code-end]
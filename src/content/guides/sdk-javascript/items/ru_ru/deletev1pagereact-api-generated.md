## Параметры

| Имя | Тип | Обязательно | Описание |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| urlId | string | Да |  |
| sso | string | Нет |  |

## Ответ

Возвращает: [`CreateV1PageReact`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateV1PageReact.ts)

## Пример

[inline-code-attrs-start title = 'Пример deleteV1PageReact'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function runExamples() {
  const tenantId: string = "tenant_9f8b7c6d";
  const urlId: string = "blog/post-2024-06-15";
  const ssoToken: string = "sso_user_42";

  const resultWithSso: CreateV1PageReact = await deleteV1PageReact(tenantId, urlId, ssoToken);
  const resultWithoutSso: CreateV1PageReact = await deleteV1PageReact(tenantId, urlId);
}
[inline-code-end]
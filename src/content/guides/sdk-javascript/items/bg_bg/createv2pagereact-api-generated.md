## Параметри

| Име | Тип | Задължителен | Описание |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| urlId | string | Да |  |
| id | string | Да |  |
| title | string | Не |  |

## Отговор

Връща: [`CreateV2PageReactResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateV2PageReactResponse.ts)

## Пример

[inline-code-attrs-start title = 'Пример за createV2PageReact'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = "acme-corp";
  const urlId: string = "blog/posts/fastcomments-integration";
  const pageId: string = "page-12345";
  const title: string = "FastComments API Integration Guide";

  const responseWithoutTitle: CreateV2PageReactResponse = await createV2PageReact(tenantId, urlId, pageId);
  const responseWithTitle: CreateV2PageReactResponse = await createV2PageReact(tenantId, urlId, pageId, title);

  console.log(responseWithoutTitle, responseWithTitle);
})();
[inline-code-end]
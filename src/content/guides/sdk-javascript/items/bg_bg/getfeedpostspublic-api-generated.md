req
tenantId
afterId

## Параметри

| Име | Тип | Задължително | Описание |
|------|------|--------------|----------|
| tenantId | string | Да |  |
| afterId | string | Не |  |
| limit | number | Не |  |
| tags | Array<string> | Не |  |
| sso | string | Не |  |
| isCrawler | boolean | Не |  |
| includeUserInfo | boolean | Не |  |

## Отговор

Връща: [`GetFeedPostsPublicResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetFeedPostsPublicResponse.ts)

## Пример

[inline-code-attrs-start title = 'Пример за getFeedPostsPublic'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function example() {
  const tenantId: string = "tenant_12345";
  const afterId: string = "post_9876";
  const limit: number = 20;
  const tags: string[] = ["news", "tech"];
  const sso: string = "userToken123";
  const isCrawler: boolean = false;
  const includeUserInfo: boolean = true;
  const response: GetFeedPostsPublicResponse = await getFeedPostsPublic(
    tenantId,
    afterId,
    limit,
    tags,
    sso,
    isCrawler,
    includeUserInfo
  );
}
example();
[inline-code-end]
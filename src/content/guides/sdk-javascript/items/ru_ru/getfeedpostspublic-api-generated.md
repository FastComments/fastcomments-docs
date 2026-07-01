запрос  
tenantId  
afterId  

## Параметры  

| Имя | Тип | Обязательно | Описание |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| afterId | string | Нет |  |
| limit | number | Нет |  |
| tags | Array<string> | Нет |  |
| sso | string | Нет |  |
| isCrawler | boolean | Нет |  |
| includeUserInfo | boolean | Нет |  |

## Ответ  

Возвращает: [`GetFeedPostsPublicResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetFeedPostsPublicResponse.ts)

## Пример  

[inline-code-attrs-start title = 'Пример getFeedPostsPublic'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]  
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
[api-resource-header-start name = 'Comment'; route = 'GET /api/v1/comments'; creditsCost = 1; api-resource-header-end]

Этот API используется для получения комментариев для отображения пользователю. Например, он автоматически фильтрует неподтверждённые или спам-комментарии.

### Pagination

Пагинация может выполняться одним из двух способов, в зависимости от требований к производительности и сценария использования:

1. Fastest: **Precalculated Pagination**:
   1. Так работает FastComments, когда вы используете наши предустановленные виджеты и клиенты.
   2. Нажатие "next" просто увеличивает номер страницы.
   3. Вы можете думать об этом как о получении из key-value хранилища.
   4. Таким образом, просто определите параметр `page`, начиная с `0`, и направление сортировки в `direction`.
   5. Размеры страниц можно настраивать через правила кастомизации.
2. Most Flexible: **Flexible Pagination**:
   1. В этом варианте вы можете определить собственные параметры `limit` и `skip`. Не передавайте `page`.
   2. Также поддерживается направление сортировки `direction`.
   3. `limit` — это общее количество возвращаемых элементов после применения `skip`.
      - Пример: установите `skip = 200, limit = 100`, когда `page size = 100` и `page = 2`.
   4. Дочерние комментарии по-прежнему учитываются в пагинации. Это можно обойти, используя опцию `asTree`.
      - Вы можете постранично получать дочерние через `limitChildren` и `skipChildren`.
      - Вы можете ограничить глубину возвращаемых тредов через `maxTreeDepth`.

### Threads

1. При использовании `Precalculated Pagination` комментарии группируются по *страницам*, и комментарии в тредах влияют на общую страницу.
   1. Таким образом, треды могут определяться на клиенте по `parentId`.
   2. Например, на странице с одним комментариев верхнего уровня и 29 ответами, при установке `page=0` в API — вы получите только верхний комментарий и 29 дочерних.
   3. [Example image here illustrating multiple pages.](https://blog.winricklabs.com/images/fc-pagination02.png)
2. При использовании `Flexible Pagination` вы можете задать параметр `parentId`.
   1. Установите его в null, чтобы получить только комментарии верхнего уровня.
   2. Затем, чтобы просмотреть треды, вызовите API снова и передайте `parentId`.
   3. Распространённое решение — вызвать API для получения комментариев верхнего уровня, а затем параллельно сделать вызовы API для получения комментариев для детей каждого комментария.
3. __NEW As of Feb 2023!__ Fetch as a tree using `&asTree=true`.
   1. Вы можете рассматривать это как `Flexible Pagination as a Tree`.
   2. В пагинацию учитываются только комментарии верхнего уровня.
   3. Установите `parentId=null`, чтобы начать дерево с корня (вы должны задать `parentId`).
   4. Установите `skip` и `limit` для пагинации.
   5. Установите `asTree` в `true`.
   6. Стоимость в кредитах увеличивается в `2x`, так как наш бэкенд должен выполнить гораздо больше работы в этом сценарии.
   7. Задайте `maxTreeDepth`, `limitChildren` и `skipChildren` по необходимости.

### Trees Explained

При использовании `asTree` иногда сложно понять, как работает пагинация. Вот наглядная схема:

<div class="screenshot white-bg">
    <div class="title">Tree Pagination Diagram</div>
    <img class="screenshot-image" src="/images/fastcomments-comments-api-tree.png" alt="Tree Pagination Diagram" />
</div>

### Fetching Comments in The Context of a User

API `/comments` может использоваться в двух контекстах, для разных сценариев использования:

- Для возврата комментариев с сортировкой и метаданными для построения собственного клиента.
  - В этом случае определите параметр запроса `contextUserId`.
- Для получения комментариев с вашего бэкенда для кастомных интеграций.
  - Платформа по умолчанию будет использовать этот сценарий без `contextUserId`. 

[inline-code-attrs-start title = 'Предварительно рассчитанная пагинация комментариев'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&page=0&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR'
[inline-code-end]

[inline-code-attrs-start title = 'Гибкая пагинация комментариев'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10'
[inline-code-end]

[inline-code-attrs-start title = 'Гибкая пагинация комментариев в контексте пользователя'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10&contextUserId=my-user-id'
[inline-code-end]

[inline-code-attrs-start title = 'Гибкая пагинация комментариев в контексте пользователя только для верхнего уровня'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10&contextUserId=my-user-id&parentId=null'
[inline-code-end]

### Get Comments as a Tree

Возможно получить комментарии в виде дерева, при этом в пагинацию учитываются только комментарии верхнего уровня.

[inline-code-attrs-start title = 'Комментарии как дерево в контексте пользователя'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10&contextUserId=my-user-id&parentId=null&asTree=true'
[inline-code-end]

Хотите получить только верхнеуровневые комментарии и их непосредственных детей? Вот один из вариантов:

[inline-code-attrs-start title = 'Комментарии как дерево с максимальной глубиной'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10&contextUserId=my-user-id&parentId=null&asTree=true&maxTreeDepth=1&limitChildren=10'
[inline-code-end]

Однако в вашем UI может понадобиться знать, показывать ли кнопку "показать ответы" для каждого комментария. При получении комментариев в виде дерева к комментариям добавляется свойство `hasChildren`, когда это применимо.

### Get Comments as a Tree, Searching by Hash Tag

Можно выполнять поиск по хештегу через API по всему вашему тенанту (не ограничиваясь одной страницей или `urlId`).

В этом примере мы опускаем `urlId` и ищем по нескольким хештегам. API вернёт только комментарии, которые содержат все запрошенные хештеги.

[inline-code-attrs-start title = 'Комментарии как дерево в контексте пользователя, по хештегу'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10&contextUserId=my-user-id&parentId=null&asTree=true&hashTag=TestTag&hashTag=OtherTestTag'
[inline-code-end]

### All Request Params

[inline-code-attrs-start title = 'Структура запроса комментариев'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface CommentsRequestQueryParams {
    tenantId: string
    API_KEY: string
    /** Идентификатор urlId (URL страницы или ID статьи), с которым связаны комментарии. **/
    urlId?: string
    /** Ограничить возвращаемые комментарии этим пользователем. **/
    userId?: string
    /** Используйте для поиска по хештегу. Чтобы получить пересечение нескольких хештегов, используйте &hashTag=a&hashTag=b. **/
    hashTag?: string
    /** Направление сортировки. По умолчанию MR (Most Relevant). Другие опции: OF (Oldest First) и NF (Newest First). **/
    direction?: 'MR' | 'OF' | 'NF'
    /** Precalculated Pagination: The page to fetch, starting with 0. Pass -1 for all comments (up to 250). **/
    page?: number
    /** Flexible Pagination: How many comments should we return? **/
    limit?: number
    /** Flexible Pagination: How many child comments should we return for each parent? **/
    limitChildren?: number
    /** Flexible Pagination: How many comments should we skip? **/
    skip?: number
    /** Flexible Pagination: How many child comments should we skip for each parent? **/
    skipChildren?: number
    /** For determining blocked and flagged comments. **/
    contextUserId?: string
    /** For determining blocked and flagged comments. **/
    anonUserId?: string
    /** For fetching child comments. **/
    parentId?: string
    /** For fetching as a tree. **/
    asTree?: boolean
    /** How far into the tree should we return data? 0 returns no children. 1 returns immediate children, etc. **/
    maxTreeDepth?: number
}
[inline-code-end]

### The Response

[inline-code-attrs-start title = 'Структура ответа комментариев'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface CommentsResponse {
    status: 'success' | 'failed'
    /** Включается в случае ошибки. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-url-id' | 'missing-date' | 'unauthorized-page' | 'invalid-pagination-request' | 'invalid-limit' | 'invalid-limit-children' | 'invalid-skip' | 'invalid-skip-children' | 'invalid-max-tree-depth'
    /** Включается в случае ошибки. **/
    reason?: string
    /** Комментарии! **/
    comments: Comment[]
}
[inline-code-end]

### Helpful Tips

#### URL ID

Вероятно, вам стоит использовать API `Comment` с параметром `urlId`. Вы можете сначала вызвать API `Pages`, чтобы увидеть, как выглядят доступные значения `urlId`. 

#### Anonymous Actions

Для анонимного комментирования, вероятно, стоит передавать `anonUserId` при получении комментариев, а также при выполнении пометки и блокировки.

(!) Это требуется во многих магазинах приложений, так как пользователи должны иметь возможность помечать пользовательский контент, который они видят, даже если они не вошли в систему. Непредоставление этой возможности может привести к удалению вашего приложения из такого магазина.

#### Comments Not Being Returned

Проверьте, что ваши комментарии одобрены и не являются спамом.
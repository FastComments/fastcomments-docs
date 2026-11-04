[api-resource-header-start name = 'Comment'; route = 'GET /api/v1/comments'; creditsCost = 1; api-resource-header-end]

Этот API используется для получения комментариев для отображения пользователю. Например, он автоматически отфильтровывает
неодобренные и спам-комментарии.

### Пагинация

Пагинация может выполняться одним из двух способов, в зависимости от требований к производительности и сценария использования:

1. Самый быстрый: **Предвычисленная пагинация (Precalculated Pagination)**:
   1. Так работает FastComments, когда вы используете наши готовые виджеты и клиенты.
   2. Нажатие «далее» просто увеличивает номер страницы.
   3. Это можно представить как получение данных из хранилища «ключ-значение».
   4. В этом случае просто задайте параметр `page`, начиная с `0`, и направление сортировки как `direction`.
   5. Размеры страниц можно настраивать с помощью правил кастомизации.
2. Самый гибкий: **Гибкая пагинация (Flexible Pagination)**:
   1. В этом случае вы можете задать собственные параметры `limit` и `skip`. Не передавайте `page`.
   2. Направление сортировки `direction` также поддерживается.
   3. `limit` — это общее количество возвращаемых записей после применения `skip`.
      - Пример: задайте `skip = 200, limit = 100` при `page size = 100` и `page = 2`.
   4. Дочерние комментарии также учитываются в пагинации. Обойти это можно с помощью опции `asTree`.
      - Дочерние комментарии можно пагинировать через `limitChildren` и `skipChildren`.
      - Глубину возвращаемых веток можно ограничить через `maxTreeDepth`.

### Ветки (Threads)

1. При использовании `Precalculated Pagination` комментарии группируются по *страницам*, и комментарии в ветках влияют на страницу в целом.
   1. В этом случае ветки могут определяться на клиенте на основе `parentId`.
   2. Например, для страницы с одним комментарием верхнего уровня и 29 ответами при установке `page=0` в API — вы получите только комментарий верхнего уровня и 29 дочерних комментариев.
2. При использовании `Flexible Pagination` вы можете задать параметр `parentId`.
   1. Установите его в null, чтобы получить только комментарии верхнего уровня.
   2. Затем для просмотра веток снова вызовите API, передав `parentId`.
   3. Распространённое решение — сделать вызов API для комментариев верхнего уровня, а затем параллельные вызовы API для получения комментариев дочерних элементов каждого комментария.
3. __НОВОЕ с февраля 2023!__ Получение в виде дерева с помощью `&asTree=true`.
   1. Это можно представить как `Flexible Pagination as a Tree`.
   2. В пагинации учитываются только комментарии верхнего уровня.
   3. Установите `parentId=null`, чтобы начать дерево с корня (вы должны установить `parentId`).
   4. Установите `skip` и `limit` для пагинации.
   5. Установите `asTree` в `true`.
   6. Стоимость в кредитах увеличивается в `2x` раза, так как наш бэкенд выполняет гораздо больше работы в этом сценарии.
   7. Установите `maxTreeDepth`, `limitChildren` и `skipChildren` по мере необходимости.

### Деревья: пояснение

При использовании `asTree` пагинацию бывает сложно представить. Вот наглядная схема:

<div class="screenshot white-bg">
    <div class="title">Диаграмма пагинации в виде дерева</div>
    <img class="screenshot-image" src="/images/fastcomments-comments-api-tree.png" alt="Диаграмма пагинации в виде дерева" />
</div>

### Получение комментариев в контексте пользователя

API `/comments` может использоваться в двух контекстах, для разных сценариев:

- Для возврата комментариев, отсортированных и снабжённых информацией для создания собственного клиента.
  - В этом случае задайте параметр запроса `contextUserId`.
- Для получения комментариев из вашего бэкенда для кастомных интеграций.
  - Платформа по умолчанию использует этот режим без `contextUserId`.

[inline-code-attrs-start title = 'Предвычисленная пагинация комментариев'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
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

[inline-code-attrs-start title = 'Гибкая пагинация комментариев в контексте пользователя, только комментарии верхнего уровня'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10&contextUserId=my-user-id&parentId=null'
[inline-code-end]

### Получение комментариев в виде дерева

Можно получить комментарии в виде дерева, где в пагинации учитываются только комментарии верхнего уровня.

[inline-code-attrs-start title = 'Комментарии в виде дерева в контексте пользователя'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10&contextUserId=my-user-id&parentId=null&asTree=true'
[inline-code-end]

Хотите получить только комментарии верхнего уровня и непосредственные ответы? Вот один из способов:

[inline-code-attrs-start title = 'Комментарии в виде дерева с максимальной глубиной'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10&contextUserId=my-user-id&parentId=null&asTree=true&maxTreeDepth=1&limitChildren=10'
[inline-code-end]

Однако в вашем интерфейсе может потребоваться знать, показывать ли кнопку «показать ответы»
на каждом комментарии. При получении комментариев в виде дерева к комментариям, где это применимо, добавляется свойство `hasChildren`.

### Получение комментариев в виде дерева с поиском по хэштегу

С помощью API можно выполнять поиск по хэштегу во всём вашем тенанте (не ограничиваясь одной страницей или `urlId`).

В этом примере мы опускаем `urlId` и выполняем поиск по нескольким хэштегам. API вернёт только те комментарии, которые содержат все запрошенные хэштеги.

[inline-code-attrs-start title = 'Комментарии в виде дерева в контексте пользователя, по хэштегу'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10&contextUserId=my-user-id&parentId=null&asTree=true&hashTag=TestTag&hashTag=OtherTestTag'
[inline-code-end]

### Все параметры запроса

[inline-code-attrs-start title = 'Структура запроса комментариев'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface CommentsRequestQueryParams {
    tenantId: string
    API_KEY: string
    /** The urlId (page url, or article id) the comments are associated with. **/
    urlId?: string
    /** Limit the comments returned by this user. **/
    userId?: string
    /** Use this to search by hashtag. To drill down to the intersection of multiple hashtags, do &hashTag=a&hashTag=b. **/
    hashTag?: string
    /** The sort direction. Default is MR (Most Relevant). Other options are OF (Oldest First) and NF (Newest First). **/
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

### Ответ

[inline-code-attrs-start title = 'Структура ответа комментариев'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface CommentsResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-url-id' | 'missing-date' | 'unauthorized-page' | 'invalid-pagination-request' | 'invalid-limit' | 'invalid-limit-children' | 'invalid-skip' | 'invalid-skip-children' | 'invalid-max-tree-depth'
    /** Included on failure. **/
    reason?: string
    /** The comments! **/
    comments: Comment[]
}
[inline-code-end]

### Полезные советы

#### URL ID

Вероятно, вы захотите использовать API `Comment` с параметром `urlId`. Вы можете сначала вызвать API `Pages`, чтобы посмотреть, как выглядят доступные вам значения `urlId`.

#### Анонимные действия

Для анонимного комментирования вам, вероятно, потребуется передавать `anonUserId` при получении комментариев, а также при отправке жалоб и блокировке.

(!) Это требуется во многих магазинах приложений, так как пользователи должны иметь возможность отправлять жалобы на видимый им пользовательский контент, даже если они не авторизованы. Несоблюдение этого требования может привести к удалению вашего приложения из соответствующего магазина.

#### Комментарии не возвращаются

Проверьте, что ваши комментарии одобрены и не помечены как спам.
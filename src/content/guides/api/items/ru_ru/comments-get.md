[api-resource-header-start name = 'Comment'; route = 'GET /api/v1/comments'; creditsCost = 1; api-resource-header-end]

Этот API используется для получения комментариев для отображения пользователю. Например, он автоматически фильтрует
неодобренные или спам-комментарии.

### Пагинация

Пагинация может выполняться одним из двух способов, в зависимости от требований к производительности и сценария использования:

1. Самый быстрый: **Precalculated Pagination**:
   1. Так работает FastComments, когда вы используете наши преднастроенные виджеты и клиенты.
   2. Нажатие "next" просто увеличивает номер страницы.
   3. Можно считать, что данные извлекаются из key-value хранилища.
   4. В этом случае просто определите параметр `page`, начиная с `0`, и направление сортировки через `direction`.
   5. Размеры страниц можно настраивать через правила кастомизации.
2. Самый гибкий: **Flexible Pagination**:
   1. В этом случае вы можете задать пользовательские параметры `limit` и `skip`. Не передавайте `page`.
   2. Поддерживается также направление сортировки `direction`.
   3. `limit` — это общее количество возвращаемых элементов после применения `skip`.
      - Пример: установите `skip = 200, limit = 100`, когда `page size = 100` и `page = 2`.
   4. Дочерние комментарии всё ещё учитываются в пагинации. Вы можете обойти это, используя опцию `asTree`.
      - Вы можете пагинировать дочерние комментарии через `limitChildren` и `skipChildren`.
      - Вы можете ограничить глубину возвращаемых веток через `maxTreeDepth`.

### Треды

1. При использовании `Precalculated Pagination` комментарии группируются по *странице*, и комментарии в тредах влияют на общую страницу.
   1. Таким образом, треды могут быть определены на клиенте на основе `parentId`.
   2. Например, на странице с одним комментарием верхнего уровня и 29 ответами, при установке `page=0` в API — вы получите только корневой комментарий и 29 детей.
   3. [Примерное изображение, иллюстрирующее несколько страниц.](https://blog.winricklabs.com/images/fc-pagination02.png)
2. При использовании `Flexible Pagination` вы можете задать параметр `parentId`.
   1. Установите его в null, чтобы получить только комментарии верхнего уровня.
   2. Затем, чтобы просмотреть треды, вызовите API снова и передайте `parentId`.
   3. Распространённое решение — сделать вызов API для комментариев верхнего уровня, а затем параллельно сделать вызовы API, чтобы получить комментарии для детей каждого комментария.
3. __NEW As of Feb 2023!__ Получайте в виде дерева, используя `&asTree=true`.
   1. Можно считать это `Flexible Pagination as a Tree`.
   2. В пагинации учитываются только комментарии верхнего уровня.
   3. Установите `parentId=null`, чтобы начать дерево с корня (вы должны задать `parentId`).
   4. Установите `skip` и `limit` для пагинации.
   5. Установите `asTree` в `true`.
   6. Стоимость в кредитах увеличивается в `2x`, так как наш бэкенд выполняет гораздо больше работы в этом сценарии.
   7. При необходимости задайте `maxTreeDepth`, `limitChildren` и `skipChildren`.

### Объяснение деревьев

При использовании `asTree` может быть сложно понять пагинацию. Вот полезная графика:

<div class="screenshot white-bg">
    <div class="title">Диаграмма пагинации дерева</div>
    <img class="screenshot-image" src="/images/fastcomments-comments-api-tree.png" alt="Диаграмма пагинации дерева" />
</div>

### Получение комментариев в контексте пользователя

API `/comments` можно использовать в двух контекстах, для разных сценариев использования:

- Для возврата комментариев, отсортированных и помеченных информацией для построения собственного клиента.
  - В этом случае определите параметр запроса `contextUserId`.
- Для получения комментариев с вашего бэкенда для кастомных интеграций.
  - Платформа по умолчанию будет работать в этом режиме без `contextUserId`. 

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

[inline-code-attrs-start title = 'Гибкая пагинация комментариев в контексте пользователя — только комментарии верхнего уровня'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10&contextUserId=my-user-id&parentId=null'
[inline-code-end]

### Получить комментарии как дерево

Возможно получить комментарии в виде дерева, при этом в пагинацию учитываются только комментарии верхнего уровня.

[inline-code-attrs-start title = 'Комментарии как дерево в контексте пользователя'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10&contextUserId=my-user-id&parentId=null&asTree=true'
[inline-code-end]

Хотите получить только комментарии верхнего уровня и их непосредственных детей? Вот один из способов:

[inline-code-attrs-start title = 'Комментарии как дерево с максимальной глубиной'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10&contextUserId=my-user-id&parentId=null&asTree=true&maxTreeDepth=1&limitChildren=10'
[inline-code-end]

Однако в вашем UI может потребоваться знать, показывать ли кнопку "показать ответы" для
каждого комментария. При получении комментариев в виде дерева к комментариям добавляется свойство `hasChildren`, когда это применимо.

### Получение комментариев как дерева, поиск по хештегу

Возможно выполнить поиск по хэштегу через API по всему вашему тенанту (не ограничиваясь одной страницей или `urlId`).

В этом примере мы опускаем `urlId` и выполняем поиск по нескольким хэштегам. API вернёт только те комментарии, которые содержат все запрошенные хэштеги.

[inline-code-attrs-start title = 'Комментарии как дерево в контексте пользователя, по хештегу'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
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

Вам, вероятно, захочется использовать API `Comment` с параметром `urlId`. Вы можете сначала вызвать API `Pages`, чтобы увидеть, как выглядят доступные для вас значения `urlId`. 

#### Анонимные действия

Для анонимного комментирования вам, вероятно, нужно передавать `anonUserId` при получении комментариев, а также при выполнении действий по пометке и блокировке.

(!) Это требуется для многих магазинов приложений, так как пользователи должны иметь возможность пометить контент, созданный пользователями, который они видят, даже если они не вошли в систему. Невыполнение этого требования может привести к удалению вашего приложения из указанного магазина.

#### Комментарии не возвращаются

Проверьте, что ваши комментарии одобрены и не являются спамом.

---
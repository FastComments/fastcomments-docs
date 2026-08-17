[api-resource-header-start name = 'Comment'; route = 'GET /api/v1/comments'; creditsCost = 1; api-resource-header-end]

Этот API используется для получения комментариев для отображения пользователю. Например, он автоматически фильтрует неутверждённые или спам‑комментарии.

### Pagination

Пагинацию можно выполнить одним из двух способов, в зависимости от требований к производительности и сценария использования:

1. **Самый быстрый:** **Precalculated Pagination**:
   1. Так работает FastComments, когда вы используете наши готовые виджеты и клиенты.
   2. Нажатие «next» просто увеличивает номер страницы.
   3. Можно представить это как получение из хранилища ключ‑значение.
   4. Таким образом, просто задайте параметр `page`, начиная с `0`, и направление сортировки как `direction`.
   5. Размеры страниц можно настраивать с помощью правил кастомизации.
2. **Самый гибкий:** **Flexible Pagination**:
   1. Таким образом вы можете задать пользовательские параметры `limit` и `skip`. Не передавайте `page`.
   2. Сортировка `direction` также поддерживается.
   3. `limit` — общее количество элементов, которое следует вернуть после применения `skip`.
      - Пример: установите `skip = 200, limit = 100`, когда `page size = 100` и `page = 2`.
   4. Дочерние комментарии всё ещё учитываются в пагинации. Можно обойти это, используя опцию `asTree`.
      - Можно пагинировать дочерние элементы с помощью `limitChildren` и `skipChildren`.
      - Можно ограничить глубину возвращаемых веток с помощью `maxTreeDepth`.

### Threads

1. При использовании `Precalculated Pagination` комментарии группируются по *странице*, и комментарии в ветках влияют на общую страницу.
   1. Таким образом, ветки можно определить на клиенте, основываясь на `parentId`.
   2. Например, на странице с одним корневым комментарием и 29 ответами, при установке `page=0` в API вы получите только корневый комментарий и 29 дочерних.
2. При использовании `Flexible Pagination` вы можете задать параметр `parentId`.
   1. Установите его в null, чтобы получать только корневые комментарии.
   2. Затем, чтобы просмотреть ветки, вызовите API снова и передайте `parentId`.
   3. Распространённое решение — выполнить запрос API для корневых комментариев, а затем параллельно запросить комментарии для дочерних элементов каждого комментария.
3. __NEW С февраля 2023!__ Получайте в виде дерева, используя `&asTree=true`.
   1. Можно рассматривать это как `Flexible Pagination as a Tree`.
   2. Только корневые комментарии учитываются в пагинации.
   3. Установите `parentId=null`, чтобы начать дерево с корня (необходимо задать `parentId`).
   4. Задайте `skip` и `limit` для пагинации.
   5. Установите `asTree` в `true`.
   6. Стоимость в кредитах увеличивается в `2x`, так как наш бекенд должен выполнить гораздо больше работы в этом сценарии.
   7. Установите `maxTreeDepth`, `limitChildren` и `skipChildren` по необходимости.

### Trees Explained

При использовании `asTree` может быть сложно понять пагинацию. Вот удобная графика:

<div class="screenshot white-bg">
    <div class="title">Диаграмма пагинации дерева</div>
    <img class="screenshot-image" src="/images/fastcomments-comments-api-tree.png" alt="Диаграмма пагинации дерева" />
</div>

### Fetching Comments in The Context of a User

API `/comments` может использоваться в двух контекстах для разных сценариев:

- Для возврата комментариев, отсортированных и помеченных информацией для построения собственного клиента.
  - В этом случае задайте параметр запроса `contextUserId`.
- Для получения комментариев из вашего бекенда для пользовательских интеграций.
  - Платформа будет использовать это по умолчанию без `contextUserId`.

[inline-code-attrs-start title = 'Комментарии с предрассчитанной пагинацией'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&page=0&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR'
[inline-code-end]

[inline-code-attrs-start title = 'Комментарии с гибкой пагинацией'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10'
[inline-code-end]

[inline-code-attrs-start title = 'Комментарии с гибкой пагинацией в контексте пользователя'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10&contextUserId=my-user-id'
[inline-code-end]

[inline-code-attrs-start title = 'Комментарии с гибкой пагинацией в контексте пользователя только для комментариев верхнего уровня'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10&contextUserId=my-user-id&parentId=null'
[inline-code-end]

### Get Comments as a Tree

Можно получить комментарии в виде дерева, при этом в пагинацию учитываются только корневые комментарии.

[inline-code-attrs-start title = 'Комментарии в виде дерева в контексте пользователя'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10&contextUserId=my-user-id&parentId=null&asTree=true'
[inline-code-end]

Хотите получить только корневые комментарии и их непосредственных потомков? Вот один из способов:

[inline-code-attrs-start title = 'Комментарии в виде дерева с максимальной глубиной'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10&contextUserId=my-user-id&parentId=null&asTree=true&maxTreeDepth=1&limitChildren=10'
[inline-code-end]

Однако в вашем UI может потребоваться знать, показывать ли кнопку «показать ответы» у каждого комментария. При получении комментариев в виде дерева к комментариям добавляется свойство `hasChildren`, если это применимо.

### Get Comments as a Tree, Searching by Hash Tag

Можно выполнять поиск по хештегу с помощью API по всему вашему арендатору (не ограничено одной страницей или `urlId`).

В этом примере мы опускаем `urlId` и ищем по нескольким хештегам. API вернёт только те комментарии, которые содержат все запрошенные хештеги.

[inline-code-attrs-start title = 'Комментарии в виде дерева в контексте пользователя, по хештегу'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
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
    /** urlId (URL страницы или идентификатор статьи), с которым связаны комментарии. **/
    urlId?: string
    /** Ограничить комментарии, возвращаемые этим пользователем. **/
    userId?: string
    /** Используйте это для поиска по хештегу. Чтобы сузить до пересечения нескольких хештегов, используйте &hashTag=a&hashTag=b. **/
    hashTag?: string
    /** Направление сортировки. По умолчанию MR (Most Relevant). Другие варианты: OF (Oldest First) и NF (Newest First). **/
    direction?: 'MR' | 'OF' | 'NF'
    /** Precalculated Pagination: Страница для получения, начиная с 0. Передайте -1 для всех комментариев (до 250). **/
    page?: number
    /** Flexible Pagination: Сколько комментариев следует вернуть? **/
    limit?: number
    /** Flexible Pagination: Сколько дочерних комментариев вернуть для каждого родителя? **/
    limitChildren?: number
    /** Flexible Pagination: Сколько комментариев следует пропустить? **/
    skip?: number
    /** Flexible Pagination: Сколько дочерних комментариев следует пропустить для каждого родителя? **/
    skipChildren?: number
    /** Для определения заблокированных и отмеченных комментариев. **/
    contextUserId?: string
    /** Для определения заблокированных и отмеченных комментариев. **/
    anonUserId?: string
    /** Для получения дочерних комментариев. **/
    parentId?: string
    /** Для получения в виде дерева. **/
    asTree?: boolean
    /** Насколько глубоко в дерево следует возвращать данные? 0 — без дочерних элементов. 1 — только непосредственные дочерние, и т.д. **/
    maxTreeDepth?: number
}
[inline-code-end]

### The Response

[inline-code-attrs-start title = 'Структура ответа комментариев'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface CommentsResponse {
    status: 'success' | 'failed'
    /** Присутствует при ошибке. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-url-id' | 'missing-date' | 'unauthorized-page' | 'invalid-pagination-request' | 'invalid-limit' | 'invalid-limit-children' | 'invalid-skip' | 'invalid-skip-children' | 'invalid-max-tree-depth'
    /** Присутствует при ошибке. **/
    reason?: string
    /** Комментарии! **/
    comments: Comment[]
}
[inline-code-end]

### Helpful Tips

#### URL ID

Вероятно, вам следует использовать API `Comment` с параметром `urlId`. Вы можете сначала вызвать API `Pages`, чтобы увидеть, какие значения `urlId` доступны.

#### Anonymous Actions

Для анонимных комментариев, вероятно, вам следует передавать `anonUserId` при получении комментариев, а также при пометке и блокировке.

(!) Это требуется во многих магазинах приложений, поскольку пользователи должны иметь возможность помечать пользовательский контент, который они видят, даже если они не вошли в систему. Отсутствие этой возможности может привести к удалению вашего приложения из соответствующего магазина.

#### Comments Not Being Returned

Убедитесь, что ваши комментарии одобрены и не являются спамом.

---
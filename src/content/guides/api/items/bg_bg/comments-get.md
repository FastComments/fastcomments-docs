[api-resource-header-start name = 'Comment'; route = 'GET /api/v1/comments'; creditsCost = 1; api-resource-header-end]

Този API се използва за получаване на коментари за показване на потребител. Например, той автоматично филтрира неодобрените или спам коментари.

### Пагинация

Пагинацията може да се извърши по един от двата начина, в зависимост от изискванията за производителност и случая на употреба:

1. Най-бързо: **Предварително изчислена пагинация**:
   1. Това е начинът, по който FastComments работи, когато използвате нашите предварително изградени уиджети и клиенти.
   2. Кликването върху „next“ просто увеличава броя на страниците.
   3. Можете да го разглеждате като извличано от хранилище с ключ-стойност.
   4. По този начин, просто задайте параметъра `page`, започвайки от `0`, и посока на сортиране като `direction`.
   5. Размерите на страниците могат да се персонализират чрез правила за персонализация.
2. Най-гъвкаво: **Гъвкава пагинация**:
   1. По този начин можете да зададете персонализирани параметри `limit` и `skip`. Не предавайте `page`.
   2. Сортирането `direction` също се поддържа.
   3. `limit` е общият брой, който да се върне след прилагане на `skip`.
      - Пример: задайте `skip = 200, limit = 100`, когато `page size = 100` и `page = 2`.
   4. Дъщерните коментари все още се броят в пагинацията. Можете да заобиколите това, като използвате опцията `asTree`.
      - Можете да пагинирате дъщерните чрез `limitChildren` и `skipChildren`.
      - Можете да ограничите дълбочината на върнатите нишки чрез `maxTreeDepth`.

### Нишки

1. При използване на `Precalculated Pagination`, коментарите се групират по *страница* и коментарите в нишките влияят върху цялата страница.
   1. По този начин нишките могат да се определят от клиента въз основа на `parentId`.
   2. Например, при страница с един коментар от най-горно ниво и 29 отговора, и задаване на `page=0` в API - ще получите само коментара от най-горно ниво и 29-те дъщерни.
2. При използване на `Flexible Pagination`, можете да зададете параметър `parentId`.
   1. Задайте го на null, за да получите само коментари от най-горно ниво.
   2. След това, за да видите нишките, извикайте отново API и предайте `parentId`.
   3. Често решение е да направите API заявка за коментарите от най-горно ниво и след това паралелни API заявки, за да получите коментари за дъщерните на всеки коментар.
3. __НОВО От февруари 2023 г.!__ Извличане като дърво, използвайки `&asTree=true`.
   1. Можете да го разглеждате като `Гъвкава пагинация като дърво`.
   2. Само коментарите от най-горно ниво се броят в пагинацията.
   3. Задайте `parentId=null`, за да започнете дървото от корена (трябва да зададете `parentId`).
   4. Задайте `skip` и `limit` за пагинация.
   5. Задайте `asTree` на `true`.
   6. Цената в кредити се увеличава с `2x`, тъй като нашият бекенд трябва да извърши много повече работа в този сценарий.
   7. Задайте `maxTreeDepth`, `limitChildren` и `skipChildren` според желанието.

### Обяснение на дърветата

При използване на `asTree`, може да е трудно да се разбере пагинацията. Ето удобна графика:

<div class="screenshot white-bg">
    <div class="title">Диаграма на пагинацията на дървото</div>
    <img class="screenshot-image" src="/images/fastcomments-comments-api-tree.png" alt="Диаграма на пагинацията на дървото" />
</div>

### Извличане на коментари в контекста на потребител

API‑т `/comments` може да се използва в два контекста, за различни случаи на употреба:

- За връщане на коментари, сортирани и маркирани с информация за изграждане на ваш собствен клиент.
  - В този случай, задайте параметъра за заявка `contextUserId`.
- За извличане на коментари от вашия бекенд за персонализирани интеграции.
  - Платформата ще използва това по подразбиране без `contextUserId`.

[inline-code-attrs-start title = 'Коментари с предварително изчислена пагинация'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&page=0&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR'
[inline-code-end]

[inline-code-attrs-start title = 'Коментари с гъвкава пагинация'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10'
[inline-code-end]

[inline-code-attrs-start title = 'Коментари с гъвкава пагинация в потребителски контекст'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10&contextUserId=my-user-id'
[inline-code-end]

[inline-code-attrs-start title = 'Коментари с гъвкава пагинация в потребителски контекст само за коментари от най-горно ниво'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10&contextUserId=my-user-id&parentId=null'
[inline-code-end]

### Получаване на коментари като дърво

Възможно е да получите коментарите върнати като дърво, като пагинацията брои само коментарите от най-горно ниво.

[inline-code-attrs-start title = 'Коментари като дърво в потребителски контекст'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10&contextUserId=my-user-id&parentId=null&asTree=true'
[inline-code-end]

Искате ли да получите само коментарите от най-горно ниво и непосредствените им дъщерни? Ето един начин:

[inline-code-attrs-start title = 'Коментари като дърво с максимална дълбочина'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10&contextUserId=my-user-id&parentId=null&asTree=true&maxTreeDepth=1&limitChildren=10'
[inline-code-end]

Въпреки това, във вашия UI може да се наложи да знаете дали да покажете бутон „показване на отговори“ за всеки коментар. При извличане на коментари чрез дърво има свойство `hasChildren`, маркирано върху коментарите, когато е приложимо.

### Получаване на коментари като дърво, търсене по хаштаг

Възможно е да търсите по хаштаг чрез API, в целия ви наемател (не е ограничено до една страница или `urlId`).

В този пример пропускаме `urlId` и търсим по множество хаштагове. API‑т ще върне само коментари, които имат всички заявени хаштагове.

[inline-code-attrs-start title = 'Коментари като дърво в потребителски контекст, по хаштаг'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10&contextUserId=my-user-id&parentId=null&asTree=true&hashTag=TestTag&hashTag=OtherTestTag'
[inline-code-end]

### Всички параметри на заявката

[inline-code-attrs-start title = 'Структура на заявка за коментари'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

### Отговорът

[inline-code-attrs-start title = 'Структура на отговор за коментари'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

### Полезни съвети

#### URL ID

Вероятно искате да използвате API‑то `Comment` с параметъра `urlId`. Можете първо да извикате API‑то `Pages`, за да видите как изглеждат наличните за вас стойности на `urlId`.

#### Анонимни действия

За анонимно коментиране вероятно искате да предадете `anonUserId`, когато извличате коментари, и при извършване на маркиране и блокиране.

(!) Това е задължително за много магазини за приложения, тъй като потребителите трябва да могат да маркират съдържание, създадено от потребители, което виждат, дори ако не са влезли. Ако не го направите, вашето приложение може да бъде премахнато от съответния магазин.

#### Коментари, които не се връщат

Проверете дали вашите коментари са одобрени и не са спам.

---
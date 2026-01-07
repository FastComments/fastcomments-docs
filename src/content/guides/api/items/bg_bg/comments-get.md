[api-resource-header-start name = 'Comment'; route = 'GET /api/v1/comments'; creditsCost = 1; api-resource-header-end]

Този API се използва за получаване на коментари за показване на потребител. Например, автоматично филтрира
неодобрени или спам коментари.

### Странициране

Страницирането може да се направи по един от два начина, в зависимост от изискванията за производителност и случая на употреба:

1. Най-бързо: **Предварително изчислено странициране**:
   1. Така работи FastComments, когато използвате нашите готови уиджети и клиенти.
   2. Щракването върху "следващ" просто увеличава броя на страниците.
   3. Можете да мислите за това като извличане чрез хранилище ключ-стойност.
   4. По този начин просто дефинирайте параметър `page`, започващ от `0`, и посока на сортиране като `direction`.
   5. Размерите на страниците могат да бъдат персонализирани чрез правила за персонализация.
2. Най-гъвкаво: **Гъвкаво странициране**:
   1. По този начин можете да дефинирате персонализирани параметри `limit` и `skip`. Не подавайте `page`.
   2. Посока на сортиране `direction` също се поддържа.
   3. `limit` е общият брой за връщане след прилагане на `skip`.
      - Пример: задайте `skip = 200, limit = 100`, когато `page size = 100` и `page = 2`.
   4. Дъщерните коментари все още се броят при страницирането. Можете да заобиколите това, като използвате опцията `asTree`.
      - Можете да странирате децата чрез `limitChildren` и `skipChildren`.
      - Можете да ограничите дълбочината на върнатите нишки чрез `maxTreeDepth`.

### Нишки

1. Когато използвате `Предварително изчислено странициране`, коментарите се групират по *страница* и коментарите в нишки влияят на общата страница.
   1. По този начин нишките могат да се определят на клиента въз основа на `parentId`.
   2. Например, със страница с един коментар от най-високо ниво и 29 отговора, и задаване на `page=0` в API - ще получите само коментара от най-високо ниво и 29-те деца.
   3. [Примерно изображение тук, илюстриращо множество страници.](https://blog.winricklabs.com/images/fc-pagination02.png)
2. Когато използвате `Гъвкаво странициране`, можете да дефинирате параметър `parentId`.
   1. Задайте това на null, за да получите само коментари от най-високо ниво.
   2. След това, за да видите нишки, извикайте API отново и подайте `parentId`.
   3. Често срещано решение е да направите API извикване за коментарите от най-високо ниво и след това да направите паралелни API извиквания, за да получите коментарите за децата на всеки коментар.
3. __НОВО от февруари 2023!__ Извличане като дърво с използване на `&asTree=true`.
   1. Можете да мислите за това като `Гъвкаво странициране като дърво`.
   2. Само коментарите от най-високо ниво се броят при страницирането.
   3. Задайте `parentId=null`, за да започнете дървото от корена (трябва да зададете `parentId`).
   4. Задайте `skip` и `limit` за странициране.
   5. Задайте `asTree` на `true`.
   6. Цената на кредитите се увеличава с `2x`, тъй като нашият бекенд трябва да свърши много повече работа в този сценарий.
   7. Задайте `maxTreeDepth`, `limitChildren` и `skipChildren` по желание.

### Обяснение на дърветата

Когато използвате `asTree`, може да е трудно да разсъждавате за страницирането. Ето полезна графика:

<div class="screenshot white-bg">
    <div class="title">Диаграма на странициране на дърво</div>
    <img class="screenshot-image" src="/images/fastcomments-comments-api-tree.png" alt="Диаграма на странициране на дърво" />
</div>

### Извличане на коментари в контекста на потребител

API `/comments` може да се използва в два контекста, за различни случаи на употреба:

- За връщане на коментари, сортирани и маркирани с информация за изграждане на ваш собствен клиент.
  - В този случай дефинирайте параметър на заявката `contextUserId`.
- За извличане на коментари от вашия бекенд за персонализирани интеграции.
  - Платформата ще използва това по подразбиране без `contextUserId`.

[inline-code-attrs-start title = 'Предварително изчислено странициране на коментари'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&page=0&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR'
[inline-code-end]

[inline-code-attrs-start title = 'Гъвкаво странициране на коментари'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10'
[inline-code-end]

[inline-code-attrs-start title = 'Гъвкаво странициране на коментари в контекст на потребител'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10&contextUserId=my-user-id'
[inline-code-end]

[inline-code-attrs-start title = 'Гъвкаво странициране на коментари в контекст на потребител само за коментари от най-високо ниво'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10&contextUserId=my-user-id&parentId=null'
[inline-code-end]

### Получаване на коментари като дърво

Възможно е коментарите да се върнат като дърво, като страницирането брои само коментарите от най-високо ниво.

[inline-code-attrs-start title = 'Коментари като дърво в контекст на потребител'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10&contextUserId=my-user-id&parentId=null&asTree=true'
[inline-code-end]

Искате да получите само коментарите от най-високо ниво и непосредствените деца? Ето един начин:

[inline-code-attrs-start title = 'Коментари като дърво с максимална дълбочина'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10&contextUserId=my-user-id&parentId=null&asTree=true&maxTreeDepth=1&limitChildren=10'
[inline-code-end]

Въпреки това, във вашия потребителски интерфейс може да се наложи да знаете дали да покажете бутон "покажи отговорите" на
всеки коментар. Когато извличате коментари чрез дърво, има свойство `hasChildren`, маркирано
към коментарите, когато е приложимо.

### Получаване на коментари като дърво, търсене по хаштаг

Възможно е да търсите по хаштаг с помощта на API, в целия ви tenant (не е ограничено до една страница или `urlId`).

В този пример пропускаме `urlId` и търсим по множество хаштагове. API ще върне само коментари, които имат всички заявени хаштагове.

[inline-code-attrs-start title = 'Коментари като дърво в контекст на потребител, по хаштаг'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10&contextUserId=my-user-id&parentId=null&asTree=true&hashTag=TestTag&hashTag=OtherTestTag'
[inline-code-end]

### Всички параметри на заявката

[inline-code-attrs-start title = 'Структура на заявката за коментари'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

[inline-code-attrs-start title = 'Структура на отговора за коментари'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

Вероятно искате да използвате API `Comment` с параметъра `urlId`. Можете първо да извикате API `Pages`, за да видите как изглеждат наличните за вас стойности на `urlId`.

#### Анонимни действия

За анонимно коментиране вероятно искате да подадете `anonUserId`, когато извличате коментари и когато извършвате докладване и блокиране.

(!) Това е необходимо за много магазини за приложения, тъй като потребителите трябва да могат да докладват съдържание, създадено от потребители, което могат да видят, дори ако не са влезли. Неизпълнението на това може да доведе до премахване на приложението ви от съответния магазин.

#### Коментарите не се връщат

Проверете дали вашите коментари са одобрени и не са спам.

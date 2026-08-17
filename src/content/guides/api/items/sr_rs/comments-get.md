[api-resource-header-start name = 'Comment'; route = 'GET /api/v1/comments'; creditsCost = 1; api-resource-header-end]

Овај API се користи за добијање коментара за приказ кориснику. На пример, аутоматски филтрира неодобрене или спам коментаре.

### Пагинација

Пагинација се може извршити на један од два начина, у зависности од захтева за перформансама и случаја употребе:

1. Најбрже: **Precalculated Pagination**:
   1. Ово је начин на који FastComments функционише када користите наше унапред изграђене виџете и клијенте.
   2. Кликом на „next“ једноставно се повећава број странице.
   3. Ово можете замислити као преузимање из кључ-врједност складишта.
   4. На овај начин, једноставно дефинишете параметар `page` који почиње од `0` и смер сортирања као `direction`.
   5. Величине страница се могу прилагодити преко правила прилагођавања.
2. Најфлексибилније: **Flexible Pagination**:
   1. На овај начин можете дефинисати прилагођене параметре `limit` и `skip`. Не прослеђујте `page`.
   2. Сортирање `direction` је такође подржано.
   3. `limit` је укупан број који се враћа након што се примени `skip`.
      - Пример: поставите `skip = 200, limit = 100` када је `page size = 100` и `page = 2`.
   4. Коментари-дете и даље улазе у пагинацију. Ово можете заобићи коришћењем опције `asTree`.
      - Можете пагинирати децу преко `limitChildren` и `skipChildren`.
      - Можете ограничити дубину веза које се враћају преко `maxTreeDepth`.

### Теме

1. Када се користи `Precalculated Pagination`, коментари се групишу по *страници* и коментари у темама утичу на целокупну страницу.
   1. На овај начин, теме се могу одредити на клијенту на основу `parentId`.
   2. На пример, са страницом која има један коментар највишег нивоа и 29 одговора, и постављањем `page=0` у API-ју - добићете само коментар највишег нивоа и 29 деце.
2. Када се користи `Flexible Pagination`, можете дефинисати параметар `parentId`.
   1. Поставите га на null да бисте добили само коментаре највишег нивоа.
   2. Затим, да бисте видели теме, позовите API поново и проследите `parentId`.
   3. Уобичајено решење је да направите API позив за коментаре највишег нивоа, а затим паралелне API позиве да добијете коментаре за децу сваког коментара.
3. __НОВО Од фебруара 2023!__ Дохватите као дрво користећи `&asTree=true`.
   1. Ово можете замислити као `Flexible Pagination as a Tree`.
   2. Само коментари највишег нивоа се рачунају у пагинацији.
   3. Поставите `parentId=null` да започнете дрво од корена (морате поставити `parentId`).
   4. Поставите `skip` и `limit` за пагинацију.
   5. Поставите `asTree` на `true`.
   6. Трошак кредита се повећава за `2x`, јер наш бекенд мора да уради много више посла у овом сценарију.
   7. Поставите `maxTreeDepth`, `limitChildren` и `skipChildren` по жељи.

### Објашњење Дрвених Структура

Када се користи `asTree`, може бити тешко размотрити пагинацију. Ево практичне графике:

<div class="screenshot white-bg">
    <div class="title">Дијаграм Пагинације Дрва</div>
    <img class="screenshot-image" src="/images/fastcomments-comments-api-tree.png" alt="Дијаграм Пагинације Дрва" />
</div>

### Дохватање Коментара у Контексту Корисника

API `/comments` може да се користи у два контекста, за различите случајеве употребе:

- За враћање коментара сортираних и означених информацијама за изградњу вашег сопственог клијента.
  - У овом случају, дефинишите параметар упита `contextUserId`.
- За дохватање коментара из вашег бекенда за прилагођене интеграције.
  - Платформа ће подразумевано користити ово без `contextUserId`. 

[inline-code-attrs-start title = 'Коментари Претходно Израђена Пагинација'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&page=0&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR'
[inline-code-end]

[inline-code-attrs-start title = 'Коментари Флексибилна Пагинација'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10'
[inline-code-end]

[inline-code-attrs-start title = 'Коментари Флексибилна Пагинација у Корисничком Контексту'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10&contextUserId=my-user-id'
[inline-code-end]

[inline-code-attrs-start title = 'Коментари Флексибилна Пагинација у Корисничком Контексту Само за Коментаре Највишег Нивоа'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10&contextUserId=my-user-id&parentId=null'
[inline-code-end]

### Дохватање Коментара као Дрво

Могуће је добити коментаре враћене као дрво, при чему пагинација броји само коментаре највишег нивоа.

[inline-code-attrs-start title = 'Коментари Као-Дрво у Корисничком Контексту'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10&contextUserId=my-user-id&parentId=null&asTree=true'
[inline-code-end]

Желите ли да добијете само коментаре највишег нивоа и непосредну децу? Ево једног начина:

[inline-code-attrs-start title = 'Коментари Као-Дрво са Максималном Дубином'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10&contextUserId=my-user-id&parentId=null&asTree=true&maxTreeDepth=1&limitChildren=10'
[inline-code-end]

Међутим, у вашем UI можда ћете морати знати да ли да прикажете дугме „прикажи одговоре“ на сваком коментару. При дохватању коментара преко дрвета постоји својство `hasChildren` означено на коментарима када је применљиво.

### Дохватање Коментара као Дрво, Претрага по Хеш Тагу

Могуће је претраживати по хеш тагу користећи API, преко целог вашег tenancy (не ограничено на једну страницу или `urlId`).

У овом примеру, изостављамо `urlId` и претражујемо по више хеш тагова. API ће вратити само коментаре који имају све захтеване хеш тагове.

[inline-code-attrs-start title = 'Коментари Као-Дрво у Корисничком Контексту, По Хеш Тагу'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10&contextUserId=my-user-id&parentId=null&asTree=true&hashTag=TestTag&hashTag=OtherTestTag'
[inline-code-end]

### Сви Параметри Захтева

[inline-code-attrs-start title = 'Структура Захтева за Коментаре'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

### Одговор

[inline-code-attrs-start title = 'Структура Одговора за Коментаре'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

### Корисни Савети

#### URL ID

Вероватно желите да користите API `Comment` са параметром `urlId`. Прво можете позвати API `Pages` да видите како изгледају доступне `urlId` вредности.

#### Анонимне Радње

За анонимно коментарисање вероватно желите да проследите `anonUserId` приликом дохватања коментара, као и приликом означавања и блокирања.

(!) Ово је потребно за многе продавнице апликација јер корисници морају моћи да означе садржај који су створили други корисници, чак и ако нису пријављени. Не испуњавање ове захтева може довести до уклањања ваше апликације из те продавнице.

#### Коментари Не Се Враћају

Проверите да ли су ваши коментари одобрени и да нису спам.

---
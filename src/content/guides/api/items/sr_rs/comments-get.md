[api-resource-header-start name = 'Comment'; route = 'GET /api/v1/comments'; creditsCost = 1; api-resource-header-end]

Овај API се користи за добијање коментара који ће бити приказани кориснику. На пример, аутоматски филтрира неприхваћене или спам коментаре.

### Пагинација

Пагинација се може извести на један од два начина, у зависности од захтева за перформансама и случаја употребе:

1. Најбрже: **Прерачуната пагинација**:
   1. Ово је начин на који FastComments ради када користите наше унапред направљене видгете и клијенте.
   2. Клик на "next" једноставно увећава број странице.
   3. Можете ово сматрати као да је дохваћено из key-value продавнице.
   4. На овај начин, једноставно дефинишете параметар `page` који почиње од `0` и смер сортирања као `direction`.
   5. Величине страница се могу прилагодити преко правила прилагођавања.
2. Најфлексибилније: **Флексибилна пагинација**:
   1. На овај начин можете дефинисати прилагођене параметре `limit` и `skip`. Не пролазите `page`.
   2. Смер сортирања `direction` је такође подржан.
   3. `limit` је укупни број који ће бити враћен након што се примени `skip`.
      - Пример: подесите `skip = 200, limit = 100` када је `page size = 100` и `page = 2`.
   4. Дечији коментари и даље улазе у пагинацију. Ово можете заобићи коришћењем опције `asTree`.
      - Можете пагинирати децу преко `limitChildren` и `skipChildren`.
      - Можете ограничити дубину тредова који се враћају преко `maxTreeDepth`.

### Теме

1. Када користите `Precalculated Pagination`, коментари су груписани по *страници* и коментари у тредовима утичу на укупну страницу.
   1. На овај начин, тредови се могу одредити на клијенту на основу `parentId`.
   2. На пример, на страници са једним коментаром највишег нивоа и 29 одговора, и подешавањем `page=0` у API-ју — добићете само коментар највишег нивоа и 29 деце.
   3. [Пример слике који илуструје више страна.](https://blog.winricklabs.com/images/fc-pagination02.png)
2. Када користите `Flexible Pagination`, можете дефинисати параметар `parentId`.
   1. Поставите га на null да бисте добили само коментаре највишег нивоа.
   2. Затим, да бисте видели тредове, позовите API поново и проследите `parentId`.
   3. Уобичајено решење је да направите позив API-ју за коментаре највишег нивоа и онда паралелне позиве да бисте добили коментаре за децу сваког коментара.
3. __НОВО: од фебруара 2023.!__ Добијте као стабло користећи `&asTree=true`.
   1. Ово можете сматрати као `Flexible Pagination as a Tree`.
   2. Само коментари највишег нивоа се рачунају у пагинацији.
   3. Поставите `parentId=null` да бисте започели стабло од корена (мора се подесити `parentId`).
   4. Поставите `skip` и `limit` за пагинацију.
   5. Поставите `asTree` на `true`.
   6. Трошак кредита се повећава за `2x`, јер наш бекенд мора много више да ради у овом сценарију.
   7. Поставите `maxTreeDepth`, `limitChildren`, и `skipChildren` по потреби.

### Објашњење стабала

Када користите `asTree`, може бити тешко разумети пагинацију. Ево корисне графике:

<div class="screenshot white-bg">
    <div class="title">Дијаграм пагинације стабла</div>
    <img class="screenshot-image" src="/images/fastcomments-comments-api-tree.png" alt="Дијаграм пагинације стабла" />
</div>

### Преузимање коментара у контексту корисника

API /comments се може користити у два контекста, за различите случајеве употребе:

- За враћање коментара сортираних и означених информацијама за израду сопственог клијента.
  - У овом случају дефинишите query параметар `contextUserId`.
- За преузимање коментара са вашег бекенда за прилагођене интеграције.
  - Платформа ће подразумевано користити ово без `contextUserId`. 

[inline-code-attrs-start title = 'Прерачуната пагинација коментара'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&page=0&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR'
[inline-code-end]

[inline-code-attrs-start title = 'Флексибилна пагинација коментара'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10'
[inline-code-end]

[inline-code-attrs-start title = 'Флексибилна пагинација коментара у контексту корисника'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10&contextUserId=my-user-id'
[inline-code-end]

[inline-code-attrs-start title = 'Флексибилна пагинација коментара у контексту корисника (само коментари највишег нивоа)'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10&contextUserId=my-user-id&parentId=null'
[inline-code-end]

### Добијање коментара као стабло

Могуће је добити коментаре враћене као стабло, при чему пагинација рачуна само коментаре највишег нивоа.

[inline-code-attrs-start title = 'Коментари као стабло у контексту корисника'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10&contextUserId=my-user-id&parentId=null&asTree=true'
[inline-code-end]

Желите да добијете само коментаре највишег нивоа и непосредну децу? Ево једног начина:

[inline-code-attrs-start title = 'Коментари као стабло са максималном дубином'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10&contextUserId=my-user-id&parentId=null&asTree=true&maxTreeDepth=1&limitChildren=10'
[inline-code-end]

Међутим, у вашем UI-у можда ћете морати да знате да ли да прикажете дугме „пкажи одговоре“ на
сваком коментару. Када се коментари дохватају као стабло, постоји својство `hasChildren` које је
додато коментарима када је применљиво.

### Добијање коментара као стабло, претраживање по хештегу

Могуће је претраживати по хештегу користећи API, преко целог вашег tenant-а (није ограничено на једну страницу или `urlId`).

У овом примеру, ми изостављамо `urlId`, и претражујемо по више хештегова. API ће вратити само коментаре који имају све тражене хештегове.

[inline-code-attrs-start title = 'Коментари као стабло у контексту корисника, по хештегу'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10&contextUserId=my-user-id&parentId=null&asTree=true&hashTag=TestTag&hashTag=OtherTestTag'
[inline-code-end]

### Сви параметри захтева

[inline-code-attrs-start title = 'Структура захтева за коментаре'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

[inline-code-attrs-start title = 'Структура одговора за коментаре'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

### Корисни савети

#### URL ID

Вероватно ћете желети да користите `Comment` API са параметром `urlId`. Можете позвати `Pages` API прво, да видите како изгледају вредности `urlId` које су вам доступне.

#### Анонимне радње

За анонимно коментарисање вероватно ћете желети да проследите `anonUserId` када дохватате коментаре, и када извршавате означавање (flagging) и блокирање.

(!) Ово је захтевано за многе продавнице апликација јер корисници морају моћи да означавају садржај који су креирали други корисници и који могу видети, чак и ако нису пријављени. Ако то не учините, ваша апликација може бити уклоњена из те продавнице.

#### Коментари се не враћају

Проверите да ли су ваши коментари одобрени и да нису означени као спам.
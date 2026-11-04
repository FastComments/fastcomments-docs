[api-resource-header-start name = 'Comment'; route = 'GET /api/v1/comments'; creditsCost = 1; api-resource-header-end]

Deze API wordt gebruikt om reacties op te halen om aan een gebruiker te tonen. Zo worden bijvoorbeeld automatisch niet-goedgekeurde reacties en spam eruit gefilterd.

### Paginering

Paginering kan op twee manieren worden gedaan, afhankelijk van de prestatie-eisen en het gebruiksdoel:

1. Snelste: **Voorberekende paginering**:
   1. Dit is hoe FastComments werkt wanneer je onze kant-en-klare widgets en clients gebruikt.
   2. Op "volgende" klikken verhoogt simpelweg het paginanummer.
   3. Je kunt dit zien als ophalen via een key-value store.
   4. Op deze manier definieer je simpelweg een `page` parameter beginnend bij `0` en een sorteerrichting als `direction`.
   5. Paginagroottes kunnen worden aangepast via aanpassingsregels.
2. Meest flexibel: **Flexibele paginering**:
   1. Op deze manier kun je aangepaste `limit` en `skip` parameters definiëren. Geef `page` niet door.
   2. Sorteerrichting `direction` wordt ook ondersteund.
   3. `limit` is het totale aantal dat wordt geretourneerd nadat `skip` is toegepast.
      - Voorbeeld: stel `skip = 200, limit = 100` in wanneer `page size = 100` en `page = 2`.
   4. Reacties op reacties tellen nog steeds mee voor de paginering. Je kunt dit omzeilen met de `asTree` optie.
      - Je kunt reacties op reacties pagineren via `limitChildren` en `skipChildren`.
      - Je kunt de diepte van de geretourneerde threads beperken via `maxTreeDepth`.

### Threads

1. Bij gebruik van `Precalculated Pagination` worden reacties gegroepeerd per *pagina* en beïnvloeden reacties in threads de totale pagina.
   1. Op deze manier kunnen threads aan de clientzijde worden bepaald op basis van `parentId`.
   2. Bijvoorbeeld, met een pagina met één reactie op het hoogste niveau en 29 antwoorden, en het instellen van `page=0` in de API - krijg je alleen de reactie op het hoogste niveau en de 29 kinderen.
2. Bij gebruik van `Flexible Pagination` kun je een `parentId` parameter definiëren.
   1. Stel deze in op null om alleen reacties op het hoogste niveau te krijgen.
   2. Om vervolgens threads te bekijken, roep je de API opnieuw aan en geef je `parentId` door.
   3. Een veelgebruikte oplossing is een API-aanroep doen voor de reacties op het hoogste niveau en vervolgens parallelle API-aanroepen doen om de reacties op te halen voor de kinderen van elke reactie.
3. __NIEUW Vanaf feb 2023!__ Ophalen als een boomstructuur met `&asTree=true`.
   1. Je kunt dit zien als `Flexible Pagination as a Tree`.
   2. Alleen de reacties op het hoogste niveau tellen mee voor de paginering.
   3. Stel `parentId=null` in om de boom bij de wortel te starten (je moet `parentId` instellen).
   4. Stel `skip` en `limit` in voor paginering.
   5. Stel `asTree` in op `true`.
   6. De credits-kosten stijgen met `2x`, omdat onze backend in dit scenario veel meer werk moet verrichten.
   7. Stel `maxTreeDepth`, `limitChildren` en `skipChildren` naar wens in.

### Boomstructuren uitgelegd

Bij gebruik van `asTree` kan het lastig zijn om over paginering na te denken. Hier is een handige afbeelding:

<div class="screenshot white-bg">
    <div class="title">Tree Pagination Diagram</div>
    <img class="screenshot-image" src="/images/fastcomments-comments-api-tree.png" alt="Tree Pagination Diagram" />
</div>

### Reacties ophalen in de context van een gebruiker

De `/comments` API kan in twee contexten worden gebruikt, voor verschillende gebruiksdoelen:

- Voor het retourneren van reacties, gesorteerd en voorzien van informatie voor het bouwen van je eigen client.
  - Definieer in dit geval een `contextUserId` query parameter.
- Voor het ophalen van reacties vanuit je backend voor aangepaste integraties.
  - Het platform gebruikt standaard deze context zonder `contextUserId`.

[inline-code-attrs-start title = 'Reacties voorberekende paginering'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&page=0&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR'
[inline-code-end]

[inline-code-attrs-start title = 'Reacties flexibele paginering'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10'
[inline-code-end]

[inline-code-attrs-start title = 'Reacties flexibele paginering in gebruikerscontext'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10&contextUserId=my-user-id'
[inline-code-end]

[inline-code-attrs-start title = 'Reacties flexibele paginering in gebruikerscontext, alleen reacties op het hoogste niveau'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10&contextUserId=my-user-id&parentId=null'
[inline-code-end]

### Reacties ophalen als boomstructuur

Het is mogelijk om de reacties als een boomstructuur te laten retourneren, waarbij de paginering alleen de reacties op het hoogste niveau telt.

[inline-code-attrs-start title = 'Reacties als boomstructuur in gebruikerscontext'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10&contextUserId=my-user-id&parentId=null&asTree=true'
[inline-code-end]

Wil je alleen de reacties op het hoogste niveau en de directe kinderen? Hier is een manier:

[inline-code-attrs-start title = 'Reacties als boomstructuur met maximale diepte'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10&contextUserId=my-user-id&parentId=null&asTree=true&maxTreeDepth=1&limitChildren=10'
[inline-code-end]

In je UI wil je echter mogelijk weten of je een "antwoorden tonen"-knop moet tonen bij
elke reactie. Bij het ophalen van reacties via een boomstructuur wordt een `hasChildren` eigenschap
aan reacties toegevoegd wanneer van toepassing.

### Reacties ophalen als boomstructuur, zoeken op hashtag

Het is mogelijk om via de API op hashtag te zoeken, binnen je hele tenant (niet beperkt tot één pagina of `urlId`).

In dit voorbeeld laten we `urlId` weg en zoeken we op meerdere hashtags. De API retourneert alleen reacties die alle opgevraagde hashtags bevatten.

[inline-code-attrs-start title = 'Reacties als boomstructuur in gebruikerscontext, op hashtag'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10&contextUserId=my-user-id&parentId=null&asTree=true&hashTag=TestTag&hashTag=OtherTestTag'
[inline-code-end]

### Alle request-parameters

[inline-code-attrs-start title = 'Reacties request-structuur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

### Het antwoord

[inline-code-attrs-start title = 'Reacties response-structuur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

### Handige tips

#### URL ID

Je wilt waarschijnlijk de `Comment` API gebruiken met de `urlId` parameter. Je kunt eerst de `Pages` API aanroepen om te zien hoe de `urlId` waarden die voor je beschikbaar zijn eruitzien.

#### Anonieme acties

Voor anoniem reageren wil je waarschijnlijk `anonUserId` doorgeven bij het ophalen van reacties, en bij het rapporteren en blokkeren.

(!) Dit is vereist voor veel app stores, omdat gebruikers door gebruikers gemaakte content die ze kunnen zien moeten kunnen rapporteren, zelfs als ze niet ingelogd zijn. Als je dit niet doet, kan je app uit de betreffende store worden verwijderd.

#### Reacties worden niet geretourneerd

Controleer of je reacties zijn goedgekeurd en geen spam zijn.
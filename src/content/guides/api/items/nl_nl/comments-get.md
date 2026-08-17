[api-resource-header-start name = 'Comment'; route = 'GET /api/v1/comments'; creditsCost = 1; api-resource-header-end]

Deze API wordt gebruikt om reacties op te halen voor weergave aan een gebruiker. Bijvoorbeeld, hij filtert automatisch niet-goedgekeurde of spamreacties.

### Pagination

Paginering kan op één van twee manieren worden gedaan, afhankelijk van prestatie-eisen en gebruikssituatie:

1. **Snelste:** **Precalculated Pagination**:
   1. Dit is hoe FastComments werkt wanneer je onze kant-en-klare widgets en clients gebruikt.
   2. Op "volgende" klikken verhoogt simpelweg het paginanummer.
   3. Je kunt dit zien als opgehaald uit een key-value store.
   4. Op deze manier definieer je eenvoudig een `page`-parameter die start bij `0` en een sorteerrichting als `direction`.
   5. Paginagroottes kunnen worden aangepast via aanpassingsregels.
2. **Meest flexibel:** **Flexible Pagination**:
   1. Op deze manier kun je aangepaste `limit`- en `skip`-parameters definiëren. Geef geen `page` mee.
   2. `direction` sortering wordt ook ondersteund.
   3. `limit` is het totale aantal dat moet worden geretourneerd nadat `skip` is toegepast.
      - Voorbeeld: stel `skip = 200, limit = 100` in wanneer `page size = 100` en `page = 2`.
   4. Kindreacties tellen nog steeds mee in de paginering. Je kunt dit omzeilen met de `asTree`-optie.
      - Je kunt kinderen pagineren via `limitChildren` en `skipChildren`.
      - Je kunt de diepte van de teruggegeven threads beperken via `maxTreeDepth`.

### Threads

1. Bij gebruik van `Precalculated Pagination` worden reacties gegroepeerd per *page* en beïnvloeden reacties in threads de gehele pagina.
   1. Op deze manier kunnen threads op de client worden bepaald op basis van `parentId`.
   2. Bijvoorbeeld, met een pagina met één top-level reactie en 29 antwoorden, en `page=0` in de API - je krijgt alleen de top-level reactie en de 29 kinderen.
2. Bij gebruik van `Flexible Pagination` kun je een `parentId`-parameter definiëren.
   1. Stel dit in op null om alleen top-level reacties te krijgen.
   2. Roep vervolgens de API opnieuw aan en geef `parentId` door om threads te bekijken.
   3. Een veelvoorkomende oplossing is een API-aanroep te doen voor de top-level reacties en vervolgens parallel API-aanroepen te doen om reacties voor de kinderen van elke reactie op te halen.
3. __NIEUW vanaf feb 2023!__ Haal op als een boom met `&asTree=true`.
   1. Je kunt dit zien als `Flexible Pagination als een Boom`.
   2. Alleen de top-level reacties tellen mee in de paginering.
   3. Stel `parentId=null` in om de boom bij de root te starten (je moet `parentId` instellen).
   4. Stel `skip` en `limit` in voor paginering.
   5. Stel `asTree` in op `true`.
   6. De creditkosten stijgen met `2x`, omdat onze backend in dit scenario veel meer werk moet doen.
   7. Stel `maxTreeDepth`, `limitChildren` en `skipChildren` in naar wens.

### Trees Explained

Bij gebruik van `asTree` kan het moeilijk zijn om over paginering na te denken. Hier is een handige afbeelding:

<div class="screenshot white-bg">
    <div class="title">Boom Paginering Diagram</div>
    <img class="screenshot-image" src="/images/fastcomments-comments-api-tree.png" alt="Boom Paginering Diagram" />
</div>

### Fetching Comments in The Context of a User

De `/comments` API kan in twee contexten worden gebruikt, voor verschillende gebruikssituaties:

- Voor het retourneren van reacties gesorteerd en getagd met informatie voor het bouwen van je eigen client.
  - Definieer in dit geval een `contextUserId` query-parameter.
- Voor het ophalen van reacties van je backend voor aangepaste integraties.
  - Het platform zal dit standaard doen zonder `contextUserId`.

[inline-code-attrs-start title = 'Reacties Precalculated Pagination'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&page=0&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR'
[inline-code-end]

[inline-code-attrs-start title = 'Reacties Flexible Pagination'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10'
[inline-code-end]

[inline-code-attrs-start title = 'Reacties Flexible Pagination in Gebruikerscontext'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10&contextUserId=my-user-id'
[inline-code-end]

[inline-code-attrs-start title = 'Reacties Flexible Pagination in Gebruikerscontext alleen voor top-level reacties'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10&contextUserId=my-user-id&parentId=null'
[inline-code-end]

### Get Comments as a Tree

Het is mogelijk om de reacties als een boom te ontvangen, waarbij alleen de top-level reacties meetellen voor paginering.

[inline-code-attrs-start title = 'Reacties As-A-Tree in Gebruikerscontext'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10&contextUserId=my-user-id&parentId=null&asTree=true'
[inline-code-end]

Wil je alleen de top-level reacties en de directe kinderen ophalen? Hier is één manier:

[inline-code-attrs-start title = 'Reacties As-A-Tree met Max Diepte'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10&contextUserId=my-user-id&parentId=null&asTree=true&maxTreeDepth=1&limitChildren=10'
[inline-code-end]

Echter, in je UI moet je mogelijk weten of je een "toon antwoorden"-knop op elke reactie moet weergeven. Bij het ophalen van reacties via een boom is er een `hasChildren`-eigenschap getagd op reacties wanneer van toepassing.

### Get Comments as a Tree, Searching by Hash Tag

Het is mogelijk om te zoeken op hashtag via de API, over je volledige tenant (niet beperkt tot één pagina, of `urlId`).

In dit voorbeeld laten we `urlId` weg en zoeken we op meerdere hashtags. De API zal alleen reacties retourneren die alle gevraagde hashtags bevatten.

[inline-code-attrs-start title = 'Reacties As-A-Tree in Gebruikerscontext, per Hash Tag'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10&contextUserId=my-user-id&parentId=null&asTree=true&hashTag=TestTag&hashTag=OtherTestTag'
[inline-code-end]

### All Request Params

[inline-code-attrs-start title = 'Reacties Verzoekstructuur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

### The Response

[inline-code-attrs-start title = 'Reacties Responsstructuur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

### Helpful Tips

#### URL ID

Je wilt waarschijnlijk de `Comment` API gebruiken met de `urlId`-parameter. Je kunt eerst de `Pages` API aanroepen om te zien hoe de beschikbare `urlId`-waarden eruitzien. 

#### Anonymous Actions

Voor anonieme reacties wil je waarschijnlijk `anonUserId` doorgeven bij het ophalen van reacties, en bij het uitvoeren van vlaggen en blokkeren.

(!) Dit is vereist voor veel app stores omdat gebruikers in staat moeten zijn om door hen bekeken gebruikersgegenereerde inhoud te markeren, zelfs als ze niet zijn ingelogd. Het niet doen kan ertoe leiden dat je app uit die store wordt verwijderd.

#### Comments Not Being Returned

Controleer of je reacties zijn goedgekeurd en geen spam zijn.

---
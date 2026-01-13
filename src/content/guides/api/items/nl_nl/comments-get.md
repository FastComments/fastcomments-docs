[api-resource-header-start name = 'Comment'; route = 'GET /api/v1/comments'; creditsCost = 1; api-resource-header-end]

Deze API wordt gebruikt om reacties op te halen voor weergave aan een gebruiker. Bijvoorbeeld filtert hij automatisch niet-goedgekeurde of spamreacties eruit.

### Paginering

Paginering kan op een van twee manieren worden gedaan, afhankelijk van prestatie-eisen en gebruiksgeval:

1. Snelste: **Precalculated Pagination**:
   1. Dit is hoe FastComments werkt wanneer u onze vooraf gebouwde widgets en clients gebruikt.
   2. Het klikken op "next" verhoogt eenvoudigweg het paginanummer.
   3. U kunt dit zien als opgehaald via een key-value store.
   4. Definieer op deze manier eenvoudig een `page`-parameter beginnend bij `0` en een sorteerrichting als `direction`.
   5. Paginagroottes kunnen worden aangepast via aanpassingsregels.
2. Meest flexibel: **Flexible Pagination**:
   1. Op deze manier kunt u aangepaste `limit`- en `skip`-parameters definiëren. Geef geen `page` door.
   2. Sorteerrichting `direction` wordt ook ondersteund.
   3. `limit` is het totale aantal dat wordt geretourneerd nadat `skip` is toegepast.
      - Voorbeeld: stel `skip = 200, limit = 100` wanneer `page size = 100` en `page = 2`.
   4. Kindreacties tellen nog steeds mee in de paginering. U kunt dit omzeilen met de optie `asTree`.
      - U kunt kinderen pagineren via `limitChildren` en `skipChildren`.
      - U kunt de diepte van de teruggegeven threads beperken via `maxTreeDepth`.

### Threads

1. Wanneer u `Precalculated Pagination` gebruikt, worden reacties gegroepeerd per *pagina* en beïnvloeden reacties in threads de gehele pagina.
   1. Op deze manier kunnen threads op de client worden bepaald op basis van `parentId`.
   2. Bijvoorbeeld, bij een pagina met één topniveau-reactie en 29 antwoorden, en het instellen van `page=0` in de API - krijgt u alleen de topniveau-reactie en de 29 kinderen.
   3. [Voorbeeldafbeelding die meerdere pagina's illustreert.](https://blog.winricklabs.com/images/fc-pagination02.png)
2. Wanneer u `Flexible Pagination` gebruikt, kunt u een `parentId`-parameter definiëren.
   1. Stel deze in op null om alleen topniveau-reacties te krijgen.
   2. Om vervolgens threads te bekijken, roept u de API opnieuw aan en geeft u `parentId` door.
   3. Een gebruikelijke oplossing is om een API-aanroep te doen voor de topniveau-reacties en vervolgens parallelle API-aanroepen te doen om reacties op te halen voor de kinderen van elke reactie.
3. __NIEUW sinds feb 2023!__ Ophalen als een boom met `&asTree=true`.
   1. U kunt dit zien als `Flexible Pagination as a Tree`.
   2. Alleen de topniveau-reacties tellen mee in de paginering.
   3. Stel `parentId=null` in om de boom bij de root te starten (u moet `parentId` instellen).
   4. Stel `skip` en `limit` in voor paginering.
   5. Stel `asTree` in op `true`.
   6. De credits-kosten nemen toe met `2x`, omdat onze backend in dit scenario veel meer werk moet doen.
   7. Stel `maxTreeDepth`, `limitChildren` en `skipChildren` in naar wens.

### Bomen uitgelegd

Wanneer u `asTree` gebruikt, kan het moeilijk zijn om over paginering na te denken. Hier is een handige afbeelding:

<div class="screenshot white-bg">
    <div class="title">Diagram van boom-paginering</div>
    <img class="screenshot-image" src="/images/fastcomments-comments-api-tree.png" alt="Tree Pagination Diagram" />
</div>

### Reacties ophalen in de context van een gebruiker

De `/comments` API kan in twee contexten worden gebruikt, voor verschillende gebruiksgevallen:

- Voor het retourneren van reacties gesorteerd en gelabeld met informatie voor het bouwen van uw eigen client.
  - Gebruik in dit geval een queryparameter `contextUserId`.
- Voor het ophalen van reacties vanaf uw backend voor aangepaste integraties.
  - Het platform zal standaard naar dit terugvallen zonder `contextUserId`. 

[inline-code-attrs-start title = 'Reacties Voorafberekende Paginering'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&page=0&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR'
[inline-code-end]

[inline-code-attrs-start title = 'Reacties Flexibele Paginering'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10'
[inline-code-end]

[inline-code-attrs-start title = 'Reacties Flexibele Paginering in Gebruikerscontext'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10&contextUserId=my-user-id'
[inline-code-end]

[inline-code-attrs-start title = 'Reacties Flexibele Paginering in Gebruikerscontext voor Alleen Topniveau Reacties'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10&contextUserId=my-user-id&parentId=null'
[inline-code-end]

### Reacties als boom ophalen

Het is mogelijk om reacties terug te krijgen als een boom, waarbij alleen de topniveau-reacties meetellen voor de paginering.

[inline-code-attrs-start title = 'Reacties Als Boom in Gebruikerscontext'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10&contextUserId=my-user-id&parentId=null&asTree=true'
[inline-code-end]

Wilt u alleen de topniveau-reacties en de directe kinderen ophalen? Hier is één manier:

[inline-code-attrs-start title = 'Reacties Als Boom met Maximale Diepte'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10&contextUserId=my-user-id&parentId=null&asTree=true&maxTreeDepth=1&limitChildren=10'
[inline-code-end]

Echter, in uw UI moet u misschien weten of u op elke reactie een knop "toon reacties" moet weergeven. Bij het ophalen van reacties via een boom is er een `hasChildren`-eigenschap toegevoegd aan reacties wanneer van toepassing.

### Reacties als boom ophalen, zoeken op hashtag

Het is mogelijk om op hashtag te zoeken met de API, over uw gehele tenant (niet beperkt tot één pagina of `urlId`).

In dit voorbeeld laten we `urlId` weg en zoeken we op meerdere hashtags. De API retourneert alleen reacties die alle gevraagde hashtags bevatten.

[inline-code-attrs-start title = 'Reacties Als Boom in Gebruikerscontext, Op Hashtag'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10&contextUserId=my-user-id&parentId=null&asTree=true&hashTag=TestTag&hashTag=OtherTestTag'
[inline-code-end]

### Alle verzoekparameters

[inline-code-attrs-start title = 'Structuur van Reactiesaanvraag'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface CommentsRequestQueryParams {
    tenantId: string
    API_KEY: string
    /** De urlId (pagina-url, of artikel-id) waarmee de reacties zijn geassocieerd. **/
    urlId?: string
    /** Beperk de geretourneerde reacties tot die van deze gebruiker. **/
    userId?: string
    /** Gebruik dit om te zoeken op hashtag. Om naar de doorsnede van meerdere hashtags te filteren, gebruik &hashTag=a&hashTag=b. **/
    hashTag?: string
    /** De sorteerrichting. Standaard is MR (Meest Relevant). Andere opties zijn OF (Oudste Eerst) en NF (Nieuwste Eerst). **/
    direction?: 'MR' | 'OF' | 'NF'
    /** Precalculated Pagination: de pagina om op te halen, beginnend met 0. Geef -1 door om alle reacties op te halen (tot 250). **/
    page?: number
    /** Flexibele paginering: hoeveel reacties moeten we retourneren? **/
    limit?: number
    /** Flexibele paginering: hoeveel kindreacties moeten we voor elk ouder-item retourneren? **/
    limitChildren?: number
    /** Flexibele paginering: hoeveel reacties moeten we overslaan? **/
    skip?: number
    /** Flexibele paginering: hoeveel kindreacties moeten we overslaan voor elk ouder-item? **/
    skipChildren?: number
    /** Voor het bepalen van geblokkeerde en gemarkeerde reacties. **/
    contextUserId?: string
    /** Voor het bepalen van geblokkeerde en gemarkeerde reacties. **/
    anonUserId?: string
    /** Voor het ophalen van kindreacties. **/
    parentId?: string
    /** Voor het ophalen als een boom. **/
    asTree?: boolean
    /** Hoe ver in de boom moeten we gegevens teruggeven? 0 geeft geen kinderen terug. 1 geeft onmiddellijke kinderen terug, enz. **/
    maxTreeDepth?: number
}
[inline-code-end]

### De respons

[inline-code-attrs-start title = 'Structuur van Reactiesrespons'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface CommentsResponse {
    status: 'success' | 'failed'
    /** Opgenomen bij mislukking. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-url-id' | 'missing-date' | 'unauthorized-page' | 'invalid-pagination-request' | 'invalid-limit' | 'invalid-limit-children' | 'invalid-skip' | 'invalid-skip-children' | 'invalid-max-tree-depth'
    /** Opgenomen bij mislukking. **/
    reason?: string
    /** De reacties! **/
    comments: Comment[]
}
[inline-code-end]

### Handige tips

#### URL ID

Waarschijnlijk wilt u de `Comment`-API gebruiken met de parameter `urlId`. U kunt eerst de `Pages`-API aanroepen om te zien hoe de voor u beschikbare `urlId`-waarden eruitzien. 

#### Anonieme acties

Voor anoniem reageren wilt u waarschijnlijk `anonUserId` doorgeven bij het ophalen van reacties, en bij het uitvoeren van markeringen en blokkeringen.

(!) Dit is vereist voor veel app stores omdat gebruikers in staat moeten zijn user-generated content die ze zien te markeren, zelfs als ze niet zijn ingelogd. Het niet doen hiervan kan ertoe leiden dat uw app uit genoemde store wordt verwijderd.

#### Reacties worden niet geretourneerd

Controleer of uw reacties zijn goedgekeurd en geen spam zijn.

---
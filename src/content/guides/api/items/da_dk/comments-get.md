[api-resource-header-start name = 'Comment'; route = 'GET /api/v1/comments'; creditsCost = 1; api-resource-header-end]

Dette API bruges til at hente kommentarer til visning for en bruger. For eksempel filtrerer det automatisk ikke-godkendte eller spam-kommentarer.

### Paginering

Paginering kan gøres på en af to måder, afhængigt af performance-krav og brugssag:

1. Hurtigst: **Forudberegnet paginering**:
   1. Dette er hvordan FastComments fungerer, når du bruger vores forbyggede widgets og klienter.
   2. At klikke "næste" øger simpelthen side-tælleren.
   3. Du kan tænke på dette som hentet fra en key-value store.
   4. På denne måde definerer du blot en `page` parameter startende ved `0` og en sorteringsretning som `direction`.
   5. Sidestørrelser kan tilpasses via tilpasningsregler.
2. Mest fleksibel: **Fleksibel paginering**:
   1. På denne måde kan du definere brugerdefinerede `limit` og `skip` parametre. Send ikke `page`.
   2. Sorterings-`direction` understøttes også.
   3. `limit` er det samlede antal der returneres efter `skip` er anvendt.
      - Eksempel: sæt `skip = 200, limit = 100` når `page size = 100` og `page = 2`.
   4. Underkommentarer tæller stadig med i pagineringen. Du kan komme uden om dette ved at bruge `asTree`-muligheden.
      - Du kan paginere børn via `limitChildren` og `skipChildren`.
      - Du kan begrænse dybden af de tråde der returneres via `maxTreeDepth`.

### Tråde

1. Når du bruger `Precalculated Pagination`, er kommentarer grupperet efter *side* og kommentarer i tråde påvirker den overordnede side.
   1. På denne måde kan tråde bestemmes på klienten baseret på `parentId`.
   2. For eksempel, med en side med én topniveau-kommentar og 29 svar, og sætning af `page=0` i API'et - vil du kun få topniveau-kommentaren og de 29 børn.
   3. [Eksempelbillede der illustrerer flere sider.](https://blog.winricklabs.com/images/fc-pagination02.png)
2. Når du bruger `Flexible Pagination`, kan du definere en `parentId` parameter.
   1. Sæt denne til null for kun at få topniveau-kommentarer.
   2. For så at se tråde, kald API'et igen og giv `parentId`.
   3. En almindelig løsning er at lave et API-kald for topniveau-kommentarerne og derefter lave parallelle API-kald for at hente kommentarer for børnene af hver kommentar.
3. __NYT fra feb 2023!__ Hent som et træ ved at bruge `&asTree=true`.
   1. Du kan tænke på dette som `Flexible Pagination as a Tree`.
   2. Kun topniveau-kommentarer tæller i pagineringen.
   3. Sæt `parentId=null` for at starte træet ved roden (du skal sætte `parentId`).
   4. Sæt `skip` og `limit` for paginering.
   5. Sæt `asTree` til `true`.
   6. Kreditforbruget øges med `2x`, da vores backend skal lave meget mere arbejde i dette scenarie.
   7. Sæt `maxTreeDepth`, `limitChildren`, og `skipChildren` som ønsket.

### Træer forklaret

Når du bruger `asTree`, kan det være svært at sætte sig ind i paginering. Her er en nyttig grafik:

<div class="screenshot white-bg">
    <div class="title">Diagram over paginering i træ</div>
    <img class="screenshot-image" src="/images/fastcomments-comments-api-tree.png" alt="Tree Pagination Diagram" />
</div>

### Hentning af kommentarer i brugerkontekst

`/comments` API'et kan bruges i to kontekster, til forskellige brugssager:

- Til at returnere kommentarer sorteret og tagget med information til at bygge din egen klient.
  - I dette tilfælde definer en `contextUserId` forespørgselsparameter.
- Til at hente kommentarer fra din backend til brugerdefinerede integrationer.
  - Platformen vil som standard bruge dette uden `contextUserId`. 

[inline-code-attrs-start title = 'Kommentarer Forudberegnet Paginering'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&page=0&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR'
[inline-code-end]

[inline-code-attrs-start title = 'Kommentarer Fleksibel Paginering'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10'
[inline-code-end]

[inline-code-attrs-start title = 'Kommentarer Fleksibel Paginering i Brugerkontekst'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10&contextUserId=my-user-id'
[inline-code-end]

[inline-code-attrs-start title = 'Kommentarer Fleksibel Paginering i Brugerkontekst kun for Topniveau-kommentarer'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10&contextUserId=my-user-id&parentId=null'
[inline-code-end]

### Hent kommentarer som et træ

Det er muligt at få kommentarerne returneret som et træ, hvor paginering kun tæller topniveau-kommentarerne.

[inline-code-attrs-start title = 'Kommentarer som træ i Brugerkontekst'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10&contextUserId=my-user-id&parentId=null&asTree=true'
[inline-code-end]

Vil du kun hente topniveau-kommentarerne og de umiddelbare børn? Her er én måde:

[inline-code-attrs-start title = 'Kommentarer som træ med Maksimal Dybde'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10&contextUserId=my-user-id&parentId=null&asTree=true&maxTreeDepth=1&limitChildren=10'
[inline-code-end]

Dog kan du i din UI få brug for at vide, om du skal vise en "show replies"-knap på hver kommentar. Når du henter kommentarer via et træ, er der en `hasChildren` property knyttet til kommentarer, når det er relevant.

### Hent kommentarer som træ, søgning efter hashtag

Det er muligt at søge efter hashtag ved hjælp af API'et, på tværs af hele din tenant (ikke begrænset til en side eller `urlId`).

I dette eksempel udelader vi `urlId`, og vi søger efter flere hashtags. API'et vil kun returnere kommentarer, der har alle de forespurgte hashtags.

[inline-code-attrs-start title = 'Kommentarer som træ i Brugerkontekst, efter Hashtag'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10&contextUserId=my-user-id&parentId=null&asTree=true&hashTag=TestTag&hashTag=OtherTestTag'
[inline-code-end]

### Alle forespørgselsparametre

[inline-code-attrs-start title = 'Kommentarer Forespørgselsstruktur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

### Svaret

[inline-code-attrs-start title = 'Kommentarer Svarstruktur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

### Brugbare tips

#### URL-id

Du vil sandsynligvis bruge `Comment` API'et med `urlId` parameteren. Du kan kalde `Pages` API'et først for at se, hvordan de `urlId` værdier der er tilgængelige for dig ser ud. 

#### Anonyme handlinger

For anonym kommentering vil du sandsynligvis sende `anonUserId` når du henter kommentarer, og når du foretager flagging og blokering.

(!) Dette er påkrævet for mange app-butikker, da brugere skal kunne markere brugeroprettet indhold, de kan se, selvom de ikke er logget ind. Hvis ikke, kan din app blive fjernet fra den pågældende butik.

#### Kommentarer returneres ikke

Kontroller at dine kommentarer er godkendte, og ikke er spam.

---
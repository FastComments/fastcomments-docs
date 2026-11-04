[api-resource-header-start name = 'Comment'; route = 'GET /api/v1/comments'; creditsCost = 1; api-resource-header-end]

Denne API bruges til at hente kommentarer til visning for en bruger. Den filtrerer f.eks. automatisk
ikke-godkendte kommentarer og spam fra.

### Paginering

Paginering kan udføres på to måder, afhængigt af ydelseskrav og brugsscenarie:

1. Hurtigst: **Forudberegnet paginering**:
   1. Sådan fungerer FastComments, når du bruger vores færdigbyggede widgets og klienter.
   2. Et klik på "næste" øger blot sidetallet.
   3. Du kan tænke på dette som data hentet fra et key-value store.
   4. På denne måde definerer du blot en `page`-parameter, der starter ved `0`, samt en sorteringsretning som `direction`.
   5. Sidestørrelser kan tilpasses via tilpasningsregler.
2. Mest fleksibel: **Fleksibel paginering**:
   1. På denne måde kan du definere brugerdefinerede `limit`- og `skip`-parametre. Send ikke `page`.
   2. Sorteringsretningen `direction` understøttes også.
   3. `limit` er det samlede antal, der returneres, efter `skip` er anvendt.
      - Eksempel: sæt `skip = 200, limit = 100`, når `page size = 100` og `page = 2`.
   4. Underkommentarer tælles stadig med i pagineringen. Du kan omgå dette med `asTree`-indstillingen.
      - Du kan paginere underkommentarer via `limitChildren` og `skipChildren`.
      - Du kan begrænse dybden af de returnerede tråde via `maxTreeDepth`.

### Tråde

1. Når du bruger `Forudberegnet paginering`, grupperes kommentarer efter *side*, og kommentarer i tråde påvirker den samlede side.
   1. På denne måde kan tråde bestemmes på klienten baseret på `parentId`.
   2. For eksempel, med en side med én topniveau-kommentar og 29 svar, og med `page=0` sat i API'en - får du kun topniveau-kommentaren og de 29 underkommentarer.
2. Når du bruger `Fleksibel paginering`, kan du definere en `parentId`-parameter.
   1. Sæt denne til null for kun at få topniveau-kommentarer.
   2. For derefter at se tråde, kald API'en igen og send `parentId`.
   3. En almindelig løsning er at foretage et API-kald for topniveau-kommentarerne og derefter foretage parallelle API-kald for at hente kommentarer for underkommentarerne til hver kommentar.
3. __NYT fra feb. 2023!__ Hent som et træ med `&asTree=true`.
   1. Du kan tænke på dette som `Fleksibel paginering som træ`.
   2. Kun topniveau-kommentarerne tælles med i pagineringen.
   3. Sæt `parentId=null` for at starte træet ved roden (du skal sætte `parentId`).
   4. Sæt `skip` og `limit` til paginering.
   5. Sæt `asTree` til `true`.
   6. Credit-omkostningerne stiger med `2x`, da vores backend skal udføre langt mere arbejde i dette scenarie.
   7. Sæt `maxTreeDepth`, `limitChildren` og `skipChildren` efter behov.

### Træer forklaret

Når du bruger `asTree`, kan det være svært at ræsonnere om paginering. Her er en nyttig grafik:

<div class="screenshot white-bg">
    <div class="title">Diagram over træpaginering</div>
    <img class="screenshot-image" src="/images/fastcomments-comments-api-tree.png" alt="Diagram over træpaginering" />
</div>

### Hentning af kommentarer i en brugers kontekst

`/comments`-API'en kan bruges i to kontekster til forskellige brugsscenarier:

- Til at returnere kommentarer sorteret og mærket med information til at bygge din egen klient.
  - I dette tilfælde skal du definere en `contextUserId` query-parameter.
- Til at hente kommentarer fra din backend til brugerdefinerede integrationer.
  - Platformen bruger som standard denne, hvis `contextUserId` ikke angives.

[inline-code-attrs-start title = 'Kommentarer - forudberegnet paginering'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&page=0&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR'
[inline-code-end]

[inline-code-attrs-start title = 'Kommentarer - fleksibel paginering'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10'
[inline-code-end]

[inline-code-attrs-start title = 'Kommentarer - fleksibel paginering i brugerkontekst'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10&contextUserId=my-user-id'
[inline-code-end]

[inline-code-attrs-start title = 'Kommentarer - fleksibel paginering i brugerkontekst, kun topniveau-kommentarer'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10&contextUserId=my-user-id&parentId=null'
[inline-code-end]

### Hent kommentarer som et træ

Det er muligt at få kommentarerne returneret som et træ, hvor pagineringen kun tæller topniveau-kommentarerne.

[inline-code-attrs-start title = 'Kommentarer som træ i brugerkontekst'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10&contextUserId=my-user-id&parentId=null&asTree=true'
[inline-code-end]

Vil du kun have topniveau-kommentarerne og de umiddelbare underkommentarer? Her er én måde:

[inline-code-attrs-start title = 'Kommentarer som træ med maksimal dybde'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10&contextUserId=my-user-id&parentId=null&asTree=true&maxTreeDepth=1&limitChildren=10'
[inline-code-end]

I din UI kan du dog have brug for at vide, om der skal vises en "vis svar"-knap på
hver kommentar. Når kommentarer hentes som et træ, er der en `hasChildren`-egenskab,
der tilføjes kommentarerne, når det er relevant.

### Hent kommentarer som et træ, søgning efter hashtag

Det er muligt at søge efter hashtag via API'en på tværs af hele din tenant (ikke begrænset til én side eller `urlId`).

I dette eksempel udelader vi `urlId` og søger efter flere hashtags. API'en returnerer kun kommentarer, der har alle de anmodede hashtags.

[inline-code-attrs-start title = 'Kommentarer som træ i brugerkontekst, efter hashtag'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10&contextUserId=my-user-id&parentId=null&asTree=true&hashTag=TestTag&hashTag=OtherTestTag'
[inline-code-end]

### Alle request-parametre

[inline-code-attrs-start title = 'Struktur for Comments-request'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

[inline-code-attrs-start title = 'Struktur for Comments-response'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

### Nyttige tips

#### URL ID

Du vil sandsynligvis bruge `Comment`-API'en med `urlId`-parameteren. Du kan først kalde `Pages`-API'en for at se, hvordan de tilgængelige `urlId`-værdier ser ud.

#### Anonyme handlinger

Ved anonym kommentering bør du sandsynligvis sende `anonUserId`, når du henter kommentarer, og når du udfører flagning og blokering.

(!) Dette er påkrævet for mange app stores, da brugerne skal kunne flagge brugerskabt indhold, de kan se, selv hvis de ikke er logget ind. Hvis du ikke gør det, kan din app blive fjernet fra den pågældende store.

#### Kommentarer, der ikke returneres

Tjek, at dine kommentarer er godkendt og ikke er spam.
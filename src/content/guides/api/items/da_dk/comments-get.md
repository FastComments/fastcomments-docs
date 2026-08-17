[api-resource-header-start name = 'Comment'; route = 'GET /api/v1/comments'; creditsCost = 1; api-resource-header-end]

Dette API bruges til at hente kommentarer til visning for en bruger. For eksempel filtrerer det automatisk uautoriserede eller spam‑kommentarer.

### Pagination

Pagination kan udføres på en af to måder, afhængigt af ydelseskrav og brugssag:

1. **Hurtigst:** **Precalculated Pagination**:
   1. Dette er hvordan FastComments fungerer, når du bruger vores forudbyggede widgets og klienter.
   2. Klik på "next" øger blot sidetællingen.
   3. Du kan tænke på dette som at blive hentet fra en nøgle‑værdilager.
   4. På denne måde definerer du blot en `page`‑parameter, der starter på `0`, og en sorteringsretning som `direction`.
   5. Sidestørrelser kan tilpasses via tilpasningsregler.
2. **Mest fleksibel:** **Flexible Pagination**:
   1. På denne måde kan du definere brugerdefinerede `limit`‑ og `skip`‑parametre. Send ikke `page`.
   2. Sorterings`direction` understøttes også.
   3. `limit` er det samlede antal, der skal returneres efter at `skip` er anvendt.
      - Eksempel: sæt `skip = 200, limit = 100` når `page size = 100` og `page = 2`.
   4. Børnekommentarer tæller stadig med i pagineringen. Du kan omgå dette ved at bruge `asTree`‑optionen.
      - Du kan paginere børn via `limitChildren` og `skipChildren`.
      - Du kan begrænse dybden af de returnerede tråde via `maxTreeDepth`.

### Threads

1. Når du bruger `Precalculated Pagination`, grupperes kommentarer efter *side*, og kommentarer i tråde påvirker den samlede side.
   1. På denne måde kan tråde bestemmes på klienten baseret på `parentId`.
   2. For eksempel, med en side med én top‑niveau kommentar og 29 svar, og ved at sætte `page=0` i API‑et – får du kun top‑niveau kommentaren og de 29 underkommentarer.
2. Når du bruger `Flexible Pagination`, kan du definere en `parentId`‑parameter.
   1. Sæt denne til null for kun at hente top‑niveau kommentarer.
   2. Herefter, for at se tråde, kald API‑et igen og send `parentId`.
   3. En almindelig løsning er at foretage et API‑kald for top‑niveau kommentarer og derefter foretage parallelle API‑kald for at hente kommentarer til hvert barns kommentarer.
3. __NYT Fra februar 2023!__ Hent som et træ ved at bruge `&asTree=true`.
   1. Du kan tænke på dette som `Flexible Pagination som et træ`.
   2. Kun top‑niveau kommentarer tæller i pagineringen.
   3. Sæt `parentId=null` for at starte træet ved roden (du skal sætte `parentId`).
   4. Sæt `skip` og `limit` for paginering.
   5. Sæt `asTree` til `true`.
   6. Kreditomkostningen stiger med `2x`, da vores backend skal udføre meget mere arbejde i dette scenarie.
   7. Sæt `maxTreeDepth`, `limitChildren` og `skipChildren` efter ønske.

### Trees Explained

Når du bruger `asTree`, kan det være svært at forstå pagineringen. Her er en praktisk grafik:

<div class="screenshot white-bg">
    <div class="title">Diagram for træpaginering</div>
    <img class="screenshot-image" src="/images/fastcomments-comments-api-tree.png" alt="Diagram for træpaginering" />
</div>

### Fetching Comments in The Context of a User

`/comments` API'et kan bruges i to kontekster, til forskellige brugssager:

- For at returnere kommentarer sorteret og mærket med information til at bygge din egen klient.
  - I dette tilfælde definer en `contextUserId`‑forespørgselsparameter.
- For at hente kommentarer fra din backend til tilpassede integrationer.
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

[inline-code-attrs-start title = 'Kommentarer Fleksibel Paginering i Bruger‑Kontekst'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10&contextUserId=my-user-id'
[inline-code-end]

[inline-code-attrs-start title = 'Kommentarer Fleksibel Paginering i Bruger‑Kontekst kun for Top‑Niveau Kommentarer'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10&contextUserId=my-user-id&parentId=null'
[inline-code-end]

### Get Comments as a Tree

Det er muligt at få kommentarerne returneret som et træ, hvor pagineringen kun tæller top‑niveau kommentarer.

[inline-code-attrs-start title = 'Kommentarer Som‑et‑Træ i Bruger‑Kontekst'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10&contextUserId=my-user-id&parentId=null&asTree=true'
[inline-code-end]

Vil du kun hente top‑niveau kommentarer og de umiddelbare børn? Her er en måde:

[inline-code-attrs-start title = 'Kommentarer Som‑et‑Træ med Maksimal Dybde'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10&contextUserId=my-user-id&parentId=null&asTree=true&maxTreeDepth=1&limitChildren=10'
[inline-code-end]

Dog kan det i din UI være nødvendigt at vide, om der skal vises en "vis svar"-knap på hver kommentar. Når du henter kommentarer via et træ, er der en `hasChildren`‑egenskab mærket på kommentarer, når det er relevant.

### Get Comments as a Tree, Searching by Hash Tag

Det er muligt at søge efter hashtag ved hjælp af API'et, på tværs af hele din lejer (ikke begrænset til én side eller `urlId`).

I dette eksempel udelader vi `urlId`, og vi søger efter flere hashtags. API'et vil kun returnere kommentarer, der har alle de anmodede hashtags.

[inline-code-attrs-start title = 'Kommentarer Som‑et‑Træ i Bruger‑Kontekst, efter Hashtag'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10&contextUserId=my-user-id&parentId=null&asTree=true&hashTag=TestTag&hashTag=OtherTestTag'
[inline-code-end]

### All Request Params

[inline-code-attrs-start title = 'Kommentarer Anmodningsstruktur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface CommentsRequestQueryParams {
    tenantId: string
    API_KEY: string
    /** URL‑ID’en (side‑URL eller artikel‑ID), som kommentarerne er knyttet til. **/
    urlId?: string
    /** Begræns de kommentarer, der returneres af denne bruger. **/
    userId?: string
    /** Brug dette til at søge efter hashtag. For at grave ned i krydsfeltet af flere hashtags, brug &hashTag=a&hashTag=b. **/
    hashTag?: string
    /** Sorteringsretningen. Standard er MR (Mest relevant). Andre muligheder er OF (Ældste først) og NF (Nyeste først). **/
    direction?: 'MR' | 'OF' | 'NF'
    /** Forudberegnet paginering: Siden der skal hentes, startende med 0. Angiv -1 for alle kommentarer (op til 250). **/
    page?: number
    /** Fleksibel paginering: Hvor mange kommentarer skal vi returnere? **/
    limit?: number
    /** Fleksibel paginering: Hvor mange underkommentarer skal vi returnere for hver forælder? **/
    limitChildren?: number
    /** Fleksibel paginering: Hvor mange kommentarer skal vi springe over? **/
    skip?: number
    /** Fleksibel paginering: Hvor mange underkommentarer skal vi springe over for hver forælder? **/
    skipChildren?: number
    /** Til bestemmelse af blokerede og flaggede kommentarer. **/
    contextUserId?: string
    /** Til bestemmelse af blokerede og flaggede kommentarer. **/
    anonUserId?: string
    /** Til hentning af underkommentarer. **/
    parentId?: string
    /** Til hentning som et træ. **/
    asTree?: boolean
    /** Hvor dybt i træet skal vi returnere data? 0 returnerer ingen børn. 1 returnerer umiddelbare børn osv. **/
    maxTreeDepth?: number
}
[inline-code-end]

### The Response

[inline-code-attrs-start title = 'Kommentarer Svarstruktur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface CommentsResponse {
    status: 'success' | 'failed'
    /** Inkluderet ved fejl. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-url-id' | 'missing-date' | 'unauthorized-page' | 'invalid-pagination-request' | 'invalid-limit' | 'invalid-limit-children' | 'invalid-skip' | 'invalid-skip-children' | 'invalid-max-tree-depth'
    /** Inkluderet ved fejl. **/
    reason?: string
    /** Kommentarerne! **/
    comments: Comment[]
}
[inline-code-end]

### Helpful Tips

#### URL ID

Du vil sandsynligvis bruge `Comment`‑API'et med `urlId`‑parameteren. Du kan først kalde `Pages`‑API'et for at se, hvordan de tilgængelige `urlId`‑værdier ser ud.

#### Anonymous Actions

For anonym kommentarering vil du sandsynligvis sende `anonUserId`, når du henter kommentarer, og når du udfører flagning og blokering.

(!) Dette er påkrævet for mange app‑butikker, da brugere skal kunne flagge bruger‑oprettet indhold, de kan se, selvom de ikke er logget ind. Undladelse kan medføre, at din app fjernes fra den pågældende butik.

#### Comments Not Being Returned

Tjek at dine kommentarer er godkendt, og ikke er spam.

---
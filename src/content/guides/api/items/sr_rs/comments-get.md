[api-resource-header-start name = 'Comment'; route = 'GET /api/v1/comments'; creditsCost = 1; api-resource-header-end]

Ovaj API se koristi za dobavljanje komentara za prikaz korisniku. Na primer, automatski filtrira neodobrene ili spam komentare.

### Pagination

Paginacija se može izvesti na jedan od dva načina, u zavisnosti od zahteva za performansama i slučaja upotrebe:

1. **Najbrže:** **Prekalkulisana Paginacija**:
   1. Ovo je način na koji FastComments funkcioniše kada koristite naše unapred izgrađene vidžete i klijente.
   2. Klikom na „next“ (sledeće) jednostavno se povećava broj stranice.
   3. Ovo možete zamisliti kao preuzimanje iz skladišta ključ‑vrednost.
   4. Na ovaj način, jednostavno definišite parametar `page` počevši od `0` i smer sortiranja kao `direction`.
   5. Veličine stranica mogu se prilagoditi putem pravila prilagođavanja.
2. **Najfleksibilnije:** **Fleksibilna Paginacija**:
   1. Na ovaj način možete definisati prilagođene parametre `limit` i `skip`. Ne prosleđujte `page`.
   2. Sortiranje `direction` je takođe podržano.
   3. `limit` je ukupan broj koji se vraća nakon što se primeni `skip`.
      - Primer: postavite `skip = 200, limit = 100` kada je `page size = 100` i `page = 2`.
   4. Podkomentari i dalje broje u paginaciji. Možete zaobići ovo korišćenjem opcije `asTree`.
      - Možete paginirati podkomentare putem `limitChildren` i `skipChildren`.
      - Možete ograničiti dubinu vraćenih niti putem `maxTreeDepth`.

### Threads

1. Kada se koristi `Prekalkulisana Paginacija`, komentari su grupisani po *stranici* i komentari u nitima utiču na ukupnu stranicu.
   1. Na ovaj način, niti se mogu odrediti na klijentu na osnovu `parentId`.
   2. Na primer, sa stranicom koja ima jedan komentar najvišeg nivoa i 29 odgovora, i postavljanjem `page=0` u API‑u – dobićete samo komentar najvišeg nivoa i 29 podkomentara.
2. Kada se koristi `Fleksibilna Paginacija`, možete definisati parametar `parentId`.
   1. Postavite ga na null da biste dobili samo komentare najvišeg nivoa.
   2. Zatim, da biste videli niti, ponovo pozovite API i prosledite `parentId`.
   3. Uobičajeno rešenje je da napravite API poziv za komentare najvišeg nivoa, a zatim paralelne API pozive da dobijete komentare za podkomentare svakog komentara.
3. __NOVO od februara 2023!__ Dohvatite kao stablo koristeći `&asTree=true`.
   1. Ovo možete zamisliti kao `Fleksibilna Paginacija kao Stablo`.
   2. Samo komentari najvišeg nivoa broje u paginaciji.
   3. Postavite `parentId=null` da započnete stablo od korena (morate postaviti `parentId`).
   4. Postavite `skip` i `limit` za paginaciju.
   5. Postavite `asTree` na `true`.
   6. Trošak kredita se povećava za `2x`, jer naš backend mora da uradi mnogo više posla u ovom scenariju.
   7. Postavite `maxTreeDepth`, `limitChildren` i `skipChildren` po želji.

### Trees Explained

Kada se koristi `asTree`, može biti teško razumeti paginaciju. Evo praktične grafike:

<div class="screenshot white-bg">
    <div class="title">Tree Pagination Diagram</div>
    <img class="screenshot-image" src="/images/fastcomments-comments-api-tree.png" alt="Tree Pagination Diagram" />
</div>

### Fetching Comments in The Context of a User

API `/comments` može se koristiti u dva konteksta, za različite slučajeve upotrebe:

- Za vraćanje komentara sortiranih i označenih informacijama za izgradnju vašeg sopstvenog klijenta.
  - U ovom slučaju, definišite upitni parametar `contextUserId`.
- Za dohvat komentara iz vašeg backend‑a za prilagođene integracije.
  - Platforma će podrazumevano koristiti ovo bez `contextUserId`. 

[inline-code-attrs-start title = 'Komentari Prekalkulisana Paginacija'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&page=0&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR'
[inline-code-end]

[inline-code-attrs-start title = 'Komentari Fleksibilna Paginacija'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10'
[inline-code-end]

[inline-code-attrs-start title = 'Komentari Fleksibilna Paginacija u Korisničkom Kontekstu'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10&contextUserId=my-user-id'
[inline-code-end]

[inline-code-attrs-start title = 'Komentari Fleksibilna Paginacija u Korisničkom Kontekstu samo za Komentare Najvišeg Nivoa'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10&contextUserId=my-user-id&parentId=null'
[inline-code-end]

### Get Comments as a Tree

Moguće je dobiti komentare vraćene kao stablo, pri čemu paginacija broji samo komentare najvišeg nivoa.

[inline-code-attrs-start title = 'Komentari Kao-Stablo u Korisničkom Kontekstu'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10&contextUserId=my-user-id&parentId=null&asTree=true'
[inline-code-end]

Želite li da dobijete samo komentare najvišeg nivoa i neposredne podkomentare? Evo jednog načina:

[inline-code-attrs-start title = 'Komentari Kao-Stablo sa Maksimalnom Dubinom'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10&contextUserId=my-user-id&parentId=null&asTree=true&maxTreeDepth=1&limitChildren=10'
[inline-code-end]

Međutim, u vašem UI‑ju možda ćete morati da znate da li da prikažete dugme „prikaži odgovore“ na svakom komentaru. Kada se komentari dohvaćaju putem stabla, na komentare se dodaje svojstvo `hasChildren` kada je primenljivo.

### Get Comments as a Tree, Searching by Hash Tag

Moguće je pretraživati po heš tagu koristeći API, kroz ceo vaš tenant (nije ograničeno na jednu stranicu ili `urlId`).

U ovom primeru izostavljamo `urlId` i pretražujemo po više heš tagova. API će vratiti samo komentare koji imaju sve tražene heš tagove.

[inline-code-attrs-start title = 'Komentari Kao-Stablo u Korisničkom Kontekstu, po Heš Tagu'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10&contextUserId=my-user-id&parentId=null&asTree=true&hashTag=TestTag&hashTag=OtherTestTag'
[inline-code-end]

### All Request Params

[inline-code-attrs-start title = 'Struktura Zahteva za Komentare'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

[inline-code-attrs-start title = 'Struktura Odgovora za Komentare'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

Verovatno želite da koristite `Comment` API sa parametrom `urlId`. Možete prvo pozvati `Pages` API da vidite kako izgledaju dostupne vrednosti `urlId`.

#### Anonymous Actions

Za anonimno komentarisanje verovatno želite da prosledite `anonUserId` prilikom dohvaćanja komentara, kao i prilikom označavanja i blokiranja.

(!) Ovo je obavezno za mnoge prodavnice aplikacija jer korisnici moraju moći da označe sadržaj koji su kreirali drugi korisnici, čak i ako nisu prijavljeni. Nepoštovanje može dovesti do uklanjanja vaše aplikacije iz te prodavnice.

#### Comments Not Being Returned

Proverite da li su vaši komentari odobreni i da nisu spam.

---
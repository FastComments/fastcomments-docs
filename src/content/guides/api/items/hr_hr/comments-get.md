[api-resource-header-start name = 'Comment'; route = 'GET /api/v1/comments'; creditsCost = 1; api-resource-header-end]

Ovaj API se koristi za dohvaćanje komentara za prikaz korisniku. Na primjer, automatski filtrira neodobrene ili spam komentare.

### Pagiranje

Pagiranje se može izvesti na jedan od dva načina, ovisno o zahtjevima za performansama i slučaju upotrebe:

1. Najbrže: **Precalculated Pagination**:
   1. Ovo je način na koji FastComments radi kada koristite naše unaprijed izgrađene widgete i klijente.
   2. Klikom na „next“ jednostavno se povećava broj stranice.
   3. Možete to zamisliti kao dohvaćanje iz key-value pohrane.
   4. Na ovaj način, jednostavno definirajte parametar `page` koji počinje od `0` i smjer sortiranja kao `direction`.
   5. Veličine stranica mogu se prilagoditi putem pravila prilagodbe.
2. Najfleksibilnije: **Flexible Pagination**:
   1. Na ovaj način možete definirati prilagođene parametre `limit` i `skip`. Nemojte proslijediti `page`.
   2. Sortiranje `direction` također je podržano.
   3. `limit` je ukupni broj koji se vraća nakon primjene `skip`.
      - Primjer: postavite `skip = 200, limit = 100` kada je `page size = 100` i `page = 2`.
   4. Podređeni komentari i dalje se računaju u paginaciji. Možete to zaobići korištenjem opcije `asTree`.
      - Možete paginirati podređene putem `limitChildren` i `skipChildren`.
      - Možete ograničiti dubinu vraćenih niti putem `maxTreeDepth`.

### Niti

1. Pri korištenju `Precalculated Pagination`, komentari su grupirani po *page* i komentari u nitima utječu na cijelu stranicu.
   1. Na ovaj način, niti se mogu odrediti na klijentu na temelju `parentId`.
   2. Na primjer, s stranicom koja ima jedan komentar na najvišoj razini i 29 odgovora, i postavljanjem `page=0` u API - dobit ćete samo komentar na najvišoj razini i 29 podređenih.
2. Pri korištenju `Flexible Pagination`, možete definirati parametar `parentId`.
   1. Postavite ga na null da biste dobili samo komentare na najvišoj razini.
   2. Zatim, za pregled niti, ponovno pozovite API i proslijedite `parentId`.
   3. Uobičajeno rješenje je napraviti API poziv za komentare na najvišoj razini, a zatim paralelne API pozive za dobivanje komentara za podređene svakog komentara.
3. __NOVO Od veljače 2023!__ Dohvatite kao stablo koristeći `&asTree=true`.
   1. Možete to zamisliti kao `Flexible Pagination as a Tree`.
   2. Samo komentari na najvišoj razini se računaju u paginaciji.
   3. Postavite `parentId=null` da započnete stablo na korijenu (morate postaviti `parentId`).
   4. Postavite `skip` i `limit` za paginaciju.
   5. Postavite `asTree` na `true`.
   6. Cijena u kreditima povećava se za `2x`, jer naš backend mora obaviti puno više posla u ovom scenariju.
   7. Postavite `maxTreeDepth`, `limitChildren` i `skipChildren` prema želji.

### Objašnjenje stabala

Kada se koristi `asTree`, može biti teško razmišljati o paginaciji. Evo praktičnog grafikona:

<div class="screenshot white-bg">
    <div class="title">Dijagram paginacije stabla</div>
    <img class="screenshot-image" src="/images/fastcomments-comments-api-tree.png" alt="Dijagram paginacije stabla" />
</div>

### Dohvaćanje komentara u kontekstu korisnika

API `/comments` može se koristiti u dva konteksta, za različite slučajeve upotrebe:

- Za vraćanje komentara sortiranih i označenih informacijama za izgradnju vlastitog klijenta.
  - U tom slučaju, definirajte upitni parametar `contextUserId`.
- Za dohvaćanje komentara iz vašeg backend-a za prilagođene integracije.
  - Platforma će prema zadanim postavkama koristiti ovo bez `contextUserId`. 

[inline-code-attrs-start title = 'Komentari Precalculated Pagination'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&page=0&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR'
[inline-code-end]

[inline-code-attrs-start title = 'Komentari Flexible Pagination'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10'
[inline-code-end]

[inline-code-attrs-start title = 'Komentari Flexible Pagination u kontekstu korisnika'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10&contextUserId=my-user-id'
[inline-code-end]

[inline-code-attrs-start title = 'Komentari Flexible Pagination u kontekstu korisnika samo za komentare najviše razine'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10&contextUserId=my-user-id&parentId=null'
[inline-code-end]

### Dohvaćanje komentara kao stablo

Moguće je dobiti komentare vraćene kao stablo, s paginacijom koja broji samo komentare najviše razine.

[inline-code-attrs-start title = 'Komentari As-A-Tree u kontekstu korisnika'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10&contextUserId=my-user-id&parentId=null&asTree=true'
[inline-code-end]

Želite li dobiti samo komentare najviše razine i neposredne podređene? Evo jednog načina:

[inline-code-attrs-start title = 'Komentari As-A-Tree s maksimalnom dubinom'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10&contextUserId=my-user-id&parentId=null&asTree=true&maxTreeDepth=1&limitChildren=10'
[inline-code-end]

Međutim, u vašem UI-ju možda ćete trebati znati treba li prikazati gumb „prikaži odgovore“ na svakom komentaru. Kada dohvaćate komentare putem stabla, postoji svojstvo `hasChildren` koje je označeno na komentarima kada je primjenjivo.

### Dohvaćanje komentara kao stablo, pretraživanje po hashtag-u

Moguće je pretraživati po hashtag-u koristeći API, kroz cijeli vaš tenant (nije ograničeno na jednu stranicu ili `urlId`).

U ovom primjeru izostavljamo `urlId` i pretražujemo po više hashtagova. API će vratiti samo komentare koji imaju sve tražene hashtagove.

[inline-code-attrs-start title = 'Komentari As-A-Tree u kontekstu korisnika, po hashtag-u'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10&contextUserId=my-user-id&parentId=null&asTree=true&hashTag=TestTag&hashTag=OtherTestTag'
[inline-code-end]

### Svi parametri zahtjeva

[inline-code-attrs-start title = 'Struktura zahtjeva za komentare'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

### Odgovor

[inline-code-attrs-start title = 'Struktura odgovora za komentare'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

### Korisni savjeti

#### URL ID

Vjerojatno želite koristiti API `Comment` s parametrom `urlId`. Možete najprije pozvati API `Pages` da vidite kako izgledaju dostupne vrijednosti `urlId`.

#### Anonimne radnje

Za anonimno komentiranje vjerojatno želite proslijediti `anonUserId` prilikom dohvaćanja komentara i prilikom označavanja i blokiranja.

(!) Ovo je potrebno za mnoge trgovine aplikacija jer korisnici moraju moći označiti sadržaj koji su kreirali drugi korisnici, čak i ako nisu prijavljeni. Nepoštivanje može uzrokovati uklanjanje vaše aplikacije iz te trgovine.

#### Komentari se ne vraćaju

Provjerite da su vaši komentari odobreni i da nisu spam.

---
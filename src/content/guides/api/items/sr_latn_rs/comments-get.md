[api-resource-header-start name = 'Comment'; route = 'GET /api/v1/comments'; creditsCost = 1; api-resource-header-end]

Ovaj API se koristi za preuzimanje komentara za prikaz korisniku. Na primer, automatski filtrira
neodobrene komentare i spam.

### Paginacija

Paginacija se može izvršiti na jedan od dva načina, u zavisnosti od zahteva za performansama i slučaja upotrebe:

1. Najbrže: **Prethodno izračunata paginacija**:
   1. tako FastComments radi kada koristite naše gotove vidžete i klijente.
   2. Klik na "next" jednostavno povećava broj stranice.
   3. Možete ovo zamisliti kao preuzimanje iz key-value skladišta.
   4. Na ovaj način, jednostavno definišite parametar `page` počevši od `0` i smer sortiranja kao `direction`.
   5. Veličine stranica mogu se prilagoditi pravilima prilagođavanja.
2. Najfleksibilnije: **Fleksibilna paginacija**:
   1. Na ovaj način možete definisati prilagođene parametre `limit` i `skip`. Ne prosleđujte `page`.
   2. Podržan je i smer sortiranja `direction`.
   3. `limit` je ukupan broj koji se vraća nakon primene `skip`.
      - Primer: postavite `skip = 200, limit = 100` kada je `page size = 100` i `page = 2`.
   4. Podređeni komentari se i dalje računaju u paginaciju. Ovo možete zaobići pomoću opcije `asTree`.
      - Podređene komentare možete paginirati pomoću `limitChildren` i `skipChildren`.
      - Dubinu vraćenih niti možete ograničiti pomoću `maxTreeDepth`.

### Niti (Threads)

1. Kada koristite `Precalculated Pagination`, komentari su grupisani po *stranici* i komentari u nitima utiču na ukupnu stranicu.
   1. Na ovaj način, niti se mogu odrediti na klijentu na osnovu `parentId`.
   2. Na primer, sa stranicom koja ima jedan komentar najvišeg nivoa i 29 odgovora, i postavljanjem `page=0` u API-ju - dobićete samo komentar najvišeg nivoa i 29 podređenih komentara.
2. Kada koristite `Flexible Pagination`, možete definisati parametar `parentId`.
   1. Postavite ovo na null da biste dobili samo komentare najvišeg nivoa.
   2. Zatim, da biste pregledali niti, ponovo pozovite API i prosledite `parentId`.
   3. Uobičajeno rešenje je da se napravi API poziv za komentare najvišeg nivoa, a zatim paralelni API pozivi za preuzimanje komentara podređenih elemenata svakog komentara.
3. __NOVO od Feb 2023!__ Preuzimanje kao stablo korišćenjem `&asTree=true`.
   1. Ovo možete zamisliti kao `Flexible Pagination as a Tree`.
   2. Samo komentari najvišeg nivoa se računaju u paginaciju.
   3. Postavite `parentId=null` da stablo počne od korena (morate postaviti `parentId`).
   4. Postavite `skip` i `limit` za paginaciju.
   5. Postavite `asTree` na `true`.
   6. Cena u kreditima se povećava `2x`, jer naš backend u ovom scenariju mora da uradi mnogo više posla.
   7. Postavite `maxTreeDepth`, `limitChildren` i `skipChildren` po želji.

### Objašnjenje stabala

Kada koristite `asTree`, paginacija može biti teška za razumevanje. Evo korisne grafike:

<div class="screenshot white-bg">
    <div class="title">Dijagram paginacije stabla</div>
    <img class="screenshot-image" src="/images/fastcomments-comments-api-tree.png" alt="Dijagram paginacije stabla" />
</div>

### Preuzimanje komentara u kontekstu korisnika

API `/comments` može se koristiti u dva konteksta, za različite slučajeve upotrebe:

- Za vraćanje komentara sortiranih i označenih informacijama za izgradnju sopstvenog klijenta.
  - U tom slučaju, definišite query parametar `contextUserId`.
- Za preuzimanje komentara sa vašeg backend-a za prilagođene integracije.
  - Platforma će podrazumevano koristiti ovo bez `contextUserId`.

[inline-code-attrs-start title = 'Prethodno izračunata paginacija komentara'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&page=0&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR'
[inline-code-end]

[inline-code-attrs-start title = 'Fleksibilna paginacija komentara'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10'
[inline-code-end]

[inline-code-attrs-start title = 'Fleksibilna paginacija komentara u kontekstu korisnika'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10&contextUserId=my-user-id'
[inline-code-end]

[inline-code-attrs-start title = 'Fleksibilna paginacija komentara u kontekstu korisnika, samo komentari najvišeg nivoa'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10&contextUserId=my-user-id&parentId=null'
[inline-code-end]

### Preuzimanje komentara kao stablo

Moguće je dobiti komentare vraćene kao stablo, gde paginacija računa samo komentare najvišeg nivoa.

[inline-code-attrs-start title = 'Komentari kao stablo u kontekstu korisnika'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10&contextUserId=my-user-id&parentId=null&asTree=true'
[inline-code-end]

Želite samo komentare najvišeg nivoa i neposredne podređene komentare? Evo jednog načina:

[inline-code-attrs-start title = 'Komentari kao stablo sa maksimalnom dubinom'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10&contextUserId=my-user-id&parentId=null&asTree=true&maxTreeDepth=1&limitChildren=10'
[inline-code-end]

Međutim, u vašem UI-ju možda treba da znate da li prikazati dugme "prikaži odgovore" na
svakom komentaru. Prilikom preuzimanja komentara kao stabla postoji svojstvo `hasChildren` koje se
dodaje komentarima kada je primenljivo.

### Preuzimanje komentara kao stablo, pretraga po heštegu

Moguće je pretraživati po heštegu korišćenjem API-ja, u okviru celog vašeg tenanta (ne ograničeno na jednu stranicu ili `urlId`).

U ovom primeru izostavljamo `urlId` i pretražujemo po više heštegova. API će vratiti samo komentare koji imaju sve tražene heštegove.

[inline-code-attrs-start title = 'Komentari kao stablo u kontekstu korisnika, po heštegu'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10&contextUserId=my-user-id&parentId=null&asTree=true&hashTag=TestTag&hashTag=OtherTestTag'
[inline-code-end]

### Svi parametri zahteva

[inline-code-attrs-start title = 'Struktura zahteva za komentare'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

### Korisni saveti

#### URL ID

Verovatno želite da koristite `Comment` API sa parametrom `urlId`. Prvo možete pozvati `Pages` API da vidite kako izgledaju dostupne vrednosti `urlId`.

#### Anonimne akcije

Za anonimno komentiranje verovatno želite da prosledite `anonUserId` prilikom preuzimanja komentara, kao i prilikom prijavljivanja i blokiranja.

(!) Ovo je obavezno za mnoge prodavnice aplikacija, jer korisnici moraju moći da prijave sadržaj koji su kreirali korisnici a koji mogu da vide, čak i ako nisu prijavljeni. Nepoštovanje ovoga može dovesti do uklanjanja vaše aplikacije iz date prodavnice.

#### Komentari se ne vraćaju

Proverite da li su vaši komentari odobreni i da nisu spam.
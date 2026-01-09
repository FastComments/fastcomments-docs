[api-resource-header-start name = 'Comment'; route = 'GET /api/v1/comments'; creditsCost = 1; api-resource-header-end]

Ovaj API se koristi za dobijanje komentara za prikaz korisniku. Na primjer, automatski filtrira neodobrene ili spam komentare.

### Paginacija

Paginacija se može vršiti na jedan od dva načina, u zavisnosti od zahtjeva za performansama i slučaja upotrebe:

1. Najbrže: **Prekalkulisana paginacija**:
   1. Ovako FastComments radi kada koristite naše prethodno izgrađene widgete i klijente.
   2. Klik na 'next' jednostavno povećava broj stranice.
   3. Možete to zamisliti kao da se dobija iz skladišta ključ-vrijednost.
   4. Na ovaj način, jednostavno definišite parametar `page` koji počinje od `0` i smjer sortiranja kao `direction`.
   5. Veličine stranica se mogu prilagoditi pravilima za prilagođavanje.
2. Najfleksibilnije: **Fleksibilna paginacija**:
   1. Na ovaj način možete definirati prilagođene parametre `limit` i `skip`. Nemojte slati `page`.
   2. Podržan je i smjer sortiranja `direction`.
   3. `limit` je ukupan broj koji će biti vraćen nakon što se primijeni `skip`.
      - Primjer: postavite `skip = 200, limit = 100` kada je `page size = 100` i `page = 2`.
   4. Podkomentari i dalje se računaju u paginaciji. Možete to izbjeći koristeći opciju `asTree`.
      - Podkomentare možete paginirati preko `limitChildren` i `skipChildren`.
      - Možete ograničiti dubinu niti koje se vraćaju putem `maxTreeDepth`.

### Niti

1. Kada koristite `Precalculated Pagination`, komentari su grupisani po *stranici* i komentari u nitima utiču na ukupnu stranicu.
   1. Na ovaj način, niti se mogu odrediti na klijentu na osnovu `parentId`.
   2. Na primjer, na stranici sa jednim komentarom najvišeg nivoa i 29 odgovora, ako postavite `page=0` u API-ju — dobićete samo komentar najvišeg nivoa i 29 djece.
   3. [Primjer slike koja ilustruje više stranica.](https://blog.winricklabs.com/images/fc-pagination02.png)
2. Kada koristite `Flexible Pagination`, možete definirati parametar `parentId`.
   1. Postavite ovo na null da dobijete samo komentare najvišeg nivoa.
   2. Zatim, da biste vidjeli niti, pozovite API ponovo i prosledite `parentId`.
   3. Uobičajeno rješenje je napraviti API poziv za komentare najvišeg nivoa, a zatim napraviti paralelne API pozive za dobijanje komentara za podkomentare svakog komentara.
3. __NOVO od Feb 2023!__ Dohvatite kao stablo koristeći `&asTree=true`.
   1. Možete to zamisliti kao `Flexible Pagination as a Tree`.
   2. Samo komentari najvišeg nivoa se računaju u paginaciji.
   3. Postavite `parentId=null` da započnete stablo od korijena (morate postaviti `parentId`).
   4. Postavite `skip` i `limit` za paginaciju.
   5. Postavite `asTree` na `true`.
   6. Trošak kredita se udvostručuje (`2x`), jer naš backend mora obaviti mnogo više posla u ovom scenariju.
   7. Postavite `maxTreeDepth`, `limitChildren` i `skipChildren` po želji.

### Objašnjenje stabala

Kada koristite `asTree`, može biti teško razumjeti paginaciju. Evo korisnog grafikona:

<div class="screenshot white-bg">
    <div class="title">Tree Pagination Diagram</div>
    <img class="screenshot-image" src="/images/fastcomments-comments-api-tree.png" alt="Tree Pagination Diagram" />
</div>

### Dohvatanje komentara u kontekstu korisnika

API `/comments` se može koristiti u dva konteksta, za različite slučajeve upotrebe:

- Za vraćanje komentara sortirano i označeno informacijama za izgradnju vašeg klijenta.
  - U ovom slučaju, definišite query parametar `contextUserId`.
- Za dobijanje komentara sa vašeg backend-a za prilagođene integracije.
  - Platforma će koristiti ovo kao zadato ako nema `contextUserId`. 

[inline-code-attrs-start title = 'Prekalkulisana paginacija komentara'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
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

[inline-code-attrs-start title = 'Fleksibilna paginacija komentara u kontekstu korisnika (samo komentari najvišeg nivoa)'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10&contextUserId=my-user-id&parentId=null'
[inline-code-end]

### Dohvati komentare kao stablo

Moguće je dobiti komentare vraćene kao stablo, pri čemu paginacija broji samo komentare najvišeg nivoa.

[inline-code-attrs-start title = 'Komentari kao stablo u kontekstu korisnika'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10&contextUserId=my-user-id&parentId=null&asTree=true'
[inline-code-end]

Želite da dobijete samo komentare najvišeg nivoa i neposredne odgovore? Evo jednog načina:

[inline-code-attrs-start title = 'Komentari kao stablo sa maksimalnom dubinom'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10&contextUserId=my-user-id&parentId=null&asTree=true&maxTreeDepth=1&limitChildren=10'
[inline-code-end]

Međutim, u vašem UI možda treba da znate da li da prikažete dugme "show replies" na
svakom komentaru. Kada dohvatate komentare putem stabla postoji svojstvo `hasChildren` koje se dodaje
komentarima kada je primjenjivo.

### Dohvati komentare kao stablo, pretraga po haštagu

Moguće je pretraživati po haštagu koristeći API, kroz cijeli vaš tenant (nije ograničeno na jednu stranicu ili `urlId`).

U ovom primjeru izostavljamo `urlId`, i pretražujemo po više haštagova. API će vratiti samo komentare koji imaju sve tražene haštagove.

[inline-code-attrs-start title = 'Komentari kao stablo u kontekstu korisnika, po haštagu'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
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
    /** ID stranice (url stranice ili ID članka) sa kojom su komentari povezani. **/
    urlId?: string
    /** Ograniči komentare koji se vraćaju na one od ovog korisnika. **/
    userId?: string
    /** Koristite ovo za pretragu po haštagu. Da biste dublje povukli presjek više haštagova, koristite &hashTag=a&hashTag=b. **/
    hashTag?: string
    /** Smjer sortiranja. Zadano je MR (Most Relevant). Ostale opcije su OF (Oldest First) i NF (Newest First). **/
    direction?: 'MR' | 'OF' | 'NF'
    /** Prekalkulisana paginacija: Stranica za dohvat, počevši od 0. Prosledite -1 za sve komentare (do 250). **/
    page?: number
    /** Fleksibilna paginacija: Koliko komentara treba da vratimo? **/
    limit?: number
    /** Fleksibilna paginacija: Koliko podkomentara treba da vratimo za svaki roditelj? **/
    limitChildren?: number
    /** Fleksibilna paginacija: Koliko komentara treba da preskočimo? **/
    skip?: number
    /** Fleksibilna paginacija: Koliko podkomentara treba da preskočimo za svakog roditelja? **/
    skipChildren?: number
    /** Za određivanje blokiranih i flagovanih komentara. **/
    contextUserId?: string
    /** Za određivanje blokiranih i flagovanih komentara. **/
    anonUserId?: string
    /** Za dohvatanje dječijih komentara. **/
    parentId?: string
    /** Za dohvatanje kao stablo. **/
    asTree?: boolean
    /** Koliko duboko u stablu treba da vraćamo podatke? 0 vraća nijedno dijete. 1 vraća neposredna djeca, itd. **/
    maxTreeDepth?: number
}
[inline-code-end]

### Odgovor

[inline-code-attrs-start title = 'Struktura odgovora za komentare'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface CommentsResponse {
    status: 'success' | 'failed'
    /** Uključeno u slučaju greške. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-url-id' | 'missing-date' | 'unauthorized-page' | 'invalid-pagination-request' | 'invalid-limit' | 'invalid-limit-children' | 'invalid-skip' | 'invalid-skip-children' | 'invalid-max-tree-depth'
    /** Uključeno u slučaju greške. **/
    reason?: string
    /** Komentari! **/
    comments: Comment[]
}
[inline-code-end]

### Korisni savjeti

#### URL ID

Vjerovatno ćete htjeti koristiti `Comment` API sa parametrrom `urlId`. Možete prvo pozvati `Pages` API, da vidite kako izgledaju vrijednosti `urlId` koje su vam na raspolaganju. 

#### Anonimne radnje

Za anonimno komentarisanje vjerovatno želite poslati `anonUserId` prilikom dohvatanja komentara, kao i prilikom prijavljivanja i blokiranja.

(!) Ovo je zahtjev za mnoge prodavnice aplikacija jer korisnici moraju biti u mogućnosti da prijave sadržaj kreiran od strane korisnika koji mogu vidjeti, čak i ako nisu prijavljeni. Nepoštovanje ovoga može uzrokovati uklanjanje vaše aplikacije iz navedene prodavnice.

#### Komentari se ne vraćaju

Provjerite da li su vaši komentari odobreni i da nisu spam.
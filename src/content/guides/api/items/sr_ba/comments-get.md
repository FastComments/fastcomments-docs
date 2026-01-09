[api-resource-header-start name = 'Comment'; route = 'GET /api/v1/comments'; creditsCost = 1; api-resource-header-end]

Ovaj API se koristi za dobavljanje komentara za prikaz korisniku. Na primjer, automatski filtrira
neodobrene ili spam komentare.

### Pagination

Paginacija se može obaviti na jedan od dva načina, u zavisnosti od zahtjeva za performansama i slučaja upotrebe:

1. Fastest: **Prekalkulisana paginacija**:
   1. Ovako FastComments radi kada koristite naše unaprijed izgrađene widgete i klijente.
   2. Klik na "next" jednostavno povećava broj stranice.
   3. Možete ovo smatrati dohvaćenim iz key-value skladišta.
   4. Na ovaj način, jednostavno definirajte parametar `page` koji počinje od `0` i smjer sortiranja kao `direction`.
   5. Veličine stranica se mogu prilagoditi putem pravila za prilagodbu.
2. Most Flexible: **Fleksibilna paginacija**:
   1. Na ovaj način možete definirati prilagođene parametre `limit` i `skip`. Ne šaljite `page`.
   2. Smjer sortiranja `direction` je također podržan.
   3. `limit` je ukupan broj koji će biti vraćen nakon što se primijeni `skip`.
      - Primjer: postavite `skip = 200, limit = 100` kada je `page size = 100` i `page = 2`.
   4. Child komentari se i dalje računaju u paginaciji. Možete zaobići ovo korištenjem opcije `asTree`.
      - Možete paginirati djecu putem `limitChildren` i `skipChildren`.
      - Možete ograničiti dubinu threadova koji se vraćaju putem `maxTreeDepth`.

### Threads

1. Kada koristite `Precalculated Pagination`, komentari se grupišu po *page*-u i komentari u threadovima utiču na ukupnu stranicu.
   1. Na ovaj način, threadovi se mogu odrediti na klijentu na osnovu `parentId`.
   2. Na primjer, sa stranicom koja ima jedan komentar najvišeg nivoa i 29 odgovora, i postavljanjem `page=0` u API-ju - dobićete samo komentar najvišeg nivoa i 29 djece.
   3. [Example image here illustrating multiple pages.](https://blog.winricklabs.com/images/fc-pagination02.png)
2. Kada koristite `Flexible Pagination`, možete definirati parametar `parentId`.
   1. Postavite ovo na null da biste dobili samo komentare najvišeg nivoa.
   2. Zatim, da biste vidjeli threadove, pozovite API ponovo i pošaljite `parentId`.
   3. Uobičajeno rješenje je napraviti API poziv za komentare najvišeg nivoa, a zatim paralelne API pozive za dobavljanje komentara djece za svaki komentar.
3. __NOVO (od feb 2023.)!__ Dohvatite kao stablo koristeći `&asTree=true`.
   1. Možete ovo zamisliti kao `Fleksibilna paginacija kao stablo`.
   2. Samo komentari najvišeg nivoa se računaju u paginaciji.
   3. Postavite `parentId=null` da počnete stablo od korijena (morate postaviti `parentId`).
   4. Postavite `skip` i `limit` za paginaciju.
   5. Postavite `asTree` na `true`.
   6. Troškovi kredita se povećavaju za `2x`, jer naš backend mora uraditi mnogo više posla u ovom scenariju.
   7. Postavite `maxTreeDepth`, `limitChildren`, i `skipChildren` po želji.

### Trees Explained

Kada koristite `asTree`, može biti teško razmišljati o paginaciji. Evo korisne grafike:

<div class="screenshot white-bg">
    <div class="title">Dijagram paginacije stabla</div>
    <img class="screenshot-image" src="/images/fastcomments-comments-api-tree.png" alt="Dijagram paginacije stabla" />
</div>

### Fetching Comments in The Context of a User

API `/comments` se može koristiti u dva konteksta, za različite slučajeve upotrebe:

- Za vraćanje komentara sortiranih i označenih informacijama za izgradnju vlastitog klijenta.
  - U tom slučaju, definirajte query parametar `contextUserId`.
- Za dohvat komentara iz vašeg backenda za prilagođene integracije.
  - Platforma će podrazumijevati ovo ako nema `contextUserId`. 

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

[inline-code-attrs-start title = 'Fleksibilna paginacija komentara u korisničkom kontekstu samo za komentare najvišeg nivoa'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10&contextUserId=my-user-id&parentId=null'
[inline-code-end]

### Get Comments as a Tree

Moguće je dobiti komentare vraćene kao stablo, pri čemu paginacija broji samo komentare najvišeg nivoa.

[inline-code-attrs-start title = 'Komentari kao stablo u kontekstu korisnika'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10&contextUserId=my-user-id&parentId=null&asTree=true'
[inline-code-end]

Želite da dobijete samo komentare najvišeg nivoa i neposrednu djecu? Evo jednog načina:

[inline-code-attrs-start title = 'Komentari kao stablo sa maksimalnom dubinom'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10&contextUserId=my-user-id&parentId=null&asTree=true&maxTreeDepth=1&limitChildren=10'
[inline-code-end]

Međutim, u vašem UI možda ćete trebati znati da li da prikažete dugme "show replies" (prikaži odgovore) na
svakom komentaru. Kada dohvaćate komentare putem stabla, postoji svojstvo `hasChildren` koje se dodaje
na komentare kada je primjenjivo.

### Get Comments as a Tree, Searching by Hash Tag

Moguće je pretraživati po hashtag-u koristeći API, preko cijelog vašeg tenant-a (nije ograničeno na jednu stranicu ili `urlId`).

U ovom primjeru, izostavljamo `urlId`, i pretražujemo po više hashtag-ova. API će vratiti samo komentare koji imaju sve tražene hashtag-ove.

[inline-code-attrs-start title = 'Komentari kao stablo u kontekstu korisnika, po hashtag-u'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10&contextUserId=my-user-id&parentId=null&asTree=true&hashTag=TestTag&hashTag=OtherTestTag'
[inline-code-end]

### All Request Params

[inline-code-attrs-start title = 'Struktura zahtjeva za komentare'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface CommentsRequestQueryParams {
    tenantId: string
    API_KEY: string
    /** urlId (URL stranice, ili ID članka) sa kojim su komentari povezani. **/
    urlId?: string
    /** Ograniči komentare vraćene za ovog korisnika. **/
    userId?: string
    /** Koristite ovo za pretragu po hashtag-u. Da biste suzili na presjek više hashtag-ova, uradite &hashTag=a&hashTag=b. **/
    hashTag?: string
    /** Smjer sortiranja. Zadano je MR (Najrelevantnije). Druge opcije su OF (Najstarije prvo) i NF (Najnovije prvo). **/
    direction?: 'MR' | 'OF' | 'NF'
    /** Prekalkulisana paginacija: Stranica koju treba dohvatiti, počevši od 0. Pošaljite -1 za sve komentare (do 250). **/
    page?: number
    /** Fleksibilna paginacija: Koliko komentara da vratimo? **/
    limit?: number
    /** Fleksibilna paginacija: Koliko dječjih komentara da vratimo za svaki parent? **/
    limitChildren?: number
    /** Fleksibilna paginacija: Koliko komentara da preskočimo? **/
    skip?: number
    /** Fleksibilna paginacija: Koliko dječjih komentara da preskočimo za svaki parent? **/
    skipChildren?: number
    /** Za određivanje blokiranih i prijavljenih komentara. **/
    contextUserId?: string
    /** Za određivanje blokiranih i prijavljenih komentara. **/
    anonUserId?: string
    /** Za dohvat podkomentara. **/
    parentId?: string
    /** Za dohvat kao stablo. **/
    asTree?: boolean
    /** Koliko duboko u stablu da vratimo podatke? 0 vraća nijedno dijete. 1 vraća neposrednu djecu, itd. **/
    maxTreeDepth?: number
}
[inline-code-end]

### The Response

[inline-code-attrs-start title = 'Struktura odgovora za komentare'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface CommentsResponse {
    status: 'success' | 'failed'
    /** Uključeno u slučaju neuspjeha. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-url-id' | 'missing-date' | 'unauthorized-page' | 'invalid-pagination-request' | 'invalid-limit' | 'invalid-limit-children' | 'invalid-skip' | 'invalid-skip-children' | 'invalid-max-tree-depth'
    /** Uključeno u slučaju neuspjeha. **/
    reason?: string
    /** Komentari! **/
    comments: Comment[]
}
[inline-code-end]

### Helpful Tips

#### URL ID

Vjerojatno ćete željeti koristiti `Comment` API sa parametrom `urlId`. Možete prvo pozvati `Pages` API, da vidite kako izgledaju vrijednosti `urlId` dostupne vama. 

#### Anonymous Actions

Za anonimno komentiranje vjerojatno ćete htjeti poslati `anonUserId` prilikom dohvaćanja komentara, kao i prilikom označavanja i blokiranja.

(!) Ovo je zahtjev za mnoge prodavnice aplikacija jer korisnici moraju moći prijaviti sadržaj koji su korisnici kreirali i koji mogu vidjeti, čak i ako nisu prijavljeni. Nepostupanje po tome može uzrokovati da vaša aplikacija bude uklonjena iz navedene prodavnice.

#### Comments Not Being Returned

Provjerite da li su vaši komentari odobreni i da nisu spam.

---
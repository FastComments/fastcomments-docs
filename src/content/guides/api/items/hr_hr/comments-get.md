[api-resource-header-start name = 'Comment'; route = 'GET /api/v1/comments'; creditsCost = 1; api-resource-header-end]

Ovaj API se koristi za dohvaćanje komentara za prikaz korisniku. Na primjer, automatski filtrira neopravdane ili spam komentare.

### Straničenje

Straničenje se može obaviti na jedan od dva načina, ovisno o zahtjevima za performansama i upotrebi:

1. Najbrže: **Prethodno izračunato straničenje**:
   1. Ovo je način na koji FastComments radi kada koristite naše predloške widgeta i klijente.
   2. Klik na "next" jednostavno povećava broj stranice.
   3. Možete to promatrati kao dohvaćanje iz key-value spremišta.
   4. Na ovaj način jednostavno definirajte parametar `page` počevši od `0` i smjer sortiranja kao `direction`.
   5. Veličine stranica se mogu prilagoditi putem pravila za prilagodbu.
2. Najfleksibilnije: **Fleksibilno straničenje**:
   1. Na ovaj način možete definirati prilagođene parametre `limit` i `skip`. Nemojte slati `page`.
   2. Podržan je i smjer sortiranja `direction`.
   3. `limit` je ukupni broj koji treba vratiti nakon primjene `skip`.
      - Primjer: postavite `skip = 200, limit = 100` kada je `page size = 100` i `page = 2`.
   4. Dječji komentari se i dalje računaju u straničenju. Možete zaobići ovo koristeći opciju `asTree`.
      - Dječje komentare možete straničiti putem `limitChildren` i `skipChildren`.
      - Možete ograničiti dubinu niti koja se vraća putem `maxTreeDepth`.

### Niti

1. Kada koristite `Precalculated Pagination`, komentari se grupiraju po *stranici* i komentari u nitima utječu na ukupnu stranicu.
   1. Na ovaj način, niti se mogu odrediti na klijentu na temelju `parentId`.
   2. Na primjer, za stranicu s jednim komentarom najviše razine i 29 odgovora, te postavljenim `page=0` u API-ju — dobit ćete samo komentar najviše razine i 29 djece.
   3. [Primjer slike koja ilustrira više stranica.](https://blog.winricklabs.com/images/fc-pagination02.png)
2. Kada koristite `Flexible Pagination`, možete definirati parametar `parentId`.
   1. Postavite ga na null da biste dobili samo komentare najviše razine.
   2. Zatim, za pregled niti, pozovite API ponovo i proslijedite `parentId`.
   3. Uobičajeno rješenje je napraviti API poziv za komentare najviše razine i potom paralelne API pozive za dohvaćanje komentara za djecu svakog komentara.
3. __NOVO od veljače 2023.!__ Dohvatite kao stablo koristeći `&asTree=true`.
   1. Možete ovo promatrati kao `Fleksibilno straničenje kao stablo`.
   2. Samo komentari najviše razine se računaju u straničenju.
   3. Postavite `parentId=null` da započnete stablo u korijenu (morate postaviti `parentId`).
   4. Postavite `skip` i `limit` za straničenje.
   5. Postavite `asTree` na `true`.
   6. Trošak kredita se povećava za `2x`, jer naš backend mora obaviti puno više posla u ovom scenariju.
   7. Po potrebi postavite `maxTreeDepth`, `limitChildren` i `skipChildren`.

### Objašnjenje stabala

Kod korištenja `asTree`, može biti teško razmišljati o straničenju. Evo korisne grafike:

<div class="screenshot white-bg">
    <div class="title">Dijagram straničenja stabla</div>
    <img class="screenshot-image" src="/images/fastcomments-comments-api-tree.png" alt="Dijagram straničenja stabla" />
</div>

### Dohvaćanje komentara u kontekstu korisnika

API `/comments` se može koristiti u dva konteksta, za različite slučajeve upotrebe:

- Za vraćanje komentara sortiranih i označenih informacijama za izgradnju vlastitog klijenta.
  - U tom slučaju definirajte upitni parametar `contextUserId`.
- Za dohvaćanje komentara iz vašeg backend-a za prilagođene integracije.
  - Platforma će prema zadanim postavkama koristiti ovo bez `contextUserId`. 

[inline-code-attrs-start title = 'Prethodno izračunato straničenje komentara'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&page=0&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR'
[inline-code-end]

[inline-code-attrs-start title = 'Fleksibilno straničenje komentara'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10'
[inline-code-end]

[inline-code-attrs-start title = 'Fleksibilno straničenje komentara u kontekstu korisnika'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10&contextUserId=my-user-id'
[inline-code-end]

[inline-code-attrs-start title = 'Fleksibilno straničenje komentara u kontekstu korisnika samo za komentare najviše razine'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10&contextUserId=my-user-id&parentId=null'
[inline-code-end]

### Dohvati komentare kao stablo

Moguće je dobiti komentare vraćene kao stablo, pri čemu se straničenje računa samo za komentare najviše razine.

[inline-code-attrs-start title = 'Komentari kao stablo u kontekstu korisnika'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10&contextUserId=my-user-id&parentId=null&asTree=true'
[inline-code-end]

Želite li dobiti samo komentare najviše razine i neposrednu djecu? Evo jednog načina:

[inline-code-attrs-start title = 'Komentari kao stablo s maksimalnom dubinom'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10&contextUserId=my-user-id&parentId=null&asTree=true&maxTreeDepth=1&limitChildren=10'
[inline-code-end]

Međutim, u vašem UI-ju možda ćete trebati znati treba li pokazati gumb "prikaži odgovore" na svakom komentaru. Kada dohvaćate komentare putem stabla, postoji svojstvo `hasChildren` dodano komentarima kada je primjenjivo.

### Dohvaćanje komentara kao stabla, pretraživanje po hashtag-u

Moguće je pretraživati po hashtag-u koristeći API, preko cijelog vašeg tenanta (nije ograničeno na jednu stranicu ili `urlId`).

U ovom primjeru izostavljamo `urlId`, i pretražujemo po više hashtagova. API će vratiti samo komentare koji imaju sve tražene hashtagove.

[inline-code-attrs-start title = 'Komentari kao stablo u kontekstu korisnika, po hashtag-u'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
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
    /** urlId (URL stranice ili id članka) s kojim su komentari povezani. **/
    urlId?: string
    /** Ograniči komentare koji se vraćaju za ovog korisnika. **/
    userId?: string
    /** Koristite ovo za pretraživanje po hashtag-u. Za sužavanje na presjek više hashtagova koristite &hashTag=a&hashTag=b. **/
    hashTag?: string
    /** Smjer sortiranja. Zadano je MR (Najrelevantnije). Ostale opcije su OF (Najstariji prvi) i NF (Najnoviji prvi). **/
    direction?: 'MR' | 'OF' | 'NF'
    /** Prethodno izračunato straničenje: Stranica za dohvat, počinje od 0. Proslijedite -1 za sve komentare (do 250). **/
    page?: number
    /** Fleksibilno straničenje: Koliko komentara treba vratiti? **/
    limit?: number
    /** Fleksibilno straničenje: Koliko dječjih komentara treba vratiti za svaki roditeljski komentar? **/
    limitChildren?: number
    /** Fleksibilno straničenje: Koliko komentara treba preskočiti? **/
    skip?: number
    /** Fleksibilno straničenje: Koliko dječjih komentara treba preskočiti za svaki roditeljski komentar? **/
    skipChildren?: number
    /** Za određivanje blokiranih i prijavljenih komentara. **/
    contextUserId?: string
    /** Za određivanje blokiranih i prijavljenih komentara. **/
    anonUserId?: string
    /** Za dohvaćanje dječjih komentara. **/
    parentId?: string
    /** Za dohvaćanje kao stablo. **/
    asTree?: boolean
    /** Koliko duboko u stablu treba vratiti podatke? 0 ne vraća djecu. 1 vraća neposrednu djecu, itd. **/
    maxTreeDepth?: number
}
[inline-code-end]

### Odgovor

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

### Korisni savjeti

#### URL ID

Vjerojatno želite koristiti `Comment` API s parametrima `urlId`. Možete prvo pozvati `Pages` API da vidite kako izgledaju dostupne vrijednosti `urlId`.

#### Anonimne radnje

Za anonimno komentiranje vjerojatno želite proslijediti `anonUserId` pri dohvaćanju komentara, kao i pri prijavljivanju i blokiranju.

(!) Ovo je zahtjev za mnoge trgovine aplikacija jer korisnici moraju moći prijaviti sadržaj koji su korisnici stvorili i koji mogu vidjeti, čak i ako nisu prijavljeni. Nepoštivanje toga može uzrokovati uklanjanje vaše aplikacije iz navedene trgovine.

#### Komentari se ne vraćaju

Provjerite jesu li vaši komentari odobreni i da nisu spam. 

---
[api-resource-header-start name = 'Comment'; route = 'GET /api/v1/comments'; creditsCost = 1; api-resource-header-end]

Ovaj API se koristi za dobijanje komentara za prikaz korisniku. Na primer, automatski filtrira
neodobrene ili spam komentare.

### Paginacija

Paginacija se može obaviti na jedan od dva načina, u zavisnosti od zahteva za performansama i slučaja upotrebe:

1. Najbrže: **Prekalkulisana paginacija**:
   1. Ovako FastComments radi kada koristite naše ugrađene vidžete i klijente.
   2. Klik na "next" jednostavno povećava broj stranice.
   3. Ovo možete posmatrati kao dobijeno iz key-value skladišta.
   4. Na ovaj način jednostavno definišite parametar `page` koji počinje od `0` i smer sortiranja kao `direction`.
   5. Veličine stranica mogu se prilagoditi putem pravila za prilagođavanje.
2. Najfleksibilnije: **Fleksibilna paginacija**:
   1. Na ovaj način možete definisati prilagođene parametre `limit` i `skip`. Ne prosleđujte `page`.
   2. Podržan je i smer sortiranja `direction`.
   3. `limit` je ukupan broj koji se vraća nakon što se primeni `skip`.
      - Primer: postavite `skip = 200, limit = 100` kada je `page size = 100` i `page = 2`.
   4. Podkomentari i dalje se računaju u paginaciji. Možete to zaobići koristeći opciju `asTree`.
      - Možete paginirati decu putem `limitChildren` i `skipChildren`.
      - Možete ograničiti dubinu vraćenih niti putem `maxTreeDepth`.

### Threads

1. Kada koristite `Precalculated Pagination`, komentari su grupisani po *stranici* i komentari u nitima utiču na ukupnu stranicu.
   1. Na ovaj način, niti se mogu odrediti na klijentu na osnovu `parentId`.
   2. Na primer, za stranicu sa jednim komentarem na vrhu i 29 odgovora, i postavljanjem `page=0` u API-ju — dobićete samo top-level komentar i 29 potomaka.
   3. [Primer slike koja ilustruje više stranica.](https://blog.winricklabs.com/images/fc-pagination02.png)
2. Kada koristite `Flexible Pagination`, možete definisati parametar `parentId`.
   1. Postavite ga na null da biste dobili samo komentare prvog nivoa.
   2. Zatim, da biste videli niti, pozovite API ponovo i prosledite `parentId`.
   3. Uobičajeno rešenje je da napravite API poziv za komentare prvog nivoa, a zatim paralelne API pozive da dobijete komentare za decu svakog komentara.
3. __NOVO od februara 2023!__ Dohvatajte kao stablo koristeći `&asTree=true`.
   1. Ovo možete posmatrati kao `Fleksibilna paginacija kao stablo`.
   2. Samo top-level komentari se računaju u paginaciji.
   3. Postavite `parentId=null` da biste započeli stablo od korena (morate postaviti `parentId`).
   4. Postavite `skip` i `limit` za paginaciju.
   5. Postavite `asTree` na `true`.
   6. Trošak kredita se povećava za `2x`, jer naš backend mora da obavi mnogo više posla u ovom scenariju.
   7. Postavite `maxTreeDepth`, `limitChildren` i `skipChildren` po želji.

### Objašnjenje stabala

Kada koristite `asTree`, može biti teško razmišljati o paginaciji. Evo korisne grafike:

<div class="screenshot white-bg">
    <div class="title">Dijagram paginacije stabla</div>
    <img class="screenshot-image" src="/images/fastcomments-comments-api-tree.png" alt="Dijagram paginacije stabla" />
</div>

### Preuzimanje komentara u kontekstu korisnika

API `/comments` se može koristiti u dva konteksta, za različite slučajeve upotrebe:

- Za vraćanje komentara sortirano i označeno informacijama za izgradnju sopstvenog klijenta.
  - U tom slučaju definišite parametar upita `contextUserId`.
- Za preuzimanje komentara sa vašeg backend-a za prilagođena integracije.
  - Platforma će podrazumevano koristiti ovo bez `contextUserId`. 

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

[inline-code-attrs-start title = 'Fleksibilna paginacija komentara u kontekstu korisnika samo za komentare prvog nivoa'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10&contextUserId=my-user-id&parentId=null'
[inline-code-end]

### Dobijanje komentara kao stabla

Moguće je dobiti komentare vraćene kao stablo, pri čemu paginacija broji samo komentare prvog nivoa.

[inline-code-attrs-start title = 'Komentari kao stablo u kontekstu korisnika'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10&contextUserId=my-user-id&parentId=null&asTree=true'
[inline-code-end]

Želite da dobijete samo komentare prvog nivoa i neposrednu decu? Evo jednog načina:

[inline-code-attrs-start title = 'Komentari kao stablo sa maksimalnom dubinom'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10&contextUserId=my-user-id&parentId=null&asTree=true&maxTreeDepth=1&limitChildren=10'
[inline-code-end]

Međutim, u vašem UI možda će vam trebati informacija da li prikazati dugme "prikaži odgovore" na
svakom komentaru. Kada preuzimate komentare kao stablo, postoji `hasChildren` svojstvo koje se dodaje
komentarima kada je primenljivo.

### Dobijanje komentara kao stablo, pretraga po haštagu

Moguće je pretraživati po haštagu koristeći API, preko čitavog vašeg tenant-a (nije ograničeno na jednu stranicu ili `urlId`).

U ovom primeru izostavljamo `urlId`, i pretražujemo po više haštagova. API će vratiti samo komentare koji imaju sve tražene haštagove.

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
    /** urlId (URL stranice ili ID članka) sa kojim su komentari povezani. **/
    urlId?: string
    /** Ograniči komentare koji se vraćaju za ovog korisnika. **/
    userId?: string
    /** Koristite ovo za pretragu po haštagu. Da biste suzili na preseku više haštagova, koristite &hashTag=a&hashTag=b. **/
    hashTag?: string
    /** Smer sortiranja. Podrazumevano je MR (Najrelevantniji). Druge opcije su OF (Najstarije prvo) i NF (Najnovije prvo). **/
    direction?: 'MR' | 'OF' | 'NF'
    /** Prekalkulisana paginacija: Stranica koju treba dohvatiti, počevši od 0. Prosledite -1 za sve komentare (do 250). **/
    page?: number
    /** Fleksibilna paginacija: Koliko komentara treba da vratimo? **/
    limit?: number
    /** Fleksibilna paginacija: Koliko podkomentara treba da vratimo za svaki roditelj? **/
    limitChildren?: number
    /** Fleksibilna paginacija: Koliko komentara treba da preskočimo? **/
    skip?: number
    /** Fleksibilna paginacija: Koliko podkomentara treba da preskočimo za svaki roditelj? **/
    skipChildren?: number
    /** Za određivanje blokiranih i prijavljenih komentara. **/
    contextUserId?: string
    /** Za određivanje blokiranih i prijavljenih komentara. **/
    anonUserId?: string
    /** Za preuzimanje podkomentara. **/
    parentId?: string
    /** Za preuzimanje kao stablo. **/
    asTree?: boolean
    /** Koliko duboko u stablu treba da vraćamo podatke? 0 vraća nijedno dete. 1 vraća neposrednu decu, itd. **/
    maxTreeDepth?: number
}
[inline-code-end]

### Odgovor

[inline-code-attrs-start title = 'Struktura odgovora za komentare'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface CommentsResponse {
    status: 'success' | 'failed'
    /** Uključeno u slučaju neuspeha. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-url-id' | 'missing-date' | 'unauthorized-page' | 'invalid-pagination-request' | 'invalid-limit' | 'invalid-limit-children' | 'invalid-skip' | 'invalid-skip-children' | 'invalid-max-tree-depth'
    /** Uključeno u slučaju neuspeha. **/
    reason?: string
    /** Komentari! **/
    comments: Comment[]
}
[inline-code-end]

### Korisni saveti

#### URL ID

Verovatno želite da koristite `Comment` API sa parametrima `urlId`. Najpre možete pozvati `Pages` API da vidite kako izgledaju vrednosti `urlId` dostupne vama. 

#### Anonimne radnje

Za anonimno komentarisanje verovatno ćete želeti da prosledite `anonUserId` prilikom preuzimanja komentara, i prilikom prijavljivanja i blokiranja.

(!) Ovo je obavezno za mnoge prodavnice aplikacija jer korisnici moraju biti u mogućnosti da prijave sadržaj koji su korisnici kreirali koji mogu da vide, čak i ako nisu prijavljeni. Nepoštovanje ovoga može dovesti do uklanjanja vaše aplikacije iz pomenute prodavnice.

#### Komentari se ne vraćaju

Proverite da li su vaši komentari odobreni i da nisu spam.

---
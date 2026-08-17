[api-resource-header-start name = 'Comment'; route = 'GET /api/v1/comments'; creditsCost = 1; api-resource-header-end]

Ovaj API se koristi za dobavljanje komentara za prikaz korisniku. Na primer, automatski filtrira neodobrene ili spam komentare.

### Pagination

Pagiranje se može izvršiti na jedan od dva načina, u zavisnosti od zahteva za performansama i slučaja upotrebe:

1. Najbrže: **Precalculated Pagination**:
   1. Ovo je način na koji FastComments radi kada koristite naše unapred izgrađene vidžete i klijente.
   2. Klikom na „next“ se jednostavno povećava broj stranice.
   3. Ovo možete zamisliti kao dohvat iz skladišta ključ‑vrednost.
   4. Na ovaj način, jednostavno definišite parametar `page` koji počinje od `0` i smer sortiranja kao `direction`.
   5. Veličine stranica mogu biti prilagođene putem pravila prilagođavanja.
2. Najfleksibilnije: **Flexible Pagination**:
   1. Na ovaj način možete definisati prilagođene parametre `limit` i `skip`. Ne prosleđujte `page`.
   2. Sortiranje `direction` je takođe podržano.
   3. `limit` je ukupan broj koji se vraća nakon što se primeni `skip`.
      - Primer: postavite `skip = 200, limit = 100` kada je `page size = 100` i `page = 2`.
   4. Podkomentari i dalje broje u paginaciji. Možete zaobići ovo korišćenjem opcije `asTree`.
      - Možete paginirati podkomentare putem `limitChildren` i `skipChildren`.
      - Možete ograničiti dubinu niti koje se vraćaju putem `maxTreeDepth`.

### Threads

1. Kada se koristi `Precalculated Pagination`, komentari su grupisani po *page* i komentari u nitima utiču na ukupnu stranicu.
   1. Na ovaj način, niti se mogu odrediti na klijentu na osnovu `parentId`.
   2. Na primer, na stranici sa jednim komentarom najvišeg nivoa i 29 odgovora, i postavkom `page=0` u API‑ju – dobićete samo komentar najvišeg nivoa i 29 podkomentara.
2. Kada se koristi `Flexible Pagination`, možete definisati parametar `parentId`.
   1. Postavite ga na null da biste dobili samo komentare najvišeg nivoa.
   2. Zatim, da biste videli niti, ponovo pozovite API i prosledite `parentId`.
   3. Uobičajeno rešenje je da napravite API poziv za komentare najvišeg nivoa, a zatim paralelne API pozive za dobijanje komentara za podkomentare svakog komentara.
3. __NEW As of Feb 2023!__ Dohvatite kao stablo koristeći `&asTree=true`.
   1. Ovo možete zamisliti kao `Flexible Pagination as a Tree`.
   2. Samo komentari najvišeg nivoa se broje u paginaciji.
   3. Postavite `parentId=null` da započnete stablo od korena (morate postaviti `parentId`).
   4. Postavite `skip` i `limit` za paginaciju.
   5. Postavite `asTree` na `true`.
   6. Trošak kredita se povećava za `2x`, jer naš backend mora da uradi mnogo više posla u ovom scenariju.
   7. Postavite `maxTreeDepth`, `limitChildren` i `skipChildren` po želji.

### Trees Explained

Kada se koristi `asTree`, može biti teško razmišljati o paginaciji. Evo praktične grafike:

<div class="screenshot white-bg">
    <div class="title">Dijagram paginacije stabla</div>
    <img class="screenshot-image" src="/images/fastcomments-comments-api-tree.png" alt="Dijagram paginacije stabla" />
</div>

### Fetching Comments in The Context of a User

`/comments` API se može koristiti u dva konteksta, za različite slučajeve upotrebe:

- Za vraćanje komentara sortiranih i označenih informacijama za izgradnju vašeg klijenta.
  - U ovom slučaju, definišite query parametar `contextUserId`.
- Za dohvat komentara sa vašeg backenda za prilagođene integracije.
  - Platforma će podrazumevano koristiti ovo bez `contextUserId`. 

[inline-code-attrs-start title = 'Komentari - Prekalkulisana paginacija'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&page=0&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR'
[inline-code-end]

[inline-code-attrs-start title = 'Komentari - Fleksibilna paginacija'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10'
[inline-code-end]

[inline-code-attrs-start title = 'Komentari - Fleksibilna paginacija u kontekstu korisnika'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10&contextUserId=my-user-id'
[inline-code-end]

[inline-code-attrs-start title = 'Komentari - Fleksibilna paginacija u kontekstu korisnika samo za komentare najvišeg nivoa'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10&contextUserId=my-user-id&parentId=null'
[inline-code-end]

### Get Comments as a Tree

Moguće je dobiti komentare vraćene kao stablo, pri čemu paginacija broji samo komentare najvišeg nivoa.

[inline-code-attrs-start title = 'Komentari - Kao stablo u kontekstu korisnika'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10&contextUserId=my-user-id&parentId=null&asTree=true'
[inline-code-end]

Želite li da dobijete samo komentare najvišeg nivoa i njihove neposredne podkomentare? Evo jednog načina:

[inline-code-attrs-start title = 'Komentari - Kao stablo sa maksimalnom dubinom'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10&contextUserId=my-user-id&parentId=null&asTree=true&maxTreeDepth=1&limitChildren=10'
[inline-code-end]

Međutim, u vašem UI‑ju možda ćete morati da znate da li da prikažete dugme „prikaži odgovore“ na svakom komentaru. Kada se komentari dohvaćaju putem stabla, postoji svojstvo `hasChildren` koje je označeno na komentarima kada je primenljivo.

### Get Comments as a Tree, Searching by Hash Tag

Moguće je pretraživati po hashtag‑u koristeći API, kroz ceo vaš tenant (nije ograničeno na jednu stranicu ili `urlId`).

U ovom primeru izostavljamo `urlId` i pretražujemo po više hashtag‑ova. API će vratiti samo komentare koji imaju sve tražene hashtag‑ove.

[inline-code-attrs-start title = 'Komentari - Kao stablo u kontekstu korisnika, po hashtag-u'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10&contextUserId=my-user-id&parentId=null&asTree=true&hashTag=TestTag&hashTag=OtherTestTag'
[inline-code-end]

### All Request Params

[inline-code-attrs-start title = 'Struktura zahteva za komentare'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface CommentsRequestQueryParams {
    tenantId: string
    API_KEY: string
    /** URL‑identifikator (URL stranice ili ID članka) sa kojim su komentari povezani. **/
    urlId?: string
    /** Ograničite komentare koje vraća ovaj korisnik. **/
    userId?: string
    /** Koristite ovo za pretragu po hashtag‑u. Da biste došli do preseka više hashtag‑ova, koristite &hashTag=a&hashTag=b. **/
    hashTag?: string
    /** Smer sortiranja. Podrazumevano je MR (Najrelevantnije). Ostale opcije su OF (Najstarije prvo) i NF (Najnovije prvo). **/
    direction?: 'MR' | 'OF' | 'NF'
    /** Precalculated Pagination: Stranica koju treba dohvatiti, počevši od 0. Prosledite -1 za sve komentare (do 250). **/
    page?: number
    /** Flexible Pagination: Koliko komentara treba da vratimo? **/
    limit?: number
    /** Flexible Pagination: Koliko podkomentara treba da vratimo za svakog roditelja? **/
    limitChildren?: number
    /** Flexible Pagination: Koliko komentara treba da preskočimo? **/
    skip?: number
    /** Flexible Pagination: Koliko podkomentara treba da preskočimo za svakog roditelja? **/
    skipChildren?: number
    /** Za određivanje blokiranih i označenih komentara. **/
    contextUserId?: string
    /** Za određivanje blokiranih i označenih komentara. **/
    anonUserId?: string
    /** Za dohvat podkomentara. **/
    parentId?: string
    /** Za dohvat kao stablo. **/
    asTree?: boolean
    /** Koliko duboko u stablo treba da vratimo podatke? 0 ne vraća decu. 1 vraća neposrednu decu, itd. **/
    maxTreeDepth?: number
}
[inline-code-end]

### The Response

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

### Helpful Tips

#### URL ID

Verovatno želite da koristite `Comment` API sa parametrom `urlId`. Prvo možete pozvati `Pages` API da vidite kako izgledaju dostupne vrednosti `urlId`.

#### Anonymous Actions

Za anonimno komentarisanje verovatno želite da prosledite `anonUserId` prilikom dohvaćanja komentara, kao i prilikom označavanja i blokiranja.

(!) Ovo je obavezno za mnoge prodavnice aplikacija jer korisnici moraju da mogu da označe sadržaj koji su videli, čak i ako nisu prijavljeni. Nepoštovanje može dovesti do uklanjanja vaše aplikacije iz te prodavnice.

#### Comments Not Being Returned

Proverite da li su vaši komentari odobreni i da nisu spam.

---
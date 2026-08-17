[api-resource-header-start name = 'Comment'; route = 'GET /api/v1/comments'; creditsCost = 1; api-resource-header-end]

Ta API se uporablja za pridobivanje komentarjev za prikaz uporabniku. Na primer, samodejno filtrira neodobrene ali neželene komentarje.

### Paginacija

Pagiranje je mogoče izvesti na enega od dveh načinov, odvisno od zahtev po zmogljivosti in primera uporabe:

1. **Najhitrejše**: **Precalculated Pagination**:
   1. Tako FastComments deluje, ko uporabljate naše vnaprej izdelane gradnike in odjemalce.
   2. Klik na "next" preprosto poveča število strani.
   3. To si lahko predstavljate kot pridobivanje iz shrambe ključ‑vrednost.
   4. Na ta način preprosto definirajte parameter `page`, ki se začne z `0`, in smer razvrščanja kot `direction`.
   5. Velikosti strani je mogoče prilagoditi prek pravil prilagajanja.
2. **Najbolj prilagodljivo**: **Flexible Pagination**:
   1. Na ta način lahko definirate lastne parametre `limit` in `skip`. Ne posredujte `page`.
   2. Smer `direction` je prav tako podprta.
   3. `limit` je skupno število, ki se vrne po uporabi `skip`.
      - Primer: nastavite `skip = 200, limit = 100`, ko je `page size = 100` in `page = 2`.
   4. Podkomentarji se še vedno štejejo v paginaciji. To lahko zaobidete z uporabo možnosti `asTree`.
      - Podkomentarje lahko paginirate z `limitChildren` in `skipChildren`.
      - Globino vrnjenih niti lahko omejite z `maxTreeDepth`.

### Niti

1. Ko uporabljate `Precalculated Pagination`, so komentarji združeni po *page* in komentarji v nitih vplivajo na celotno stran.
   1. Na ta način je mogoče niti določiti na odjemalcu na podlagi `parentId`.
   2. Na primer, pri strani z enim komentarjem najvišje ravni in 29 odgovori, ter nastavitvijo `page=0` v API-ju - dobili boste samo komentar najvišje ravni in 29 podkomentarjev.
2. Ko uporabljate `Flexible Pagination`, lahko definirate parameter `parentId`.
   1. Nastavite ga na null, da dobite le komentarje najvišje ravni.
   2. Nato za ogled niti ponovno pokličite API in posredujte `parentId`.
   3. Pogosta rešitev je, da najprej izvedete klic API-ja za komentarje najvišje ravni, nato pa vzporedne klice API-ja za pridobitev komentarjev za podkomentarje vsakega komentarja.
3. __NOVO od 2023-02!__ Fetch as a tree using `&asTree=true`.
   1. To si lahko predstavljate kot `Flexible Pagination as a Tree`.
   2. V paginaciji se štejejo le komentarji najvišje ravni.
   3. Nastavite `parentId=null`, da začnete drevo pri korenu (morate nastaviti `parentId`).
   4. Nastavite `skip` in `limit` za paginacijo.
   5. Nastavite `asTree` na `true`.
   6. Strošek kreditov se poveča za `2x`, saj mora naš strežnik v tem scenariju opraviti veliko več dela.
   7. Nastavite `maxTreeDepth`, `limitChildren` in `skipChildren` po želji.

### Razlaga dreves

Ko uporabljate `asTree`, je lahko težko razumeti paginacijo. Tukaj je priročna grafika:

<div class="screenshot white-bg">
    <div class="title">Diagram paginacije dreves</div>
    <img class="screenshot-image" src="/images/fastcomments-comments-api-tree.png" alt="Diagram paginacije dreves" />
</div>

### Pridobivanje komentarjev v kontekstu uporabnika

`/comments` API se lahko uporablja v dveh kontekstih, za različne primere uporabe:

- Za vračanje komentarjev, razvrščenih in označenih z informacijami za izgradnjo lastnega odjemalca.
  - V tem primeru definirajte poizvedbeni parameter `contextUserId`.
- Za pridobivanje komentarjev iz vašega strežnika za prilagojene integracije.
  - Platforma bo to privzeto uporabila brez `contextUserId`.

[inline-code-attrs-start title = 'Komentarji vnaprej izračunana paginacija'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&page=0&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR'
[inline-code-end]

[inline-code-attrs-start title = 'Komentarji prilagodljiva paginacija'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10'
[inline-code-end]

[inline-code-attrs-start title = 'Komentarji prilagodljiva paginacija v kontekstu uporabnika'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10&contextUserId=my-user-id'
[inline-code-end]

[inline-code-attrs-start title = 'Komentarji prilagodljiva paginacija v kontekstu uporabnika samo za komentarje najvišje ravni'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10&contextUserId=my-user-id&parentId=null'
[inline-code-end]

### Pridobivanje komentarjev kot drevo

Možno je dobiti komentarje vrnjene kot drevo, pri čemer paginacija šteje le komentarje najvišje ravni.

[inline-code-attrs-start title = 'Komentarji kot drevo v kontekstu uporabnika'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10&contextUserId=my-user-id&parentId=null&asTree=true'
[inline-code-end]

Želite dobiti le komentarje najvišje ravni in takojšnje podkomentarje? Tukaj je en način:

[inline-code-attrs-start title = 'Komentarji kot drevo z največjo globino'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10&contextUserId=my-user-id&parentId=null&asTree=true&maxTreeDepth=1&limitChildren=10'
[inline-code-end]

Vendar pa boste v vašem UI morda morali vedeti, ali naj se prikaže gumb "prikaži odgovore" na vsakem komentarju. Ko pridobivate komentarje prek drevesa, je na komentarje označena lastnost `hasChildren`, kadar je to primerno.

### Pridobivanje komentarjev kot drevo, iskanje po hashtag-u

Možno je iskati po hashtag-u z uporabo API-ja, po celotnem najemniku (ni omejeno na eno stran ali `urlId`).

V tem primeru izpustimo `urlId` in iščemo po več hashtag-ih. API bo vrnil le komentarje, ki imajo vse zahtevane hashtag-e.

[inline-code-attrs-start title = 'Komentarji kot drevo v kontekstu uporabnika, po hashtag-u'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10&contextUserId=my-user-id&parentId=null&asTree=true&hashTag=TestTag&hashTag=OtherTestTag'
[inline-code-end]

### Vsi parametri zahteve

[inline-code-attrs-start title = 'Struktura zahteve za komentarje'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface CommentsRequestQueryParams {
    tenantId: string
    API_KEY: string
    /** urlId (URL strani ali ID članka), s katerim so komentarji povezani. **/
    urlId?: string
    /** Omeji komentarje, ki jih vrne ta uporabnik. **/
    userId?: string
    /** Uporabite to za iskanje po hashtag-u. Za iskanje preseka več hashtag-ov uporabite &hashTag=a&hashTag=b. **/
    hashTag?: string
    /** Smer razvrščanja. Privzeto je MR (najbolj relevantno). Drugi možnosti so OF (najstarejše najprej) in NF (najnovejše najprej). **/
    direction?: 'MR' | 'OF' | 'NF'
    /** Precalculated Pagination: Stran, ki jo želite pridobiti, začne z 0. Za vse komentarje (do 250) podajte -1. **/
    page?: number
    /** Flexible Pagination: Koliko komentarjev naj vrnemo? **/
    limit?: number
    /** Flexible Pagination: Koliko podkomentarjev naj vrnemo za vsakega starša? **/
    limitChildren?: number
    /** Flexible Pagination: Koliko komentarjev naj preskočimo? **/
    skip?: number
    /** Flexible Pagination: Koliko podkomentarjev naj preskočimo za vsakega starša? **/
    skipChildren?: number
    /** Za določanje blokiranih in označenih komentarjev. **/
    contextUserId?: string
    /** Za določanje blokiranih in označenih komentarjev. **/
    anonUserId?: string
    /** Za pridobivanje podkomentarjev. **/
    parentId?: string
    /** Za pridobivanje kot drevo. **/
    asTree?: boolean
    /** Kako globoko v drevo naj vrnemo podatke? 0 ne vrne podkomentarjev. 1 vrne takojšnje podkomentarje, itd. **/
    maxTreeDepth?: number
}
[inline-code-end]

### Odgovor

[inline-code-attrs-start title = 'Struktura odgovora za komentarje'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface CommentsResponse {
    status: 'success' | 'failed'
    /** Vključeno ob napaki. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-url-id' | 'missing-date' | 'unauthorized-page' | 'invalid-pagination-request' | 'invalid-limit' | 'invalid-limit-children' | 'invalid-skip' | 'invalid-skip-children' | 'invalid-max-tree-depth'
    /** Vključeno ob napaki. **/
    reason?: string
    /** Komentarji! **/
    comments: Comment[]
}
[inline-code-end]

### Koristni nasveti

#### URL ID

Verjetno želite uporabiti API `Comment` s parametrom `urlId`. Najprej lahko pokličete API `Pages`, da vidite, kako izgledajo vrednosti `urlId`, ki so vam na voljo.

#### Anonimna dejanja

Za anonimno komentiranje verjetno želite posredovati `anonUserId` pri pridobivanju komentarjev ter pri označevanju in blokiranju.

(!) To je zahtevano v mnogih trgovinah z aplikacijami, saj morajo uporabniki imeti možnost označiti vsebino, ki jo ustvarijo drugi uporabniki, tudi če niso prijavljeni. Če tega ne storite, lahko vaša aplikacija odstrani iz omenjene trgovine.

#### Komentarji se ne vračajo

Preverite, da so vaši komentarji odobreni in niso neželeni.

---
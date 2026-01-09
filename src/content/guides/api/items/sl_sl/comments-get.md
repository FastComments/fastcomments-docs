[api-resource-header-start name = 'Comment'; route = 'GET /api/v1/comments'; creditsCost = 1; api-resource-header-end]

Ta API se uporablja za pridobivanje komentarjev za prikaz uporabniku. Na primer, samodejno filtrira neodobrene ali neželene (spam) komentarje.

### Straničenje

Straničenje je mogoče izvesti na enega od dveh načinov, odvisno od zahtev glede zmogljivosti in primera uporabe:

1. Najhitrejše: **Vnaprej izračunano straničenje**:
   1. Tako FastComments deluje, ko uporabljate naše vnaprej izdelane widgete in odjemalce.
   2. Klik na "next" preprosto poveča števec strani.
   3. To lahko razumete kot pridobitev iz ključ-vrednost shrambe.
   4. Na ta način preprosto določite parameter `page`, ki se začne pri `0`, in smer razvrščanja kot `direction`.
   5. Velikosti strani je mogoče prilagoditi preko pravil prilagoditve.
2. Najbolj prilagodljivo: **Fleksibilno straničenje**:
   1. Na ta način lahko določite po meri `limit` in `skip` parametra. Ne pošiljajte `page`.
   2. Podprta je tudi smer razvrščanja `direction`.
   3. `limit` je skupno število, ki ga vrnemo po uporabi `skip`.
      - Primer: nastavite `skip = 200, limit = 100`, ko je `page size = 100` in `page = 2`.
   4. Otroški komentarji se še vedno štejejo v straničenju. To lahko zaobidete z uporabo opcije `asTree`.
      - Otroke lahko straničite z `limitChildren` in `skipChildren`.
      - Globino vrnjenih niti lahko omejite z `maxTreeDepth`.

### Niti

1. Ko uporabljate `Precalculated Pagination`, so komentarji združeni po *strani* in komentarji v nitih vplivajo na celotno stran.
   1. Tako se niti lahko določijo na odjemalcu glede na `parentId`.
   2. Na primer, na strani z enim vrhnjim komentarjem in 29 odgovori, in če v API nastavite `page=0` - boste dobili samo vrhnji komentar in 29 otrok.
   3. [Primer slike tukaj, ki prikazuje več strani.](https://blog.winricklabs.com/images/fc-pagination02.png)
2. Ko uporabljate `Flexible Pagination`, lahko določite parameter `parentId`.
   1. Nastavite ga na null, da dobite samo vrhnje komentarje.
   2. Nato za ogled niti pokličite API znova in posredujte `parentId`.
   3. Pogosta rešitev je, da naredite API klic za vrhnje komentarje in nato vzporedne API klice za pridobitev komentarjev za otroke vsakega komentarja.
3. __NOVO od februarja 2023!__ Pridobite kot drevo z uporabo `&asTree=true`.
   1. To lahko razumete kot `Fleksibilno straničenje kot drevo`.
   2. Samo vrhnji komentarji štejejo v straničenju.
   3. Nastavite `parentId=null`, da začnete drevo na korenu (mora biti nastavljen `parentId`).
   4. Nastavite `skip` in `limit` za straničenje.
   5. Nastavite `asTree` na `true`.
   6. Cena v kreditih se poveča za `2x`, saj mora naš backend v tem scenariju opraviti veliko več dela.
   7. Nastavite `maxTreeDepth`, `limitChildren` in `skipChildren` po želji.

### Drevesa pojasnjena

Ko uporabljate `asTree`, je lahko težko razmišljati o straničenju. Tukaj je priročna grafika:

<div class="screenshot white-bg">
    <div class="title">Diagram straničenja dreves</div>
    <img class="screenshot-image" src="/images/fastcomments-comments-api-tree.png" alt="Diagram straničenja dreves" />
</div>

### Pridobivanje komentarjev v kontekstu uporabnika

API `/comments` se lahko uporablja v dveh kontekstih, za različne primere uporabe:

- Za vračanje komentarjev, razvrščenih in označenih z informacijami za izdelavo vašega lastnega odjemalca.
  - V tem primeru določite parameter poizvedbe `contextUserId`.
- Za pridobivanje komentarjev iz vašega backenda za prilagojene integracije.
  - Platforma bo to privzeto uporabila, če `contextUserId` ni podan. 

[inline-code-attrs-start title = 'Komentarji - vnaprej izračunano straničenje'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&page=0&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR'
[inline-code-end]

[inline-code-attrs-start title = 'Komentarji - fleksibilno straničenje'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10'
[inline-code-end]

[inline-code-attrs-start title = 'Komentarji - fleksibilno straničenje v uporabniškem kontekstu'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10&contextUserId=my-user-id'
[inline-code-end]

[inline-code-attrs-start title = 'Komentarji - fleksibilno straničenje v uporabniškem kontekstu samo za vrhnje komentarje'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10&contextUserId=my-user-id&parentId=null'
[inline-code-end]

### Pridobitev komentarjev kot drevo

Možno je vrniti komentarje kot drevo, pri čemer straničenje upošteva samo vrhnje komentarje.

[inline-code-attrs-start title = 'Komentarji kot drevo v uporabniškem kontekstu'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10&contextUserId=my-user-id&parentId=null&asTree=true'
[inline-code-end]

Želite pridobiti samo vrhnje komentarje in neposredne otroke? Tukaj je en način:

[inline-code-attrs-start title = 'Komentarji kot drevo z največjo globino'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10&contextUserId=my-user-id&parentId=null&asTree=true&maxTreeDepth=1&limitChildren=10'
[inline-code-end]

Vendar boste v svojem UI morda morali vedeti, ali prikazati gumb "prikaži odgovore" pri vsakem komentarju. Ko pridobivate komentarje kot drevo, imajo komentarji, kjer je primerno, lastnost `hasChildren`.

### Pridobitev komentarjev kot drevo, iskanje po hashtagih

Možno je iskati po hashtagih z uporabo API-ja, po celotnem vašem najemniku (ne omejeno na eno stran ali `urlId`).

V tem primeru izpustimo `urlId` in iščemo po več hashtagih. API bo vrnil samo komentarje, ki vsebujejo vse zahtevane hashtage.

[inline-code-attrs-start title = 'Komentarji kot drevo v uporabniškem kontekstu, po hashtagih'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
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
    /** Omejite komentarje, ki jih vrne ta uporabnik. **/
    userId?: string
    /** Uporabite to za iskanje po hashtagih. Če želite zožiti iskanje na presečišče več hashtagov, uporabite &hashTag=a&hashTag=b. **/
    hashTag?: string
    /** Smer razvrščanja. Privzeto je MR (Najbolj relevantno). Druge možnosti so OF (Najstarejši prvi) in NF (Najnovejši prvi). **/
    direction?: 'MR' | 'OF' | 'NF'
    /** Vnaprej izračunano straničenje: Stran za pridobitev, začetek pri 0. Za vse komentarje (do 250) uporabite -1. **/
    page?: number
    /** Fleksibilno straničenje: Koliko komentarjev naj vrnemo? **/
    limit?: number
    /** Fleksibilno straničenje: Koliko otroških komentarjev naj vrnemo za vsakega starša? **/
    limitChildren?: number
    /** Fleksibilno straničenje: Koliko komentarjev naj preskočimo? **/
    skip?: number
    /** Fleksibilno straničenje: Koliko otroških komentarjev naj preskočimo za vsakega starša? **/
    skipChildren?: number
    /** Za določanje blokiranih in prijavljenih komentarjev. **/
    contextUserId?: string
    /** Za določanje blokiranih in prijavljenih komentarjev. **/
    anonUserId?: string
    /** Za pridobivanje otroških komentarjev. **/
    parentId?: string
    /** Za pridobitev kot drevo. **/
    asTree?: boolean
    /** Kako globoko v drevo naj vrnemo podatke? 0 ne vrne otrok. 1 vrne neposredne otroke itd. **/
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

Verjetno boste želeli uporabiti API `Comment` s parametrom `urlId`. Najprej lahko pokličete API `Pages`, da vidite, kako izgledajo razpoložljive vrednosti `urlId`.

#### Anonimna dejanja

Za anonimno komentiranje boste verjetno želeli podati `anonUserId` pri pridobivanju komentarjev in pri prijavljanju in blokiranju.

(!) To je zahtevano pri mnogih trgovinah z aplikacijami, saj mora uporabnik lahko prijavi vsebino, ki jo je ustvaril uporabnik in ki jo vidi, tudi če ni prijavljen. Neupoštevanje tega lahko povzroči, da bo vaša aplikacija odstranjena iz navedene trgovine.

#### Komentarji se ne vračajo

Preverite, ali so vaši komentarji odobreni in niso neželeni (spam).
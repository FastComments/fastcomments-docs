Plaćanje može propasti jer je kartica istekla, zamijenjena je, nema sredstava ili je banka blokirala naplatu. FastComments vam daje vremena da to ispravite prije nego što nešto prestane raditi, i obavještava osobe koje to mogu popraviti.

### Što se događa

1. **Naplata propada.** Račun ostaje `Unpaid` u [Billing History](https://fastcomments.com/auth/my-account/billing-history) s gumbom `Pay Now`. Na fiksnom planu Stripe može pokušati ponovnu naplatu. Na Flexu, FastComments svake noći pokušava naplatiti sve neplaćene račune starije od karence.
2. **Šalje se upozorenje.** Svake noći FastComments provjerava svaki račun. Ako je Flex račun i dalje neplaćen nakon karence i ponovna naplata propadne, ili je pretplata na fiksnom planu završila jer naplata nije uspjela, svaki korisnik s dovolom `Billing Admin` dobiva e-mail `Your FastComments Tenant Will Be Deactivated Soon!` s datumom, a račun se zakazuje za deaktivaciju jednu karencu kasnije. Widget u međuvremenu i dalje radi.
3. **Deaktivacija.** Ako se ništa nije promijenilo do zakazanog datuma, račun se deaktivira: komentari su skriveni na vašoj stranici, a posjetitelji vide `This FastComments account's billing information is more than sixty days out of date. Comments hidden until the issue is resolved.` Administratori naplate ponovno dobivaju e-mail. Vaši podaci se čuvaju.

Karenca je jedan tjedan za većinu računa i dva tjedna za račune starije od godinu dana. Na Flexu primjenjuje se dvaput, jednom prije upozorenja i jednom između upozorenja i deaktivacije, tako da između propale naplate i skrivanja komentara prođe najmanje dva tjedna.

### Kako to popraviti

Spremite ispravnu karticu na stranici [Billing Info](https://fastcomments.com/auth/my-account/billing-info). To samo po sebi odmah reaktivira račun i otkazuje svaku zakazanu deaktivaciju; neplaćeni račun se zatim naplaćuje novom karticom, odmah na fiksnom planu i tijekom noćne provjere na Flexu. Također možete odmah platiti račun pomoću `Pay Now` u Billing History.

Ako je kartica na datoteci ispravna, a propast je bila privremena, ne morate ništa poduzeti: noćna provjera primijetit će plaćeni račun ili aktivnu pretplatu, otkazat će deaktivaciju i poslati e-mail administratorima naplate kao potvrdu. Već deaktivirani račun se na isti način reaktivira.

### Nekoliko računa odjednom

Ako je vaša kartica bila neispravna nekoliko mjeseci, a zatim ispravljena, svaki neplaćeni mjesec se naplaćuje odvojeno, pa možete vidjeti nekoliko naplata istog dana. Provjerite razdoblje naplate na svakom računu u Billing History: odnose se na različite mjesece, nisu duplikati.

### I dalje vidite upozorenje o naplati nakon plaćanja?

Upozorenje pripada određenom računu. Ako ste platili, ali stranica i dalje prikazuje `billing information is out of date`, widget na toj stranici najvjerojatnije koristi drugi račun od onog na kojem ste platili. To se događa kada je stranica postavljena dvaput, ili kada postoji jedan račun na `fastcomments.com` i drugi na EU implementaciji na `eu.fastcomments.com`, koji se naplaćuju odvojeno. Usporedite `tenantId` u kodu na vašoj stranici s onim pod `Settings -> API/SSO`, ili [kontaktirajte podršku](https://fastcomments.com/auth/my-account/help) i to ćemo riješiti.
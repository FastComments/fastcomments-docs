Plaćanje može da padne jer je kartica istekla, bila zamenjena, nema sredstava, ili je banka blokirala naplatu. FastComments vam daje vreme da to ispravite pre nego što bilo šta prestane da radi, i obaveštava osobe koje to mogu da reše.

### Šta Se Događa

1. **Naplata pada.** Faktura ostaje `Unpaid` u [Billing History](https://fastcomments.com/auth/my-account/billing-history) sa dugmetom `Pay Now`. Na fiksnom planu Stripe može da pokuša ponovnu naplatu. Na Flex planu, FastComments svake noći pokušava da naplati svaku neplaćenu fakturu stariju od grace perioda.
2. **Šalje se upozorenje.** Svake noći FastComments proverava svaki nalog. Ako Flex faktura i dalje nije plaćena nakon grace perioda i ponovna naplata padne, ili je pretplata na fiksnom planu završena jer nije mogla da se naplati, svaki korisnik sa dozvolom `Billing Admin` dobija imejl `Your FastComments Tenant Will Be Deactivated Soon!` sa datumom, a nalog se zakazuje za deaktivaciju jedan grace period kasnije. Vidžet u međuvremenu i dalje radi.
3. **Deaktivacija.** Ako se ništa nije promenilo do zakazanog datuma, nalog se deaktivira: komentari se sakrivaju na vašem sajtu i posetioci vide `This FastComments account's billing information is more than sixty days out of date. Comments hidden until the issue is resolved.` Billing admini ponovo dobijaju imejl. Vaši podaci se čuvaju.

Grace period je jedna nedelja za većinu naloga i dve nedelje za naloge starije od godinu dana. Na Flex planu primenjuje se dva puta, jednom pre upozorenja i jednom između upozorenja i deaktivacije, tako da između neuspele naplate i skrivanja komentara prođe najmanje dve nedelje.

### Kako Da To Rešite

Sačuvajte ispravnu karticu na stranici [Billing Info](https://fastcomments.com/auth/my-account/billing-info). To samo po sebi odmah reaktivira nalog i otkazuje svako zakazano deaktivaciju; neplaćena faktura se zatim naplaćuje novom karticom, odmah na fiksnom planu i tokom noćne provere na Flex planu. Takođe možete odmah platiti fakturu dugmetom `Pay Now` u Billing History.

Ako je kartica na datoteci ispravna i neuspeh je bio privremen, ne morate ništa da radite: noćna provera uoči plaćenu fakturu ili aktivnu pretplatu, otkazuje deaktivaciju i šalje imejl billing adminima radi potvrde. Već deaktiviran nalog se na isti način reaktivira.

### Više Faktura Odjednom

Ako je vaša kartica bila nevažeća nekoliko meseci, a zatim ispravljena, svaki neplaćeni mesec se naplaćuje odvojeno, tako da možete videti više naplata istog dana. Proverite obračunski period na svakoj fakturi u Billing History: odnose se na različite mesece, a nisu duplikati.

### I Dalje Vidite Upozorenje Za Naplatu Nakon Plaćanja?

Upozorenje pripada konkretnom nalogu. Ako ste platili, ali sajt i dalje prikazuje `billing information is out of date`, vidžet na tom sajtu najverovatnije koristi drugi nalog od onog na kojem ste platili. To se dešava kada je sajt podešen dva puta, ili kada postoji jedan nalog na `fastcomments.com` i drugi na EU implementaciji na `eu.fastcomments.com`, koji se naplaćuju odvojeno. Uporedite `tenantId` u kodu na vašoj stranici sa onim pod `Settings -> API/SSO`, ili [kontaktirajte podršku](https://fastcomments.com/auth/my-account/help) i rešićemo to.
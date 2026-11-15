Plačilo lahko ne uspe, ker je kartica potekla, bila zamenjana, nima sredstev ali je banka blokirala breme. FastComments vam da čas, da to uredite, preden kaj preneha delovati, in obvesti ljudi, ki lahko to popravijo.

### Kaj se zgodi

1. **Breme ne uspe.** Račun ostane `Unpaid` v [Billing History](https://fastcomments.com/auth/my-account/billing-history) z gumbom `Pay Now`. Na fiksni paket Stripe lahko poskusi ponovno breme. Na Flex, FastComments vsako noč poskusi ponovno breme za vsak neplačan račun, starejši od obdobja milosti.
2. **Opozorilo je poslano.** Vsako noč FastComments preveri vsak račun. Če je Flex račun še vedno neplačan po obdobju milosti in ponovni poskus ne uspe, ali če je fiksna naročnina končana, ker je ni bilo mogoče plačati, vsak uporabnik z dovoljenjem `Billing Admin` prejme e-pošto `Your FastComments Tenant Will Be Deactivated Soon!` z datumom, in račun je načrtovan za deaktivacijo eno obdobje milosti kasneje. Widget v tem času še naprej deluje.
3. **Deaktivacija.** Če se nič ni spremenilo do načrtovanega datuma, je račun deaktiviran: komentarji so skriti na vašem spletnem mestu in obiskovalci vidijo `This FastComments account's billing information is more than sixty days out of date. Comments hidden until the issue is resolved.` Skrbnikom za obračunavanje je poslana e-pošta še enkrat. Vaši podatki so ohranjeni.

Obdobje milosti je en teden za večino računov in dva tedna za račune, starejše od enega leta. Na Flex se uporabi dvakrat, enkrat pred opozorilom in enkrat med opozorilom in deaktivacijo, tako da je med neuspelim bremenom in skritimi komentarji vsaj dva tedna.

### Kako to urediti

Shranite delujočo kartico na strani [Billing Info](https://fastcomments.com/auth/my-account/billing-info). To samo takoj ponovno aktivira račun in prekliče vsako načrtovano deaktivacijo; neplačan račun je nato plačan z novo kartico, takoj na fiksni paket in med nočnim preverjanjem na Flex. Račun lahko plačate tudi takoj z `Pay Now` v Billing History.

Če je kartica v redu in je napaka bila začasna, vam ni treba storiti ničesar: nočno preverjanje opazi plačan račun ali aktivno naročnino, prekliče deaktivacijo in pošlje e-pošto skrbnikom za obračunavanje za potrditev. Že deaktiviran račun je ponovno aktiviran na enak način.

### Več računov hkrati

Če je bila vaša kartica neveljavna nekaj mesecev in nato popravljena, je vsak neplačan mesec plačan ločeno, tako da lahko vidite več bremen na isti dan. Preverite obdobje obračunavanja na vsakem računu v Billing History: so za različne mesece, ne podvojeni.

### Še vedno vidite opozorilo o obračunavanju po plačilu?

Opozorilo pripada določenemu računu. Če ste plačali, ampak spletno mesto še vedno prikazuje `billing information is out of date`, widget na tem spletnem mestu najverjetneje uporablja drug račun kot tistega, na katerega ste plačali. To se zgodi, ko je bilo spletno mesto nastavljeno dvakrat, ali ko je en račun na `fastcomments.com` in drug na EU namestitvi na `eu.fastcomments.com`, ki sta obračunana ločeno. Primerjajte `tenantId` v kodi na vaši strani s tistim pod `Settings -> API/SSO`, ali [kontaktirajte podporo](https://fastcomments.com/auth/my-account/help) in bomo uredili.
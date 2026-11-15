Što se naplaćuje s vaše kartice i kada, ovisi o vrsti plana koji imate.

### Fiksni planovi

Kada započnete pretplatu na `Creators` ili `Pro`, prvi mjesec ili godina naplaćuje se odmah, u pregledniku, kada kliknete `Pay Now & Start Subscription`. Ako vam banka zatraži potvrdu plaćanja, dovršite provjeru u prozoru koji se otvori. Pretplata počinje tek nakon što plaćanje prođe.

Nakon toga, pretplata se obnavlja istog dana svakog mjeseca, ili svake godine ako ste odabrali `Pay Annually`, računajući od dana pretplate. Obnova se automatski naplaćuje s kartice na datoteci, a račun se šalje e-poštom na vašu adresu za naplatu. Nije potrebna nikakva radnja.

Godišnja naplata je 15% jeftinija od mjesečne. Flex se ne može naplaćivati godišnje.

### Flex

Flex se naplaćuje naknadno: koristite uslugu tijekom kalendarskog mjeseca, a račun za taj mjesec stvara se posljednjeg dana mjeseca (UTC) i odmah se naplaćuje s vaše kartice. Datum naplate je uvijek posljednji dan mjeseca i ne može se promijeniti.

Račun pokriva samo taj kalendarski mjesec. Korištenje u kalendarskom mjesecu u kojem ste se prijavili nikada se ne naplaćuje, tako da je vaš prvi račun najranije na kraju mjeseca nakon prijave.

Ako vaše korištenje bude manje od Flex minimuma, stavka `FastComments Flex. (minimum ... /mo)` dopunjuje račun do minimalnog iznosa. Trenutni minimum prikazan je na vašoj stranici [Billing Info](https://fastcomments.com/auth/my-account/billing-info) i na [stranici s cijenama](https://fastcomments.com/traffic-pricing).

U svakom trenutku možete vidjeti koliko će iznositi sljedeći račun. `Next Invoice Amount` nalazi se na stranici [Billing Info](https://fastcomments.com/auth/my-account/billing-info), a potpuni raščlamba po stavkama na stranici [Billing Analytics](https://fastcomments.com/auth/my-account/analytics/billing). Što se broji opisano je u [Kako se mjeri Flex korištenje](#how-flex-usage-is-measured).

### Spremanje kartice je ne naplaćuje

Jednom kada je naplata postavljena, spremanje ili ažuriranje kartice bez promjene plana je nikada ne naplaćuje, osim ako imate neplaćen račun. Obrazac kaže `You won't be charged now unless you change plans.` i kartica se jednostavno pohranjuje kod Stripe-a za sljedeći račun. Tek kada još nemate valjanu naplatu, obrazac kaže `Your card will be charged now.`, a čak i tada Flex račun nije naplaćen do kraja mjeseca. Svaki unaprijed plaćeni [kredit](#credits) na vašem računu primjenjuje se na svaki račun prije naplate kartice.

### Upravljani pod-računi

Ako vaš račun upravlja pod-računima (na primjer jedan po stranici ili okruženju, ili white-labeled stanari), naplaćuje se samo nadređeni račun. Korištenje svakog pod-računa pojavljuje se kao vlastite stavke na Flex računu nadređenog računa, s prefiksom imena pod-računa, a pod-računi dijele način plaćanja i status naplate nadređenog računa.
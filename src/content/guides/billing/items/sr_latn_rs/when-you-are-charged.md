Šta se naplaćuje sa vaše kartice, i kada, zavisi od toga koji plan koristite.

### Fiksni planovi

Kada započnete `Creators` ili `Pro` pretplatu, prvi mesec ili godina se naplaćuje odmah, u pregledaču, kada kliknete `Pay Now & Start Subscription`. Ako vam banka zatraži potvrdu plaćanja, završite proveru u prozoru koji se otvori. Pretplata počinje tek kada plaćanje prođe.

Posle toga, pretplata se obnavlja istog dana svakog meseca, ili svake godine ako ste izabrali `Pay Annually`, računajući od dana kada ste se pretplatili. Obnova se automatski naplaćuje sa kartice na datoteci, a faktura se šalje na vašu e-adresu za naplatu. Nema potrebe za nikakvom akcijom.

Godišnja naplata je 15% jeftinija od mesečne. Flex se ne može naplaćivati godišnje.

### Flex

Flex se naplaćuje naknadno: koristite servis tokom kalendarskog meseca, a faktura za taj mesec se kreira poslednjeg dana meseca (UTC) i odmah naplaćuje sa vaše kartice. Datum naplate je uvek poslednji dan meseca i ne može se promeniti.

Faktura pokriva samo taj kalendarski mesec. Korišćenje u kalendarskom mesecu u kojem ste se prijavili se nikada ne naplaćuje, tako da je vaša prva faktura najranije na kraju meseca posle prijave.

Ako vaše korišćenje bude manje od Flex minimuma, stavka `FastComments Flex. (minimum ... /mo)` dopunjuje fakturu do minimalnog iznosa. Trenutni minimum je prikazan na vašoj stranici [Billing Info](https://fastcomments.com/auth/my-account/billing-info) i na [stranici sa cenama](https://fastcomments.com/traffic-pricing).

U svakom trenutku možete videti kako će izgledati sledeća faktura. `Next Invoice Amount` se nalazi na stranici [Billing Info](https://fastcomments.com/auth/my-account/billing-info), a potpuni raspored po stavkama na stranici [Billing Analytics](https://fastcomments.com/auth/my-account/analytics/billing). Šta se računa je opisano u [Kako se meri Flex korišćenje](#how-flex-usage-is-measured).

### Čuvanje kartice je ne naplaćuje

Kada vaša naplata bude podešena, čuvanje ili ažuriranje kartice bez promene plana je nikada ne naplaćuje, osim ako imate neplaćenu fakturu. Forma kaže `You won't be charged now unless you change plans.` i kartica se samo čuva kod Stripe-a za sledeću fakturu. Tek kada još nemate važeću naplatu, forma kaže `Your card will be charged now.`, i čak i tada Flex nalog nije naplaćen do kraja meseca. Svaki preplaćeni [kredit](#credits) na vašem nalogu se primenjuje na svaku fakturu pre naplate sa kartice.

### Upravljani pod-nalozi

Ako vaš nalog upravlja pod-nalozima (na primer jedan po sajtu ili okruženju, ili white-labeled zakupci), naplaćuje se samo roditeljski nalog. Korišćenje svakog pod-naloga se pojavljuje kao sopstvene stavke na Flex fakturi roditeljskog naloga, sa prefiksom imena pod-naloga, a pod-nalozi dele roditeljski način plaćanja i status naplate.
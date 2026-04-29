Agentovi webhooki se ob napaki ponovno poskušajo. Dostava je **pošlji in pozabi z vidika agenta** - neuspešna dostava ne blokira izvajanja agenta ali ne razveljavi nobenih dejanj - ponovitve pa asinkrono upravlja vrsta + cron.

### Model vrste

Vsak dogodek je vstavljen v vrsto **enkrat za vsak ujemajoči webhook**. Če imate tri webhooke naročene na `trigger.succeeded` za določenega agenta + domeno, platforma v vrsto doda tri dostave; vsaka se dostavlja in ponavlja neodvisno. Napaka na enem webhooku nikoli ne vpliva na druge.

### Kaj se poskuša znova

Dostava se poskusi znova, kadar:

- HTTP zahteva **se ne dokonča** (napaka DNS, zavrnjena povezava, potekel čas).
- HTTP odzivna koda je katera koli ne-2xx vrednost, ki ni na konfiguriranem seznamu **Kode brez ponovitve**.

Dostava se **ne poskuša znova**, ko:

- Odzivna koda je `2xx` (uspeh).
- Odzivna koda je na konfiguriranem seznamu **Kode brez ponovitve**. Privzeto je ta seznam prazen - katerakoli ne-2xx koda se ponovno poskusi.

### Konfiguracija kod brez ponovitve

Obrazec za konfiguracijo webhooka ima polje **Kode brez ponovitve** (več vrednosti). Pogoste vnose:

- `410` - Gone. Vaša končna točka je trajno premaknjena ali vir ne obstaja več. Ponovni poskus bi le zapravili pasovno širino obeh strani.
- `422` - Unprocessable Entity. Vaša končna točka je razumela vsebino, vendar jo je označila za neveljavno. Ponovno pošiljanje iste vsebine bo vrnilo enak odgovor.
- `400` - Bad Request, v istem duhu.

Dodajanje kode sem pomeni: ko jo končna točka vrne, označi dostavo kot terminalno neuspešno in prenehaj z njenim ponavljanjem.

### Razpored ponavljanja

Ozadni delovni proces teče vsako nekaj sekund in obdeluje vse dostave, katerih čas naslednjega poskusa je potekel.

Po vsaki napaki je čas naslednjega poskusa potisnjen naprej z **linearnim upočasnjevanjem**: čakanje raste kot `60 seconds * attempt count` (torej poskus 1 počaka 1 minuto, poskus 2 počaka 2 minuti itd.).

Po 99 neuspelih poskusih (ali 3 v lokalnem razvoju) se od dostave obupa in se odstrani iz vrste. Vnosi v dnevniku dostave pa ostanejo in so vidni na strani [Dnevniki dostave webhookov](#webhook-logs) dokler ne potečejo.

### Idempotentnost na vaši strani

Ker ponavljamo, mora biti vaša končna točka **idempotentna**. Isti `triggerId` (ali `approvalId`) lahko prispe večkrat. Vaša končna točka naj:

- Uporabi edinstven ključ (`triggerId` za sprožilne dogodke, `approvalId` za dogodke odobritve) kot žeton za deduplikacijo.
- Sprejme podvojene dostave brez težav (ob drugi dostavi vrnite 200).

Neidempotentna končna točka bo na koncu dvakrat obdelala nekatere dostave, zlasti med začasnimi izpadi, ko en poskus poteče, se ponovi čez 30 sekund, medtem ko je originalna zahteva dejansko uspela.

### Zaporedje

Dostave niso **strogo urejene**. `trigger.succeeded` in sledi `approval.requested` (iz istega zagona) se lahko pojavita v katerem koli vrstnem redu, če se eden ponovi, drugi pa ne. Vaša končna točka ne sme predpostavljati kauzalnega vrstnega reda.

Če potrebujete vrstni red, uporabite časovne žige - `occurredAt` na ovojnici, plus `createdAt` sprožilca/odobritve v podatkovnem bloku - za rekonstruiranje vrstnega reda na vaši strani.

### Čiščenje

Dostave se odstranijo iz vrste takoj, ko uspejo ali dosežejo omejitev poskusov. Platforma v vrsti ne hrani terminalno neuspelih dostav; trajen zapis vsakega poskusa je na strani [Dnevniki dostave webhookov](#webhook-logs).

### Kje iskati, ko ponovitve ne uspejo

Stran [Dnevniki dostave webhookov](#webhook-logs) je mesto, kjer vidite, zakaj webhook ne deluje. Pogosti vzroki:

- **Napaka pri reševanju DNS** - URL je napačen ali domena ne obstaja več.
- **Napake TLS** - certifikat vaše končne točke je neveljaven ali potekel.
- **Povezava zavrnjena / potekel čas** - vaša končna točka je nedosegljiva.
- **Odgovori 5xx** - vaša končna točka deluje, vendar je prišlo do napake. Telo odgovora (skrajšano) je zabeleženo.
- **Odgovori 4xx** - vaša končna točka je zavrnila vsebino. Če je to namerno, dodajte kodo v **Kode brez ponovitve**.

### Začasno onemogočanje nezdravega webhooka

Če webhook stalno ne deluje, je najčistejša rešitev, da ga izbrišete (ali začasno počistite njegov seznam naročnin na dogodke). Platforma ne onemogoča samodejno neuspešnih webhookov - ti se nadaljujejo s ponavljanjem, dokler se ne opusti dostave.
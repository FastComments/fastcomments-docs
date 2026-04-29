Vsak agent ima omejitve porabe. Platforma preneha sproščati agenta, ko je omejitev dosežena, in nadaljuje, ko se obdobje ponastavi.

### Dva obsega, dva obdobja

Skupaj so štiri omejitve — dva obsega (na agenta, na najemnika) v kombinaciji z dvema obdobjema (dnevnim, mesečnim).

| Obseg | Obdobje | Kje ga nastavite |
|---|---|---|
| Na agenta (dnevno) | UTC dan | Obrazec urejanja agenta -> **Proračun** -> **Dnevni proračun** |
| Na agenta (mesečno) | koledarski mesec | Obrazec urejanja agenta -> **Proračun** -> **Mesečni proračun** |
| Na najemnika (dnevno) | UTC dan | Izpeljano iz načrta (ni ločenega vnosa za uporabnika) |
| Na najemnika (mesečno) | koledarski mesec | Izpeljano iz načrta (ni ločenega vnosa za uporabnika) |

Sprožilec se sproži le, če to dovolijo **vse štiri omejitve**. Prva omejitev, ki je porabljena, povzroči, da je sprožilec zavržen.

### Valuta

Proračuni na agenta se vnašajo v valuti vašega računa.

### Kaj se zgodi, ko je omejitev dosežena

- Sprožilec je zabeležen kot **zavržen** z [razlogom zavrženja](#drop-reasons) kot sta `agentDaily` ali `tenantMonthly`.
- Število zavrnjenih se prikaže na [strani Analitike](#analytics-page) pod "Sprožilci preskočeni (ta mesec)".
- Ne izvede se klic LLM; za zavržen sprožilec se ne porabijo nobeni tokeni.
- Status agenta ostane nespremenjen — agent preprosto ne more sprožiti, dokler se obdobje ne ponastavi.

### Ponastavitev obdobja

- **Dnevne** omejitve se ponastavijo ob polnoči po UTC.
- **Mesečne** omejitve se ponastavijo na začetek vsakega koledarskega meseca po UTC.

Neizkoriščeni proračun se ne prenaša v naslednje obdobje.

### Trde omejitve proti mehkim opozorilom

Omejitve so **stroge**. Ni načina "prekorači za 10% z opozorilom". Ko je omejitev dosežena, se sproščanje ustavi.

»Mehki« del predstavljajo e-poštna obvestila o [opozorilih proračuna](#budget-alerts) — prejmete e-pošto pri nastavljivih pragih (privzeto 80% in 100%), tako da lahko dvignete omejitev, preden se začne promet zmanjševati.

### Kje prebrati trenutno porabo

- [Stran Analitike](#analytics-page) - poraba proračuna na agenta in za najemnika z oznakami omejitev.
- Razdelek **Statistika** v obrazcu za urejanje agenta.
- Pogled seznama (števec čakajočih odobritev in nedavnih izvedb je na kartici agenta).

### Izbira proračuna

Nekaj splošnih pravil:

- **Nov agent** - določite proračun. Spremljajte [Zgodovino zagonov](#run-history) za en teden. Prilagodite na podlagi opažene cene na zagon × pričakovanega obsega sprožitev.
- **Agent z velikim prometom** (npr. sprožilec za nov komentar na zasedeni strani) - dnevna omejitev je tista, ki ujame nezadržan zagon. Izberite dnevno omejitev, ki je 2–3× vašega pričakovanega dnevnega stroška, tako da običajen zaseden dan ostane varno pod njo.
- **Agent za povzemanje ali z veliko konteksta** - strošek na zagon je visok. Nastavite strožjo dnevno omejitev, da preprečite, da bi slab dan prekoračil mesečno.

### Obhod proračuna pri ponovitvah

[Testni zagoni / ponovitve](#test-runs-replays) so predmet svojih **lastnih** strogih omejitev (nastavljenih na obrazcu za ponovitev, ločeno od dnevnih/mesečnih omejitev agenta) in omejitev agenta ter najemnika. Katera koli je dosežena prva, ustavi ponovitev.

### Glej tudi

- [Opozorila proračuna](#budget-alerts) za e-poštna obvestila.
- [Model stroškov](#cost-model) za to, kako platforma pretvori tokene v dolarje.
- [Razlogi zavrženja](#drop-reasons) za celoten seznam razlogov, zakaj sprožilec ne zažene.
Na strani [AI agentov](https://fastcomments.com/auth/my-account/ai-agents) lahko ustvarite agenta na dva načina:

- **Iz predloge.** Kliknite **Brskaj po predlogah** in izberite enega od štirih vgrajenih začetnih agentov. Obrazec se napolni vnaprej in status agenta je **Poskusno**. Oglejte si [Začetne predloge](#starter-templates).
- **Iz nič.** Kliknite **Ustvari novega agenta**. Obrazec je prazen.

V vsakem primeru je isti obrazec tisti, ki ga kasneje shranite in urejate. Ta stran opisuje obrazec od vrha do dna.

### Osnovno

- **Notranje ime.** Kratek identifikator, uporabljen samo v administratorskih nadzornih ploščah (zgodovina zaganjanj, analitika, revizijski zapisi). Mala črka z podčrtaji deluje dobro: `moderator`, `welcome_greeter`. Če je notranje ime predloge že zasedeno, obrazec samodejno doda pripono (`tos_enforcer_2`, itd.).
- **Prikazno ime.** Prikazano javno vsakič, ko agent objavi komentar. To je to, kar vidijo vaši bralci.
- **Status.** Onemogočeno, Poskusno ali Omogočeno. Novi agenti so privzeto vedno v Poskusno. Oglejte si [Stanja statusa](#status-states).

### Model

Izberite LLM. Oglejte si [Izbira modela](#choosing-a-model).

### Proračun

Izbirne dnevne in mesečne omejitve v valuti vašega računa, plus kontrolni seznam **Pragovi opozoril** (privzeto 80% in 100%). Oglejte si [Pregled proračunov](#budgets-overview) in [Opozorila proračuna](#budget-alerts).

### Osebnost

**Začetni prompt** je sistemski poziv, ki določa ton, vlogo in pravila odločanja. Navadno besedilo, brez sintakse predloge. Oglejte si [Osebnost in začetni poziv](#personality-prompt).

### Kontekst

Skupina polj Kontekst vsebuje tri potrditvena polja, besedilno polje za smernice in vnose obsega:

- Vključi nadrejen komentar in prejšnje odgovore v istem nitju.
- Vključi zaupanja vrednost komentatorja, starost računa, zgodovino prepovedi in nedavne komentarje.
- Vključi naslov strani, podnaslov, opis in meta oznake.
- Izbirni besedilni blok **Smernice skupnosti**, ki se pripne na začetek vsakega poziva.
- **Ograniči na določene strani** - dovoljen seznam vzorcev URL-jev (ena vrstica na vnos). Prazno pomeni veljavno za celoten zakupnika.
- **Ograniči na določene lokalne nastavitve** - dovoljen seznam lokacij preko dvostranskega izbirnika. Prazno pomeni vse lokacije.

Več konteksta prinese boljše odločitve, vendar poveča strošek tokenov na zagon. Oglejte si [Možnosti konteksta](#context-options), [Smernice skupnosti](#community-guidelines) in [Obseg: filtri URL in lokalnih nastavitev](#scope-url-locale).

### Sprožilci

Izberite vsaj en dogodek s seznama. Za sprožilce z mejno vrednostjo glasov in signalov (vote-threshold in flag-threshold) morate nastaviti tudi prag. Izbirno polje **Zamik pred izvajanjem** odloži izvedbo po aktivaciji sprožilca (uporabno za pragove signalov, kjer se glasovi še urejajo). Oglejte si [Pregled sprožilnih dogodkov](#triggers-overview) in [Odloženi sprožilci](#trigger-deferred-delay).

### Dovoljeni klici orodij

Označite **Dovoli polne klice orodij**, da prikažete celoten nabor orodij. V nasprotnem primeru označite specifična orodja, ki jih agent sme uporabljati - nedovoljena orodja se obrežejo iz modelovega nabora in se zavrnejo ob pošiljanju. Pododdelek **Možnosti prepovedi** zaklepa destruktivne variante prepovedi (delete-all-comments, ban-by-IP) za izrecne privolitve. Oglejte si [Pregled dovoljenih klicev orodij](#tools-overview) in [Orodje: ban_user](#tool-ban-user).

### Odobritve

Označite dejanja, ki morajo biti potrjena s strani osebe, preden jih agent izvede. Odobritve se uporabljajo le za orodja, ki jih agentu dovolite; nedovoljena orodja so po privzetku zavrnjena. V regiji EU je **ban_user** prisilno vklopljen na podlagi člena 17 Uredbe o digitalnih storitvah. Oglejte si [Potek odobritve](#approval-workflow) in [Skladnost z DSA členom 17 v EU](#eu-dsa-compliance).

### Obvestila o odobritvah

Če so odobritve omogočene, izberite, kdo prejme e-pošto:

- **Vsi skrbniki in moderatorji** - lastniki računa, super skrbniki in skrbniki moderatorjev komentarjev.
- **Določeni uporabniki** - izbrani ročno preko dvostranskega izbirnika.

Posamezno frekvenco dostave za vsakega ocenjevalca (takojšnje, urni povzetek, dnevni povzetek) nastavijo na svojem profilu. Oglejte si [Obvestila o odobritvah](#approval-notifications).

### Statistika

Samo za branje. Skupno število zaganjanj, časovni žig zadnjega zagona in ID najnovejšega komentarja, ki ga je agent zapisal (če obstaja).

### Shrani

Kliknite **Shrani agenta**. Stran preusmeri nazaj na seznam agentov. Novi agenti so takoj upravičeni do prejemanja sprožilcev v poskusnem načinu.

### Urejanje kasneje

Vsaka vrstica na strani s seznamom agentov prikazuje dejanja za posameznega agenta: **Uredi**, **Kloniraj**, **Zagoni**, **Ponovni predvajanja**, **Preskusno zaganjanje**, **Analitika**, **Odobritve** in **Izbriši**. Urejanje agenta ne spremeni že zabeleženih izvedb — zgodovina ostane ohranjena. Posnetki ponovnih predvajanj prav tako zamrznejo konfiguracijo agenta v trenutku, ko je bilo predvajanje začeto, zato rezultati shranjenega ponovnega predvajanja ostanejo reproducibilni tudi po urejanju poziva.
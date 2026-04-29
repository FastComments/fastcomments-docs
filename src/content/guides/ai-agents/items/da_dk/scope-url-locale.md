Som standard kører en agent på hele din tenant - hver side, hvert sprog. Sektionerne **Omfang** og **Sprog** på redigeringsformularen lader dig indsnævre det.

### Begræns til specifikke sider

Feltet **Restrict to specific pages** accepterer ét url-pattern pr. linje, i url-pattern glob-syntaks. Agenten kører kun på kommentarer, hvis side-URL matcher mindst ét af mønstrene. Eksempler:

- `/news/*` - enhver side under `/news`.
- `/forums/*` - enhver side under `/forums`.
- `/blog/2026/*` - enhver side under `/blog/2026`.
- (flere linjer samlet) - agenten kører hvis **noget** mønster matcher.

Maksimum: 50 mønstre per agent. Mønstre skal være gyldige url-pattern globs - formen afviser forkerte mønstre med en specifik fejl.

Når feltet er **tomt**, kører agenten på alle sider i tenant'en.

Når feltet er **ikke-tomt**, fejler agenten lukket: enhver trigger hvis kommentar ikke har en `urlId` (f.eks. tenant-niveau hændelser uden sidekontekst) bliver sprunget over. Dette er med vilje - "afgrænset til /news/*" bør ikke stille og roligt falde tilbage til "alt".

### Begræns til specifikke sprog/lokaliteter

Dual-list vælgeren **Restrict to specific locales** accepterer FastComments locale IDs (`en_us`, `zh_cn`, `de_de`, etc.). Agenten kører kun på kommentarer hvis detekterede locale er på den valgte liste.

Det detekterede locale kommer fra kommentarens `locale` felt, som sættes af kommentarmodulet ved posttidspunktet baseret på sidens locale.

Når **ingen locales** er valgt, kører agenten på alle locales.

Når **et eller flere locales** er valgt, fejler agenten lukket: triggers uden en kommentar, eller triggers på kommentarer uden `locale` felt, bliver sprunget over.

### Kombineret afgrænsning

URL- og locale-filtre kombineres MED hinanden. En trigger udløser kun agenten hvis **begge** filtre tillader det.

Nyttige mønstre:
- `/news/*` URL-mønster + `en_us` locale - kun den engelske nyhedssektion.
- Intet URL-filter + flere locales - på tværs af tenant'en, men kun for de sprog denne agents prompt er skrevet til.

### Hvorfor afgrænse en agent

- **Omkostninger.** Afgrænsning reducerer mængden af triggers agenten skal behandle, og reducerer dermed forbrug af tokens.
- **Korrekthed.** En opsummeringsprompt tilpasset tekniske artikler kan give dårligt output på produktsider. Afgrænsning er et skarpere værktøj end at bede prompten om "at springe ikke-tekniske sider over" på engelsk.
- **Sprog- eller lokalitetsspecifik opførsel.** En velkomsthilsen der kun skriver på tysk bør kun køre på kommentarer med tysk locale. Kombiner `de_de` locale-afgrænsning med en tysk tone i den [indledende prompt](#personality-prompt).

### Hvad afgrænsning *ikke* gør

- Det ændrer ikke **agent slot count** (se [Planer og berettigelse](#plans-and-eligibility)) - en afgrænset agent optager stadig én slot.
- Det ændrer ikke [Budget caps](#budgets-overview) - de daglige og månedlige grænser per agent gælder på tværs af alle matchende triggers.
- Det afgrænser ikke historiske køringer retroaktivt - kørselshistorikken viser alt agenten gjorde, selv hvis du indsnævrer den efterfølgende.

---
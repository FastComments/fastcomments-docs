Budget-advarsels-e-mails sendes, når en agents forbrug krydser en konfigurerbar procentdel af dens grænse. De sendes til de personer, der ejer regningen.

### How alerts work

Hver agent har et felt **Alert thresholds** på redigeringsformularen. Standardværdierne er `80%` og `100%`. Du kan afkrydse eller fjerne kryds ved individuelle thresholds, og du kan tilføje andre procenter.

Når agentens forbrug i et givet scope (dagligt eller månedligt) krydser en threshold for første gang i den periode, sender platformen én e-mail per modtager. At krydse threshold igen senere i samme periode (f.eks. forbruget faldt under 80% og steg over igen) genudsender **ikke** e-mailen.

Dette gælder per periode: en ny daglig nulstilling starter threshold-krydse-logikken for den dag.

### Tenant-scope alerts

Tenanten (kontoen) har sine egne daglige og månedlige grænser. Tenant-scope alerts udløses ved faste thresholds (`80%` og `100%`). Disse kan ikke konfigureres per agent, fordi de gælder for hele tenant'en.

### Recipients

Budget-advarsler sendes til:

- Enhver bruger markeret **Super admin** på tenant'en.
- Enhver bruger markeret **Billing Admin** på tenant'en.

Det inkluderer foreningen af de to roller - en bruger med begge roller modtager én e-mail.

### Why both roles

Super admins er typisk de operatører, som har brug for at vide, når en agent rammer sin grænse. Billing admins ejer fakturaen og skal kende til omkostningsspidser uanset om de administrerer agenter til daglig. For rent faktisk at redigere agenten (hæv grænsen, sæt den på pause) har modtageren også brug for rollen **Customization Admin** - som beskytter agent-redigeringssiden.

### Per-user opt-out

Modtagere, der har frameldt sig admin-notifikationer på deres profil, springes over. Dette er den samme frameldingsknap, der styrer andre admin-notifikationer.

Hvis **alle** modtagere er frameldt, logges advarslen (advarselsniveau) og der sendes ingen e-mail.

### Email content

E-mailen indeholder:

- Agentens **display name** og interne navn.
- Det **scope**, der blev krydset (f.eks. "agent daily budget", "agent monthly budget", "account daily budget", "account monthly budget").
- Den krydsede **threshold percentage**.
- **Usage** i tenant'ens valuta.
- **Cap** i tenant'ens valuta.
- Et **one-click signed login link**, som tager modtageren direkte til:
  - Agent-redigeringssiden, for agent-scope alerts.
  - AI Agents list page, for tenant-scope alerts.

Linket er præ-autentificeret, så modtageren er kun ét klik fra at hæve grænsen eller deaktivere agenten.

### How thresholds fire

Platformen sporer, hvilke thresholds der allerede er udløst denne periode, separat for agenten og tenant'en. Så:

- At krydse 80% og derefter 100% i samme periode udløser begge, i rækkefølge.
- At gå direkte fra 0% til 100% i ét stort hop udløser den **højeste** krydsede threshold (100%), ikke 80%, så den mest alvorlige advarsel er den, der sendes.

### When you stop getting alerts

Hvis agentens forbrug aldrig når den næste threshold i denne periode, modtager du ikke flere e-mails i perioden. Næste daglige nulstilling (eller månedlige nulstilling) rydder sporingstavlen.

### Disabling alerts

Fjern krydset ved den threshold, du ikke ønsker. Hvis du ikke ønsker nogen advarsler for en specifik agent, fjern krydset ved alle procenter. Tenant-scope alerts kan ikke deaktiveres per agent (de er tenant-wide).

### See also

- [Budgets Overview](#budgets-overview).
- [Drop Reasons](#drop-reasons) - hvad der sker, når grænsen er fuldt nået.
- [Cost Model](#cost-model) - hvad der måles.
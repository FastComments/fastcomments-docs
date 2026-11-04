Budgetadvarsels-e-mails udsendes, når en agents forbrug overskrider en konfigurerbar procentdel af dens grænse. De sendes til de personer, der ejer regningen.

### Sådan fungerer advarsler

Hver agent har et felt **Advarselstærskler** på redigeringsformularen. Som standard er det `80%` og `100%`. Du kan markere eller fjerne markeringen af individuelle tærskler, og du kan tilføje andre procenter.

Når agentens forbrug i et givet scope (dagligt eller månedligt) overskrider en tærskel for første gang i den periode, sender platformen én e-mail pr. modtager. At overskride tærsklen igen senere i samme periode (f.eks. hvis forbruget faldt under 80% og kom over igen) udløser **ikke** en ny e-mail.

Dette er pr. periode: en ny daglig nulstilling starter tærskel-logikken for den pågældende dag forfra.

### Advarsler på tenant-niveau

Tenanten (kontoen) har sine egne daglige og månedlige grænser. Advarsler på tenant-niveau udløses ved faste tærskler (`80%` og `100%`). Disse kan ikke konfigureres pr. agent, fordi de gælder for hele tenanten.

### Modtagere

Budgetadvarsler sendes til:

- Alle brugere, der er markeret som **Super admin** på tenanten.
- Alle brugere, der er markeret som **Billing Admin** på tenanten.

Det inkluderer foreningen af de to roller - en bruger med begge roller modtager én e-mail.

### Hvorfor begge roller

Super admins er typisk de operatører, der har brug for at vide, hvornår en agent nærmer sig sin grænse. Billing-admins ejer fakturaen og har brug for at vide, om der er omkostningsspidninger, uanset om de administrerer agenter til daglig. For faktisk at redigere agenten (hæve grænsen, sætte den på pause) kræver det dog også rollen **Customization Admin** - som styrer adgangen til agentens redigeringsside.

### Framelding pr. bruger

Modtagere, der har meldt sig fra admin-notifikationer på deres profil, springes over. Det er den samme frameldingsknap, der styrer andre admin-notifikationer.

Hvis **alle** modtagere har meldt sig fra, logges advarslen (på advarselsniveau), og der sendes ingen e-mail.

### E-mail-indhold

E-mailen indeholder:

- Agentens **viste navn** og interne navn.
- Det **scope**, der blev overskredet (f.eks. "agent daily budget", "agent monthly budget", "account daily budget", "account monthly budget").
- Den overskredne **tærskelprocent**.
- **Forbrug** i tenantens valuta.
- **Grænse** i tenantens valuta.
- Et **one-click signeret login-link**, der fører modtageren direkte til:
  - Agentens redigeringsside, for advarsler på agent-niveau.
  - Listen over AI-agenter, for advarsler på tenant-niveau.

Linket er forudautentificeret, så modtageren kun er ét klik fra at hæve grænsen eller deaktivere agenten.

### Sådan udløses tærskler

Platformen registrerer, hvilke tærskler der allerede er udløst i denne periode, separat for agenten og tenanten. Så:

- At overskride 80% og derefter 100% i samme periode udløser begge, i rækkefølge.
- At gå direkte fra 0% til 100% i ét stort spring udløser den **højeste** overskredne tærskel (100%), ikke 80%, så det mest alvorlige advarsel er den, der leveres.

### Når du holder op med at modtage advarsler

Hvis agentens forbrug aldrig når den næste tærskel i denne periode, modtager du ikke flere e-mails i denne periode. Den næste daglige nulstilling (eller månedlige nulstilling) rydder registreringen.

### Deaktivering af advarsler

Fjern markeringen af den tærskel, du ikke ønsker. Hvis du ikke vil have nogen advarsler på en bestemt agent, fjerner du markeringen af alle procenter. Advarsler på tenant-niveau kan ikke deaktiveres pr. agent (de gælder for hele tenanten).

### Se også

- [Budgets Overview](#budgets-overview).
- [Drop Reasons](#drop-reasons) - hvad der sker, når grænsen er fuldt nået.
- [Cost Model](#cost-model) - hvad der måles.
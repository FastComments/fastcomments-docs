[related-parameter-start name = 'customCSS'; type = 'string'; related-parameter-end]

FastComments er designet til at kunne tilpasses. Kommentar-widget'en kører i en iframe af sikkerhedsmæssige årsager, så for at anvende brugerdefineret styling skal du følge én af to fremgangsmåder.

Den første, den letteste fremgangsmåde, og den vi foretrækker, er at bruge [widget customization page](https://fastcomments.com/auth/my-account/customize-widget).

På widget-tilpasningssiden, se sektionen "Show Advanced Options", hvor der er et område mærket "Custom CSS":

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.show-advanced-option'; selector = '.custom-css'; alt='Custom CSS-editor under Vis avancerede indstillinger på widget-tilpasningssiden'; title='Custom CSS-indtastningsområde' app-screenshot-end]

Denne fremgangsmåde har nogle fordele:
1. Den indtastede CSS minimeres, før den sendes til brugeren, og formateringen holdes konsistent i redigerings‑UI’et.
2. Du får alle fordelene ved widget‑tilpasnings‑UI’et, f.eks. nem tilpasning af kommentar‑widget’en forskelligt for forskellige sites.
3. Når vi laver ændringer i kommentar‑widget’en, vil din brugerdefinerede styling blive testet som en del af vores udgivelsesproces.

Den anden fremgangsmåde er at angive **customCSS**‑parameteren i widget‑konfigurationen, som følger:

[code-example-start config = {customCSS: "button { background: red; }" }; linesToHighlight = [6]; title = 'Overførsel af custom CSS'; code-example-end]

Dette har dog *begrænsninger*:
1. Der er en grænse for, hvor meget custom CSS der kan sendes, før vores servere afviser anmodningen, på grund af header‑størrelsen.
2. Du skal selv håndtere den brugerdefinerede CSS i din infrastruktur og dit build‑system. Dette kan også være en fordel snarere end en ulempe.
3. Der er en ekstra overhead ved at sende den brugerdefinerede CSS over netværket **to gange** i dette tilfælde, da den først skal sendes til vores servere og derefter sendes tilbage i iframe‑indholdet. For de fleste payload‑størrelser er dette dog ikke mærkbart.
4. En almindelig optimering er at minimere CSS’en for at reducere dens størrelse over netværket, men med denne fremgangsmåde skal du selv håndtere det.
5. Din brugerdefinerede CSS vil ikke blive testet, når vi laver ændringer.

### Eksterne CSS-filer

Du kan fortælle widget'en at hente en ekstern fil ved at bruge `@import`!

Det anbefales at placere `@import` i en tilpasningsregel. På den måde kan vi, hvis vi nogensinde skal foretage en ændring i kommentar‑widget'en, bruge vores automatiserings‑værktøjer til at verificere din opsætning. Så for eksempel ville du oprette en tilpasningsregel i Widget‑Customization‑UI’et, klikke på `Advanced` og indtaste i `Custom CSS`:

    @import url(https://example.com/styles.css);

#### I kode - Ikke anbefalet

Du kan også indlæse en ekstern CSS‑fil via `customCSS`‑egenskaben:

[code-example-start config = {customCSS: "@import url(https://example.com/styles.css);" }; linesToHighlight = [6]; title = 'Ekstern CSS-fil'; code-example-end]

Men husk, at din CSS ikke vil kunne blive testet af os, hvis du gør dette.

### Styling af brugerprofilmodal

Brugerprofil‑modaler kan også styles med custom CSS. For at sikre, at brugerdefineret styling anvendes på brugerprofiler, skal alle CSS‑selektorer være forudgået af `.user-profile`. Uden dette præfiks vil brugerdefineret styling blive ignoreret for brugerprofil‑modaler.

For eksempel:

[code-example-start config = {customCSS: ".user-profile .profile-name { color: blue; }" }; title = 'Brugerprofil CSS'; code-example-end]

### Bagudkompatibilitet

Hos FastComments ved vi, at vores kunder tilpasser kommentar‑widget'en. Det er med vilje – det sidste, vi ønsker, er at vores produkt forårsager design‑inkonsistens i dit produkt.

Da dette er en vigtig del af vores produkt, har vi en build‑pipeline, der gør det muligt for os at gennemgå ændringer i kommentar‑widget'en per kunde ved hver udgivelse.

Hvis vi finder mindre problemer, vil vi opdatere din konto for at sikre, at udgivelsen forløber glat. Hvis vi ser større, kritiske ændringer, giver dette os mulighed for at stoppe udgivelsen.
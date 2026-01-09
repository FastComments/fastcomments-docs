[related-parameter-start name = 'customCSS'; type = 'string'; related-parameter-end]

FastComments er designet til at kunne tilpasses. Selve kommentars-widgeten kører inde i et iframe af sikkerhedsmæssige årsager, så for at anvende brugerdefineret styling skal du følge en af to tilgange.

Den første, nemmeste tilgang, og den vi foretrækker, er at bruge [widget customization page](https://fastcomments.com/auth/my-account/customize-widget).

På widget-tilpasningssiden skal du se afsnittet "Show Advanced Options", under hvilket der er et område mærket "Custom CSS":

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.show-advanced-option'; selector = '.custom-css'; title='Custom CSS Input Area' app-screenshot-end]

Denne tilgang har nogle fordele:
1. Den indtastede CSS minificeres, før den sendes til brugeren, og formateringen holdes konsistent i redigerings-UI'en.
2. Du får alle fordelene ved widget-tilpasnings-UI'en, for eksempel nemt at tilpasse kommentars-widgeten forskelligt for forskellige sites.
3. Når vi laver ændringer til kommentars-widgeten, vil din brugerdefinerede styling blive testet som en del af vores release-proces.

Den anden tilgang er at angive parameteren **customCSS** i widget-konfigurationen, som følger:

[code-example-start config = {customCSS: "button { background: red; }" }; linesToHighlight = [6]; title = 'Passing Custom CSS'; code-example-end]

Dette har dog *begrænsninger*:
1. Der er en grænse for, hvor meget brugerdefineret CSS der kan sendes, før vores servere afviser forespørgslen, på grund af header-størrelsen.
2. Du skal administrere den brugerdefinerede CSS i din infrastruktur og dit build-system. Dette kan også være en fordel frem for en ulempe.
3. Der er en ekstra overhead ved at sende den brugerdefinerede CSS over netværket **to gange** i dette tilfælde, da den først skal sendes til vores servere og derefter sendes tilbage i iframe-indholdet. For de fleste payload-størrelser er dette dog ikke mærkbart.
4. En almindelig optimering er at minificere CSS'en for at reducere dens størrelse over nettet, men med denne tilgang skal du selv håndtere det.
5. Din brugerdefinerede CSS vil ikke blive testet, når vi laver ændringer.

### External CSS Files

Du kan få widgetten til at hente en ekstern fil ved at bruge `@import`!

Det anbefales at placere `@import` i en tilpasningsregel. På den måde, hvis vi nogensinde får behov for at ændre kommentars-widgeten, kan vi bruge vores automatiseringsværktøjer til at verificere din opsætning. Så for eksempel opretter du en tilpasningsregel i Widget Customization UI'en, klikker `Advanced`, og indtaster i `Custom CSS`:

    @import url(https://example.com/styles.css);

#### In Code - Not Recommended

Du kan også indlæse en ekstern CSS-fil via `customCSS`-egenskaben:

[code-example-start config = {customCSS: "@import url(https://example.com/styles.css);" }; linesToHighlight = [6]; title = 'External CSS File'; code-example-end]

Husk dog, at din CSS ikke vil kunne testes af os, hvis du gør dette. 

### User Profile Modal Styling

Brugerprofilmodaler kan også styles med brugerdefineret CSS. For at sikre, at brugerdefineret styling anvendes på brugerprofiler, skal alle CSS-selektorer dog være præfikset med `.user-profile`. Uden dette præfiks vil brugerdefineret styling blive ignoreret for brugerprofilmodaler.

For eksempel:

[code-example-start config = {customCSS: ".user-profile .profile-name { color: blue; }" }; title = 'User Profile CSS'; code-example-end]

### Backwards Compatibility

Hos FastComments ved vi, at vores kunder tilpasser kommentars-widgeten. Det er med vilje - det sidste, vi ønsker, er at vores produkt skaber designmæssige uoverensstemmelser i dit produkt.

Da dette er en vigtig del af vores produkt, har vi en build-pipeline, der gør det muligt for os at gennemgå ændringer til kommentars-widgeten per kunde ved hver release.

Hvis vi finder mindre problemer, vil vi opdatere din konto for at sikre, at vores release forløber glat. Hvis vi ser større brydende ændringer, gør dette os i stand til at standse releasen.
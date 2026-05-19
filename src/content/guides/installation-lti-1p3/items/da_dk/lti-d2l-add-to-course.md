Denne side dækker tilføjelse af FastComments til et Brightspace-kursus efter en administrator har registreret værktøjet og oprettet en deployment. Hvis værktøjet ikke er registreret endnu, se først D2L-registreringsguiden.

<div class="screenshot white-bg">
    <div class="title">FastComments indlejret som et emne i Brightspace</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-d2l-comments-in-unit.png" alt="FastComments running inside a Brightspace unit, showing threaded comments and an @-mention picker" />
</div>

Brightspace leverer to indholdsoprettelsesoplevelser: **Classic Content** og **New Content Experience** (også kaldet **Lessons**). Begge gør FastComments tilgængeligt, men menupunkterne adskiller sig. Hvert afsnit nedenfor dækker begge, hvor de divergerer.

#### Find FastComments-værktøjet

FastComments-værktøjet vises to steder inde i en kursusindholdseditor:

1. Aktivitetspickeret, som nås fra et modul/units **Add Existing**-knap (mærket **Add Existing Activities** i ældre Brightspace-versioner). FastComments vises direkte i pickeret i nyere Brightspace-builds; ældre versioner ligger det under en **External Learning Tools**-undermenu. Begge stier tilføjer FastComments som et separat emne.
2. **Insert Stuff**-dialogen inde i HTML-editoren, under **LTI Advantage**. Dette indlejrer FastComments inline i et HTML-emne via LTI deep linking-flowet.

Hvis FastComments ikke vises i nogen af pickerne, er deployment ikke aktiveret for den organisatoriske enhed (org unit), der holder kurset. Bed din Brightspace-administrator om at åbne **Admin Tools** > **Manage Extensibility** > **LTI Advantage** > FastComments tool > **View Deployments**, åbne deploymenten og tilføje kursets org unit (eller en overordnet org unit) under **Org Units**.

#### Tilføj FastComments som et emne i et modul

Classic Content:

1. Åbn kurset og klik på **Content** i navigationsbaren.
2. Vælg det modul, der skal indeholde diskussionen (eller opret et via **Add a module**).
3. Klik på **Add Existing** (ældre Brightspace: **Add Existing Activities** > **External Learning Tools**).
4. I pickeret, klik på **FastComments**. Brightspace opretter et emne i modulet og returnerer dig til indholdsvisningen.
5. Klik på det nye emne. Omdøb det til noget beskrivende som `FastComments Discussion` ved hjælp af den inline titelditor.

New Content Experience (Lessons):

1. Åbn kurset og klik på **Content**.
2. Åbn unit og lesson, som skal indeholde diskussionen.
3. Klik **Add** > **Existing Activity** og vælg **FastComments** (ældre Brightspace: indlejret under **External Learning Tools**).
4. Aktiviteten bliver tilføjet til lesson.
5. Klik på aktivitetstitlen for at omdøbe den.

Første gang en bruger (underviser eller studerende) åbner emnet, initialiserer FastComments tråden for det resource link. Tråden er bundet til resource link ID, så omdøbning eller flytning af emnet ændrer ikke hvilken tråd der indlæses.

#### Indlejr FastComments inline i et HTML-emne

Brug dette flow når du ønsker, at kommentarer skal vises under en læsning, video eller andet indhold inde på samme emneside i stedet for som et separat emne.

1. Åbn eller opret et HTML-emne i modulet/lesson.
2. Klik **Edit HTML** for at åbne Brightspace HTML-editoren.
3. Placér cursor hvor kommentartråden skal vises.
4. Klik på **Insert Stuff**-knappen (puslespilsikonet i editorens værktøjslinje).
5. I Insert Stuff-dialogen, rul til **LTI Advantage** og klik **FastComments**.
6. FastComments åbner en deep linking-picker. Bekræft placeringen (standardindstillingerne fungerer til indholdsdiskussioner); klik **Insert** eller **Continue**.
7. Brightspace vender tilbage til HTML-editoren med en pladsholderblok, der repræsenterer LTI-launch. Klik **Save and Close** på emnet.

Når emnet indlæses, erstatter Brightspace pladsholderen med en iframe, der automatisk starter FastComments via LTI. Studerende ser diskussionstråden inline.

Et enkelt HTML-emne kan indeholde flere deep-linked FastComments-indsættelser. Hver indlejring får sin egen tråd, fordi hvert deep link genererer et særskilt resource link ID.

#### Modul-emne vs Inline Quicklink

Vælg tilgangen med et **modul-emne** når:

- Diskussionen er den primære aktivitet for det trin i modulet.
- Du ønsker, at emnet skal vises i Brightspace' indholdsfortegnelse, fuldførelsessporing og Class Progress.

Vælg **inline embed**-tilgangen når:

- Kommentarer skal stå under andet indhold på samme side.
- Du ikke ønsker et separat punkt i indholdsfortegnelsen, der kan spores for fuldførelse.

#### Synlighed, kladde og frigivelsesbetingelser

Et nyt FastComments-emne er som standard synligt for studerende. For at skjule det mens du opsætter det:

1. I indholdseditoren, klik emnets titel (Classic) eller de tre prikker på aktiviteten (New Content Experience).
2. Sæt status til **Draft** (Classic) eller slå **Visibility** fra (New Content Experience).

Kladde-emner er usynlige for studerende. Undervisere og TA'er ser dem stadig med et "Draft"-mærke.

For at begrænse emnet til en specifik gruppe eller sektion:

1. Åbn emnet.
2. Klik emnets titelmenu > **Edit Properties In-place** (Classic) eller **Edit** > **Restrictions** (New Content Experience).
3. Under **Release Conditions**, klik **Create**.
4. Vælg **Group enrollment** eller **Section enrollment**, vælg gruppen/sektionen, og gem.

Frigivelsesbetingelser stables oven i FastComments' egen rolle-mapping. Studerende som ikke kan se emnet får ikke en LTI-launch.

#### Hvad studerende ser ved første launch

Når en studerende klikker emnet (eller indlæser et HTML-emne med en indlejring):

1. Brightspace udfører LTI 1.3-launch i baggrunden.
2. FastComments modtager den studerendes navn, e-mail, avatar-URL og LMS-rolle, og logger dem automatisk ind. Der er ingen FastComments-loginprompt.
3. Kommentartråden for det resource link gengives inde i Brightspace-iframe'en.

Rolle-mapping ved launch:

- Brightspace `Administrator` bliver en FastComments **admin** for tråden (fuld moderation, sletning, udelukkelse og konfigurationsadgang).
- Brightspace `Instructor` bliver en FastComments **moderator** (pin, skjul, slet, udeluk).
- Alle andre roller (`Learner`, `TeachingAssistant`, osv.) bliver almindelige kommentatorer.

Kommentarer tilskrives den studerendes Brightspace-konto. Hvis den studerende ændrer deres navn eller avatar i Brightspace, synkroniserer næste LTI-launch ændringen.

#### Luk offentlig adgang (anbefalet)

Som standard er FastComments-kommentardata offentligt læsbare. Enhver, der kan gætte en tråds URL eller API-endpoint, kan se dens kommentarer, også uden for Brightspace. For kursusdiskussioner vil du næsten helt sikkert begrænse visningen til kun indskrevne deltagere.

Åbn din <a href="https://fastcomments.com/auth/my-account/customize-widget" target="_blank">widget-tilpasningsside</a> og opret en regel med **Require SSO To View Comments** aktiveret, og sæt sikkerhedsniveauet til **Secure SSO**, så tråde kun kan indlæses gennem den signerede LTI-launch.

Se [Protecting Comment Threads With Single-Sign-On](/guide-customizations-and-configuration.html#sso-require-to-view-comments) for den fulde gennemgang, inklusive hvordan du afgrænser reglen til et enkelt domæne eller en side.

#### Iframe-højde og resize

FastComments udsender `org.imsglobal.lti.frameResize` postMessage ved hver trådgengivelse og ved indholdsændringer (ny kommentar, udvid svar). Brightspace lytter efter denne besked og justerer iframe-højden, så tråden ikke bliver klippet og ikke viser en indre scrollbar.

Hvis iframe'en forbliver på en fast lav højde:

- Bekræft at kurset indlæses over HTTPS. Brightspace' postMessage-listener afviser mixed-content frames.
- Bekræft at ingen browserudvidelse blokerer postMessage-kanalen.
- For inline-indlejringer i et HTML-emne må den omgivende HTML ikke pakke iframe'en ind i en container med fast højde. Fjern enhver inline `style="height: ..."` fra parent-elementet.

#### Brightspace-specifikke problemer

**Værktøj vises ikke i Add Existing-pickeret.** Deployment er ikke aktiveret for dette kursus' org unit. En administrator skal tilføje org unit (eller en overordnet) til deploymentens **Org Units**-liste. Værktøjsregistrering alene er ikke nok; deployment bestemmer hvilke kurser der ser værktøjet.

**`deployment_id` mismatch ved launch.** FastComments TOFU-pinner det første `deployment_id` det ser for en registrering. Hvis en administrator sletter den oprindelige deployment og opretter en ny, afvises launches fra den nye deployment med en deployment mismatch-fejl. Løsningen er at genregistrere FastComments (generer en ny registrerings-URL (<a href="https://fastcomments.com/auth/my-account/lti-config" target="_blank">hent den her</a>) og kør Dynamic Registration igen); den gamle konfigurationspost bliver erstattet.

**Værktøjet starter men viser "Invalid LTI launch".** Kurset er i en anden tenant/org-struktur end deploymenten dækker, eller deploymenten blev deaktiveret efter registrering. Tjek igen **Admin Tools** > **Manage Extensibility** > **LTI Advantage** > FastComments > **Enabled**-toggle og deploymentens org unit-liste.

**Navne og roller mangler i FastComments.** Brightspace sender LTI-launches med Names and Role Provisioning Services (NRPS) claims. Hvis et kursus blev opgraderet fra et ældre LTI 1.1-link, mangler launch `name` og `email` claims. Tilføj FastComments-emnet igen via **Add Existing** (migrer ikke det gamle link), så launch bruger LTI 1.3.

**Indlejringen viser en login-skærm i stedet for auto-SSO.** HTML-emnet blev indsat som en almindelig <iframe> pegende direkte på FastComments i stedet for via **Insert Stuff** > **LTI Advantage**. Almindelige iframes springer LTI-launch over og lander brugere på den offentlige FastComments-side. Slet iframe'en og indsæt den igen via Insert Stuff-flowet.
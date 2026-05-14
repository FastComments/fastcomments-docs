Denne side beskriver, hvordan man tilføjer FastComments til et Brightspace-kursus, efter en administrator har registreret værktøet og oprettet en deployment. Hvis værktøjet ikke er registreret endnu, se først D2L-registreringsguiden.

<div class="screenshot white-bg">
    <div class="title">FastComments indlejret som et enhedsemne i Brightspace</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-d2l-comments-in-unit.png" alt="FastComments kørende inde i en Brightspace-enhed, vist trådede kommentarer og en @-mention-vælger" />
</div>

Brightspace leverer to indholdsoprettelsesoplevelser: **Classic Content** og **New Content Experience** (også kaldet **Lessons**). Begge eksponerer FastComments, men menustierne er forskellige. Hvert afsnit nedenfor dækker begge, hvor de adskiller sig.

#### Find FastComments-værktøjet

FastComments-værktøjet vises to steder i en kursusindholdseditor:

1. Aktivitetsvælgeren, tilgået fra et modul/enheds **Tilføj eksisterende**-knap (mærket **Tilføj eksisterende aktiviteter** i ældre Brightspace-versioner). FastComments dukker direkte op i vælgeren i de nuværende Brightspace-builds; ældre versioner har det under en undermenu **Eksterne læringsværktøjer**. Begge veje tilføjer FastComments som et selvstændigt emne.
2. **Indsæt indhold**-dialogen i HTML-editoren, under **LTI Advantage**. Dette indlejrer FastComments inline i et HTML-emne via LTI deep linking-flowet.

Hvis FastComments ikke vises i nogen af vælgerne, er deployment ikke aktiveret for den organisationsenhed, der indeholder kurset. Bed din Brightspace-administrator om at åbne **Admin-værktøjer** > **Administrer udvidelsesmuligheder** > **LTI Advantage** > FastComments-værktøj > **Vis implementeringer**, åbne deployment og tilføje kursusets organisationsenhed (eller en overordnet organisationsenhed) under **Organisationsenheder**.

#### Tilføj FastComments som et emne i et modul

Classic Content:

1. Åbn kurset og klik **Indhold** i navigationsbjælken.
2. Vælg det modul, der skal indeholde diskussionen (eller opret et via **Tilføj et modul**).
3. Klik **Tilføj eksisterende** (ældre Brightspace: **Tilføj eksisterende aktiviteter** > **Eksterne læringsværktøjer**).
4. I vælgeren, klik **FastComments**. Brightspace opretter et emne i modulet og returnerer dig til indholdsvisningen.
5. Klik på det nye emne. Omdøb det til noget beskrivende som `FastComments Discussion` ved hjælp af den inline titel-editor.

New Content Experience (Lessons):

1. Åbn kurset og klik **Indhold**.
2. Åbn den enhed og lektion, der skal indeholde diskussionen.
3. Klik **Tilføj** > **Eksisterende aktivitet** og vælg **FastComments** (ældre Brightspace: indlejret under **Eksterne læringsværktøjer**).
4. Aktiviteten tilføjes til lektionen.
5. Klik på aktivitetens titel for at omdøbe den.

Første gang en bruger (underviser eller studerende) åbner emnet, initialiserer FastComments tråden for det ressource-link. Tråden er bundet til resource link ID, så omdøbning eller flytning af emnet ændrer ikke, hvilken tråd der indlæses.

#### Indlejr FastComments inline i et HTML-emne

Brug dette flow, når du ønsker, at kommentarer skal vises under en læsning, video eller andet indhold på samme emneside i stedet for som et separat emne.

1. Åbn eller opret et HTML-emne i modulet/lektionen.
2. Klik **Rediger HTML** for at åbne Brightspace HTML-editoren.
3. Placer markøren, hvor kommentartråden skal vises.
4. Klik på **Indsæt indhold**-knappen (puslespilsbrikke-ikon i editorens værktøjslinje).
5. I Indsæt indhold-dialogen, rul til **LTI Advantage** og klik **FastComments**.
6. FastComments åbner en deep linking-vælger. Bekræft placeringen (standardindstillingerne fungerer til indholdsdiskussioner); klik **Indsæt** eller **Fortsæt**.
7. Brightspace vender tilbage til HTML-editoren med en pladsholderblok, der repræsenterer LTI-launch. Klik **Gem og luk** på emnet.

Når emnet indlæses, erstatter Brightspace pladsholderen med en iframe, der automatisk starter FastComments via LTI. Studerende ser diskussionstråden inline.

Et enkelt HTML-emne kan indeholde flere deep-linked FastComments-indsættelser. Hver indsættelse får sin egen tråd, fordi hvert deep link producerer et unikt resource link ID.

#### Modultopic vs. Inline Quicklink

Vælg tilgangen med **modultopic** når:

- Diskussionen er den primære aktivitet for det trin i modulet.
- Du ønsker, at emnet skal vises i Brightspaces indholdsfortegnelse, færdiggørelsessporing og Class Progress.

Vælg **inline embed**-tilgangen når:

- Kommentarer skal ligge under andet indhold på samme side.
- Du ikke ønsker et separat element, der kan spores til færdiggørelse i indholdsfortegnelsen.

#### Synlighed, udkast og udgivelsesbetingelser

Et nyt FastComments-emne er synligt for studerende som standard. For at skjule det mens du opsætter det:

1. I indholdseditoren, klik emnets titel (Classic) eller de tre prikker på aktiviteten (New Content Experience).
2. Sæt status til **Udkast** (Classic) eller slå **Synlighed** fra (New Content Experience).

Udkast-emner er usynlige for studerende. Undervisere og TAs ser dem stadig med en "Udkast"-badge.

For at begrænse emnet til en specifik gruppe eller sektion:

1. Åbn emnet.
2. Klik emnets titelsmenu > **Rediger egenskaber på stedet** (Classic) eller **Rediger** > **Begrænsninger** (New Content Experience).
3. Under **Udgivelsesbetingelser**, klik **Opret**.
4. Vælg **Gruppeindskrivning** eller **Sektionindskrivning**, vælg gruppen/sektionen, og gem.

Udgivelsesbetingelser stabler med FastComments' egen rolle-mapping. Studerende, der ikke kan se emnet, får ikke en LTI-launch.

#### Hvad studerende ser ved første launch

Når en studerende klikker emnet (eller indlæser et HTML-emne med en embed):

1. Brightspace udfører LTI 1.3-launch i baggrunden.
2. FastComments modtager studerendes navn, e-mail, avatar-URL og LMS-rolle og logger dem automatisk ind. Der kommer ingen FastComments-loginprompt.
3. Kommentartråden for det resource link renderes inde i Brightspace-iframe'en.

Rolle-mapping ved launch:

- Brightspace `Administrator` bliver en FastComments **admin** for tråden (fuld moderation, slet, udeluk og konfigurationsadgang).
- Brightspace `Instructor` bliver en FastComments **moderator** (pin, skjul, slet, udeluk).
- Alle andre roller (`Learner`, `TeachingAssistant`, osv.) bliver almindelige kommentatorer.

Kommentarer tilskrives den studerendes Brightspace-konto. Hvis den studerende redigerer sit navn eller avatar i Brightspace, synkroniserer næste LTI-launch ændringen.

#### Iframe-højde og resize

FastComments udsender postMessage `org.imsglobal.lti.frameResize` ved hver trådrendering og ved indholdsændringer (ny kommentar, udvid svar). Brightspace lytter efter denne besked og justerer iframe-højden, så tråden ikke bliver klippet og ikke viser en indre scrollbar.

Hvis iframe'en forbliver i en fast kort højde:

- Bekræft, at kurset indlæses over HTTPS. Brightspaces postMessage-lytter afviser mixed-content-frames.
- Bekræft, at ingen browserudvidelse blokerer postMessage-kanalen.
- For inline embeds i et HTML-emne må den omgivende HTML ikke omslutte iframe'en i en container med fast højde. Fjern enhver inline `style="height: ..."` fra overordnede element.

#### Brightspace-specifikke faldgruber

**Værktøj vises ikke i Tilføj eksisterende-vælgeren.** Deployment er ikke aktiveret for dette kursus' organisationsenhed. En administrator skal tilføje organisationsenheden (eller en overordnet) til deploymentets **Organisationsenheder**-liste. Værktøjsregistrering alene er ikke nok; deployment bestemmer, hvilke kurser der kan se værktøjet.

**`deployment_id` mismatch ved launch.** FastComments TOFU-pinner det første `deployment_id`, det ser for en registrering. Hvis en administrator sletter den oprindelige deployment og opretter en ny, vil launches fra den nye deployment blive afvist med en deployment mismatch-fejl. Løsningen er at genregistrere FastComments (generer en ny registrerings-URL (<a href="https://fastcomments.com/auth/my-account/lti-config" target="_blank">hent den her</a>) og kør Dynamic Registration igen); den gamle konfigurationspost erstattes.

**Værktøj lancerer, men viser "Invalid LTI launch".** Kurset er i en anden tenant/organisationsstruktur end deployment dækker, eller deployment blev deaktiveret efter registrering. Tjek igen **Admin-værktøjer** > **Administrer udvidelsesmuligheder** > **LTI Advantage** > FastComments > **Aktiveret**-toggle og deploymentets liste over organisationsenheder.

**Navne og roller mangler inde i FastComments.** Brightspace sender LTI-launches med Names and Role Provisioning Services (NRPS)-claims. Hvis et kursus blev opgraderet fra et ældre LTI 1.1-link, mangler launch `name` og `email`-claims. Gentilføj FastComments-emnet via **Tilføj eksisterende** (migrer ikke det gamle link), så launch bruger LTI 1.3.

**Embed viser en login-side i stedet for auto-SSO.** HTML-emnet blev indsat som en almindelig `<iframe>` pegende på FastComments i stedet for via **Indsæt indhold** > **LTI Advantage**. Almindelige iframes springer LTI-launch over og lander brugere på den offentligt tilgængelige FastComments-side. Slet iframe'en og indsæt igen via Indsæt indhold-flowet.
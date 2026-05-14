Denne side beskriver, hvordan man tilføjer FastComments til et Brightspace-kursus, efter en administrator har registreret værktøjet og oprettet en deployment. Hvis værktøjet endnu ikke er registreret, se først D2L-registreringsvejledningen.

<div class="screenshot white-bg">
    <div class="title">FastComments indlejret som et emne i en Brightspace-enhed</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-d2l-comments-in-unit.png" alt="FastComments, der kører inde i en Brightspace-enhed, og viser trådede kommentarer og en @-mention-vælger" />
</div>

Brightspace leverer to indholdsskabelser: **Classic Content** og **New Content Experience** (også kaldet **Lessons**). Begge eksponerer FastComments, men menustierne er forskellige. Hvert afsnit nedenfor dækker begge, hvor de adskiller sig.

#### Find FastComments-værktøjet

FastComments-værktøjet vises to steder inde i en kursusindholdseditor:

1. Aktivitetspickeret, nås via et modul/enheds **Tilføj eksisterende**-knap (mærket **Tilføj eksisterende aktiviteter** i ældre Brightspace-versioner). FastComments vises direkte i pickeret i nyere Brightspace-udgivelser; ældre versioner ligger det under en **Eksterne læringsværktøjer**-undermenu. Begge stier tilføjer FastComments som et selvstændigt emne.
2. **Indsæt indhold**-dialogen inde i HTML-editoren, under **LTI Advantage**. Dette indlejrer FastComments inline i et HTML-emne via LTI deep linking-flowet.

Hvis FastComments ikke vises i nogen af pickerne, er deploymenten ikke aktiveret for den organisatoriske enhed, der holder kurset. Bed din Brightspace-administrator om at åbne **Adminværktøjer** > **Administrer udvidelser** > **LTI Advantage** > FastComments-værktøj > **Vis deployments**, åbne deploymenten og tilføje kursets organisatoriske enhed (eller en overordnet enhed) under **Organisationsenheder**.

#### Tilføj FastComments som et emne i et modul

Classic Content:

1. Åbn kurset og klik **Content** i navigationslinjen.
2. Vælg det modul, der skal indeholde diskussionen (eller opret et via **Tilføj et modul**).
3. Klik **Tilføj eksisterende** (ældre Brightspace: **Tilføj eksisterende aktiviteter** > **Eksterne læringsværktøjer**).
4. I pickeret klikker du **FastComments**. Brightspace opretter et emne i modulet og returnerer dig til indholdsvisningen.
5. Klik på det nye emne. Omdøb det til noget beskrivende som `FastComments Discussion` ved hjælp af den indlejrede titelredigerer.

New Content Experience (Lessons):

1. Åbn kurset og klik **Content**.
2. Åbn enheden og lektionen, der skal indeholde diskussionen.
3. Klik **Tilføj** > **Eksisterende aktivitet** og vælg **FastComments** (ældre Brightspace: indlejret under **Eksterne læringsværktøjer**).
4. Aktiviteten tilføjes lektionen.
5. Klik på aktivitetstitlen for at omdøbe den.

Første gang en bruger (instruktør eller studerende) åbner emnet, initialiserer FastComments tråden for det resourcelink. Tråden er bundet til resource link ID, så omdøbning eller flytning af emnet ændrer ikke, hvilken tråd der indlæses.

#### Indlejre FastComments inline i et HTML-emne

Brug dette flow, når du ønsker, at kommentarer skal vises under en læsning, video eller andet indhold inde på samme emneside i stedet for som et separat emne.

1. Åbn eller opret et HTML-emne i modulet/lektionen.
2. Klik **Rediger HTML** for at åbne Brightspace HTML-editoren.
3. Placer markøren, hvor kommentartråden skal vises.
4. Klik på **Indsæt indhold**-knappen (puslespils-ikonet i editorens værktøjslinje).
5. I Indsæt indhold-dialogen ruller du til **LTI Advantage** og klikker **FastComments**.
6. FastComments åbner en deep linking-picker. Bekræft placeringen (standardindstillingerne fungerer til indholds-diskussioner); klik **Indsæt** eller **Fortsæt**.
7. Brightspace returnerer til HTML-editoren med en pladsholderblok, der repræsenterer LTI-launch. Klik **Gem og luk** på emnet.

Når emnet indlæses, erstatter Brightspace pladsholderen med en iframe, som auto-launcher FastComments via LTI. Studerende ser diskussionstråden inline.

Et enkelt HTML-emne kan indeholde flere deep-linked FastComments-indsættelser. Hver indsættelse får sin egen tråd, fordi hvert deep link producerer et særskilt resource link ID.

#### Modul-emne vs Inline hurtiglink

Vælg tilgangen med **modul-emne** når:

- Diskussionen er den primære aktivitet i dette trin af modulet.
- Du ønsker, at emnet skal vises i Brightspaces indholdsfortegnelse, fuldførelsessporing og Class Progress.

Vælg **inline-indlejringen** når:

- Kommentarer skal ligge under andet indhold på samme side.
- Du ikke ønsker et separat fuldførelsessporbart element i indholdsfortegnelsen.

#### Synlighed, udkast og udgivelsesbetingelser

Et nyt FastComments-emne er synligt for studerende som standard. For at skjule det, mens du sætter det op:

1. I indholdseditoren klik på emnets titel (Classic) eller tre-punkts-menuen på aktiviteten (New Content Experience).
2. Sæt status til **Udkast** (Classic) eller slå **Synlighed** fra (New Content Experience).

Udkast-emner er usynlige for studerende. Instruktører og TAs kan stadig se dem med et "Udkast"-badge.

For at begrænse emnet til en specifik gruppe eller sektion:

1. Åbn emnet.
2. Klik på emnets titels menu > **Rediger egenskaber på stedet** (Classic) eller **Rediger** > **Begrænsninger** (New Content Experience).
3. Under **Udgivelsesbetingelser**, klik **Opret**.
4. Vælg **Gruppeindskrivning** eller **Sektionindskrivning**, vælg gruppen/sektionen og gem.

Udgivelsesbetingelser stables med FastComments' egen rolle-mapping. Studerende, der ikke kan se emnet, får ikke en LTI-launch.

#### Hvad studerende ser ved første opstart

Når en studerende klikker emnet (eller indlæser et HTML-emne med en indlejring):

1. Brightspace udfører LTI 1.3-launch i baggrunden.
2. FastComments modtager den studerendes navn, e-mail, avatar-URL og LMS-rolle, og logger dem ind automatisk. Der vises ikke en FastComments-loginprompt.
3. Kommentartråden for det pågældende resource link rendres inde i Brightspace-iframe'en.

Rolle-mapping ved lancering:

- Brightspace `Administrator` bliver en FastComments **admin** for tråden (fuld moderation, sletning, udelukkelse og adgang til konfiguration).
- Brightspace `Instructor` bliver en FastComments **moderator** (fastgør, skjul, slet, udeluk).
- Alle andre roller (`Learner`, `TeachingAssistant`, osv.) bliver almindelige kommentatorer.

Kommentarer tilskrives den studerendes Brightspace-konto. Hvis den studerende redigerer sit navn eller avatar i Brightspace, synkroniserer næste LTI-launch ændringen.

#### Iframe-højde og tilpasning

FastComments udsender postMessage `org.imsglobal.lti.frameResize` ved hver rendering af en tråd og ved indholdsændringer (ny kommentar, udvid svar). Brightspace lytter efter denne besked og justerer iframe-højden, så tråden ikke bliver beskåret og ikke viser en indre rulningsliste.

Hvis iframe'en forbliver i en fast kort højde:

- Bekræft, at kurset indlæses over HTTPS. Brightspaces postMessage-lytter afviser frames med mixed content.
- Bekræft, at ingen browserudvidelse blokerer postMessage-kanalen.
- For inline-indlejringer i et HTML-emne må den omgivende HTML ikke pakke iframe'en ind i en container med fast højde. Fjern enhver inline `style="height: ..."` fra parent-elementet.

#### Brightspace-specifikke faldgruber

**Værktøj vises ikke i Tilføj eksisterende-pickeret.** Deploymenten er ikke aktiveret for dette kursus' organisatoriske enhed. En administrator skal tilføje enheden (eller en overordnet) til deploymentens **Organisationsenheder**-liste. Værktøjets registrering alene er ikke nok; deploymenten bestemmer, hvilke kurser der ser værktøjet.

**`deployment_id` mismatch ved lancering.** FastComments TOFU-pins det første `deployment_id`, den ser for en registrering. Hvis en administrator sletter den oprindelige deployment og opretter en ny, afvises launches fra den nye deployment med en deployment mismatch-fejl. Løsningen er at gen-registrere FastComments (generer en ny registrerings-URL og kør Dynamic Registration igen); den gamle konfigurationspost bliver erstattet.

**Værktøjet launcher, men viser "Invalid LTI launch".** Kurset er i en anden tenant/organisationsstruktur, end deploymenten dækker, eller deploymenten blev deaktiveret efter registrering. Tjek igen **Adminværktøjer** > **Administrer udvidelser** > **LTI Advantage** > FastComments > **Aktiveret**-knappen og deploymentens liste over organisationsenheder.

**Navne og roller mangler inde i FastComments.** Brightspace sender LTI-launches med Names and Role Provisioning Services (NRPS) claims. Hvis et kursus blev opgraderet fra et ældre LTI 1.1-link, mangler launch'en `name` og `email` claims. Tilføj FastComments-emnet igen via **Tilføj eksisterende** (migrer ikke det gamle link), så launch'en bruger LTI 1.3.

**Indlejring viser en login-skærm i stedet for auto-SSO.** HTML-emnet blev indsat som en almindelig `<iframe>` med henvisning til FastComments i stedet for via **Indsæt indhold** > **LTI Advantage**. Almindelige iframes springer LTI-launch over og lander brugere på den offentlige FastComments-side. Slet iframe'en og indsæt igen via Indsæt indhold-flowet.
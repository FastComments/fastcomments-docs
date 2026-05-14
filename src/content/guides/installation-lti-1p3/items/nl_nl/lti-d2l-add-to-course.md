Deze pagina behandelt het toevoegen van FastComments aan een Brightspace-cursus nadat een beheerder de tool heeft geregistreerd en een deployment heeft gemaakt. Als de tool nog niet is geregistreerd, raadpleeg eerst de D2L-registratiehandleiding.

<div class="screenshot white-bg">
    <div class="title">FastComments ingebed als een onderwerp in Brightspace</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-d2l-comments-in-unit.png" alt="FastComments uitgevoerd binnen een Brightspace-eenheid, met gespreksdraden en een keuzemenu voor @-vermeldingen" />
</div>

Brightspace levert twee manieren om content te creëren: **Classic Content** en de **New Content Experience** (ook wel **Lessons** genoemd). Beide bieden toegang tot FastComments, maar de menupaden verschillen. Elk gedeelte hieronder behandelt beide waar ze afwijken.

#### Locate the FastComments Tool

De FastComments-tool verschijnt op twee plaatsen binnen de content-editor van een cursus:

1. De activity picker, bereikbaar via de module/unitknop **Add Existing** (in oudere Brightspace-versies gelabeld **Add Existing Activities**). In de huidige Brightspace-builds verschijnt FastComments direct in de picker; oudere versies plaatsen het onder een submenu **External Learning Tools**. Beide paden voegen FastComments toe als een zelfstandig onderwerp.
2. Het **Insert Stuff**-dialoogvenster in de HTML-editor, onder **LTI Advantage**. Dit embedt FastComments inline in een HTML-onderwerp via de LTI deep linking-stroom.

Als FastComments niet in één van beide pickers verschijnt, is de deployment niet ingeschakeld voor de org-eenheid die de cursus bevat. Vraag uw Brightspace-beheerder om naar **Admin Tools** > **Manage Extensibility** > **LTI Advantage** > FastComments tool > **View Deployments** te gaan, de deployment te openen en de org-eenheid van de cursus (of een bovenliggende org-eenheid) toe te voegen onder **Org Units**.

#### Add FastComments as a Topic in a Module

Classic Content:

1. Open de cursus en klik op **Content** in de navigatiebalk.
2. Selecteer de module die de discussie moet bevatten (of maak er een via **Add a module**).
3. Klik op **Add Existing** (oudere Brightspace: **Add Existing Activities** > **External Learning Tools**).
4. Klik in de picker op **FastComments**. Brightspace maakt een onderwerp in de module en keert terug naar de contentweergave.
5. Klik het nieuwe onderwerp aan. Hernoem het tot iets beschrijvends zoals `FastComments Discussion` met de inline titelbewerker.

New Content Experience (Lessons):

1. Open de cursus en klik op **Content**.
2. Open de unit en lesson die de discussie moeten bevatten.
3. Klik op **Add** > **Existing Activity** en selecteer **FastComments** (oudere Brightspace: genest onder **External Learning Tools**).
4. De activiteit wordt aan de lesson toegevoegd.
5. Klik de activiteitstitel om deze te hernoemen.

De eerste keer dat een gebruiker (instructeur of student) het onderwerp opent, initialiseert FastComments de thread voor die resource link. De thread is gebonden aan de resource link ID, dus het hernoemen of verplaatsen van het onderwerp verandert niet welke thread wordt geladen.

#### Embed FastComments Inline in an HTML Topic

Gebruik deze flow wanneer je wilt dat reacties onder een tekst, video of andere content binnen dezelfde onderwerppagina verschijnen in plaats van als een apart onderwerp.

1. Open of maak een HTML-onderwerp in de module/lesson.
2. Klik op **Edit HTML** om de Brightspace HTML-editor te openen.
3. Plaats de cursor waar de comment-thread moet verschijnen.
4. Klik op de knop **Insert Stuff** (puzzelstuk-icoon in de editorwerkbalk).
5. Scroll in het Insert Stuff-dialoogvenster naar **LTI Advantage** en klik op **FastComments**.
6. FastComments opent een deep linking-picker. Bevestig de plaatsing (de standaardopties werken voor contentdiscussies); klik op **Insert** of **Continue**.
7. Brightspace keert terug naar de HTML-editor met een placeholderblok dat de LTI-launch vertegenwoordigt. Klik op **Save and Close** op het onderwerp.

Wanneer het onderwerp laadt, vervangt Brightspace de placeholder door een iframe dat automatisch FastComments start via LTI. Studenten zien de discussiedraad inline.

Een enkel HTML-onderwerp kan meerdere deep-linked FastComments-embeds bevatten. Elke embed krijgt zijn eigen thread omdat elke deep link een unieke resource link ID produceert.

#### Module Topic vs Inline Quicklink

Kies de benadering met het **module topic** wanneer:

- De discussie de primaire activiteit is voor die stap in de module.
- Je wilt dat het onderwerp verschijnt in Brightspace's inhoudsopgave, voltooiingstracking en Class Progress.

Kies de **inline embed**-benadering wanneer:

- Reacties onder andere content op dezelfde pagina moeten staan.
- Je geen apart voltooiingsvolgbaar item in de inhoudsopgave wilt.

#### Visibility, Draft, and Release Conditions

Een nieuw FastComments-onderwerp is standaard zichtbaar voor studenten. Om het te verbergen terwijl je het instelt:

1. Klik in de content-editor op de onderwerpstitel (Classic) of het drie-puntjesmenu bij de activiteit (New Content Experience).
2. Zet de status op **Draft** (Classic) of zet **Visibility** uit (New Content Experience).

Draft-onderwerpen zijn onzichtbaar voor studenten. Instructeurs en TA's zien ze nog steeds met een "Draft"-badge.

Om het onderwerp te beperken tot een specifieke groep of sectie:

1. Open het onderwerp.
2. Klik op het onderwerptitelmenu > **Edit Properties In-place** (Classic) of **Edit** > **Restrictions** (New Content Experience).
3. Klik onder **Release Conditions** op **Create**.
4. Kies **Group enrollment** of **Section enrollment**, selecteer de groep/sectie en sla op.

Release-voorwaarden stapelen zich bovenop FastComments' eigen rolmapping. Studenten die het onderwerp niet kunnen zien, krijgen geen LTI-launch.

#### What Students See on First Launch

Wanneer een student op het onderwerp klikt (of een HTML-onderwerp met een embed laadt):

1. Brightspace voert de LTI 1.3-launch op de achtergrond uit.
2. FastComments ontvangt de naam van de student, e-mail, avatar-URL en LMS-rol, en logt hen automatisch in. Er verschijnt geen FastComments-inlogprompt.
3. De comment-thread voor die resource link wordt weergegeven binnen het Brightspace-iframe.

Rollenmapping bij launch:

- Brightspace `Administrator` wordt een FastComments **admin** voor de thread (volledige moderatie, verwijderen, blokkeren en configuratietoegang).
- Brightspace `Instructor` wordt een FastComments **moderator** (vastzetten, verbergen, verwijderen, blokkeren).
- Alle andere rollen (`Learner`, `TeachingAssistant`, enz.) worden standaard reacteurs.

Reacties worden toegeschreven aan het Brightspace-account van de student. Als de student zijn/haar naam of avatar in Brightspace wijzigt, synchroniseert de volgende LTI-launch de wijziging.

#### Iframe Height and Resize

FastComments verzendt het `org.imsglobal.lti.frameResize` postMessage bij elke weergave van een thread en bij inhoudsveranderingen (nieuwe reactie, reacties uitvouwen). Brightspace luistert naar dit bericht en past de iframe-hoogte aan zodat de thread niet wordt afgesneden en er geen interne scrollbar verschijnt.

Als het iframe op een vaste korte hoogte blijft staan:

- Controleer of de cursus via HTTPS wordt geladen. De postMessage-listener van Brightspace weigert mixed-content frames.
- Controleer of geen browserextensie het postMessage-kanaal blokkeert.
- Voor inline embeds in een HTML-onderwerp mag de omliggende HTML het iframe niet in een container met vaste hoogte wikkelen. Verwijder eventuele inline `style="height: ..."` van het bovenliggende element.

#### Brightspace-Specific Gotchas

**Tool not showing in the Add Existing picker.** De deployment is niet ingeschakeld voor de org-eenheid van deze cursus. Een beheerder moet de org-eenheid (of een bovenliggende) aan de deployment's **Org Units**-lijst toevoegen. Tool-registratie alleen is niet voldoende; de deployment bepaalt welke cursussen de tool zien.

**`deployment_id` mismatch on launch.** FastComments pinnt TOFU het eerste `deployment_id` dat het ziet voor een registratie. Als een beheerder de originele deployment verwijdert en een nieuwe maakt, worden launches vanaf de nieuwe deployment afgewezen met een deployment mismatch-fout. De oplossing is FastComments opnieuw te registreren (genereer een nieuwe registratie-URL en voer Dynamic Registration opnieuw uit); het oude configuratierecord wordt vervangen.

**Tool launches but shows "Invalid LTI launch".** De cursus bevindt zich in een andere tenant/org-structuur dan de deployment dekt, of de deployment is uitgeschakeld na registratie. Controleer opnieuw **Admin Tools** > **Manage Extensibility** > **LTI Advantage** > FastComments > toggle **Enabled** en de org-eenhedenlijst van de deployment.

**Names and roles missing inside FastComments.** Brightspace levert LTI-launches met Names and Role Provisioning Services (NRPS)-claims. Als een cursus is geüpgraded van een oudere LTI 1.1-link, ontbreekt de launch `name` en `email` claims. Voeg het FastComments-onderwerp opnieuw toe via **Add Existing** (migreer de oude link niet) zodat de launch LTI 1.3 gebruikt.

**Embed shows a login screen instead of auto-SSO.** Het HTML-onderwerp is ingevoegd als een gewoon `<iframe>` dat naar FastComments wijst in plaats van via **Insert Stuff** > **LTI Advantage**. Gewone iframes slaan de LTI-launch over en brengen gebruikers naar de publiekelijk toegankelijke FastComments-pagina. Verwijder het iframe en voeg het opnieuw in via de Insert Stuff-flow.
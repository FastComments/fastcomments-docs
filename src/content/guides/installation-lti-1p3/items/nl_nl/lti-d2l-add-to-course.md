Deze pagina behandelt het toevoegen van FastComments aan een Brightspace-cursus nadat een beheerder de tool heeft geregistreerd en een deployment heeft gemaakt. Als de tool nog niet geregistreerd is, zie eerst de D2L-registratiehandleiding.

Brightspace biedt twee content-creatie-ervaringen: **Classic Content** en de **New Content Experience** (ook genoemd **Lessons**). Beide bieden toegang tot FastComments, maar de menu-paden verschillen. Elke sectie hieronder behandelt beide waar ze uit elkaar lopen.

#### Vind de FastComments-tool

De FastComments-tool verschijnt op twee plaatsen in een cursuscontent-editor:

1. De activiteitkiezer, bereikbaar via de **Add Existing** knop van een module/unit (in oudere Brightspace-versies aangeduid als **Add Existing Activities**). FastComments verschijnt direct in de kiezer in huidige Brightspace-builds; oudere versies plaatsen het onder een submenu **External Learning Tools**. Beide paden voegen FastComments toe als een op zichzelf staand onderwerp.
2. Het **Insert Stuff** dialoogvenster in de HTML-editor, onder **LTI Advantage**. Dit embedt FastComments inline in een HTML-onderwerp via de LTI deep linking-flow.

Als FastComments niet in een van beide kiezers verschijnt, is de deployment niet ingeschakeld voor de org-eenheid die de cursus bevat. Vraag uw Brightspace-beheerder om **Admin Tools** > **Manage Extensibility** > **LTI Advantage** > FastComments tool > **View Deployments** te openen, de deployment te openen en de org-eenheid van de cursus (of een bovenliggende org-eenheid) toe te voegen onder **Org Units**.

#### Voeg FastComments toe als een onderwerp in een module

Classic Content:

1. Open de cursus en klik op **Content** in de navigatiebalk.
2. Selecteer de module die de discussie moet bevatten (of maak er een via **Add a module**).
3. Klik op **Add Existing** (oudere Brightspace: **Add Existing Activities** > **External Learning Tools**).
4. In de kiezer, klik op **FastComments**. Brightspace maakt een onderwerp in de module en keert terug naar de content-weergave.
5. Klik het nieuwe onderwerp aan. Hernoem het naar iets beschrijvends zoals `FastComments Discussion` met de inline titel-editor.

New Content Experience (Lessons):

1. Open de cursus en klik op **Content**.
2. Open de unit en lesson die de discussie moeten bevatten.
3. Klik op **Add** > **Existing Activity** en selecteer **FastComments** (oudere Brightspace: genest onder **External Learning Tools**).
4. De activiteit wordt toegevoegd aan de lesson.
5. Klik op de activiteitsnaam om deze te hernoemen.

De eerste keer dat een gebruiker (instructeur of student) het onderwerp opent, initialiseert FastComments de thread voor die resource link. De thread is gebonden aan de resource link ID, dus het hernoemen of verplaatsen van het onderwerp verandert niet welke thread wordt geladen.

#### Embed FastComments inline in een HTML-onderwerp

Gebruik deze flow wanneer je wilt dat reacties onder een leesstuk, video of andere inhoud op dezelfde onderwerppagina verschijnen in plaats van als een apart onderwerp.

1. Open of maak een HTML-onderwerp in de module/lesson.
2. Klik op **Edit HTML** om de Brightspace HTML-editor te openen.
3. Plaats de cursor waar de comment-draad moet verschijnen.
4. Klik op de knop **Insert Stuff** (puzzelstukpictogram in de editorwerkbalk).
5. Scroll in het Insert Stuff-dialoogvenster naar **LTI Advantage** en klik op **FastComments**.
6. FastComments opent een deep linking-kiezer. Bevestig de plaatsing (de standaardopties werken voor content-discussies); klik op **Insert** of **Continue**.
7. Brightspace keert terug naar de HTML-editor met een tijdelijke aanduiding die de LTI-launch vertegenwoordigt. Klik op **Save and Close** in het onderwerp.

Wanneer het onderwerp wordt geladen, vervangt Brightspace de tijdelijke aanduiding door een iframe dat FastComments automatisch start via LTI. Studenten zien de discussie-thread inline.

Een enkel HTML-onderwerp kan meerdere deep-linked FastComments-embeds bevatten. Elke embed krijgt zijn eigen thread omdat elke deep link een unieke resource link ID produceert.

#### Moduleonderwerp vs Inline Quicklink

Kies de **moduleonderwerp**-aanpak wanneer:

- De discussie de primaire activiteit is voor die stap in de module.
- Je wilt dat het onderwerp verschijnt in Brightspace’s inhoudsopgave, voltooiings-tracking en Class Progress.

Kies de **inline embed**-aanpak wanneer:

- Reacties onder andere inhoud op dezelfde pagina moeten staan.
- Je geen afzonderlijk, voor voltooiing traceerbaar item in de inhoudsopgave wilt.

#### Zichtbaarheid, concept en release-voorwaarden

Een nieuw FastComments-onderwerp is standaard zichtbaar voor studenten. Om het te verbergen terwijl je het instelt:

1. Klik in de content-editor op de onderwerpnaam (Classic) of het drie-puntjesmenu bij de activiteit (New Content Experience).
2. Zet de status op **Draft** (Classic) of zet **Visibility** uit (New Content Experience).

Conceptonderwerpen zijn onzichtbaar voor studenten. Instructeurs en TA’s zien ze nog wel met een “Draft”-badge.

Om het onderwerp te beperken tot een specifieke groep of sectie:

1. Open het onderwerp.
2. Klik op het onderwerptitelmenu > **Edit Properties In-place** (Classic) of **Edit** > **Restrictions** (New Content Experience).
3. Onder **Release Conditions**, klik op **Create**.
4. Kies **Group enrollment** of **Section enrollment**, selecteer de groep/sectie en sla op.

Release-voorwaarden stapelen zich op de roltoewijzing van FastComments. Studenten die het onderwerp niet kunnen zien, krijgen geen LTI-launch.

#### Wat studenten zien bij de eerste lancering

Wanneer een student op het onderwerp klikt (of een HTML-onderwerp met een embed laadt):

1. Brightspace voert de LTI 1.3-launch op de achtergrond uit.
2. FastComments ontvangt de naam van de student, e-mail, avatar-URL en LMS-rol, en meldt hen automatisch aan. Er verschijnt geen FastComments-aanmeldprompt.
3. De comment-thread voor die resource link rendert in het Brightspace-iframe.

Roltoewijzing bij de launch:

- Brightspace `Administrator` wordt een FastComments **admin** voor de thread (volledige moderatie, verwijderen, bannen en configuratietoegang).
- Brightspace `Instructor` wordt een FastComments **moderator** (vastzetten, verbergen, verwijderen, bannen).
- Alle andere rollen (`Learner`, `TeachingAssistant`, enz.) worden gewone commentaarschrijvers.

Reacties worden toegeschreven aan het Brightspace-account van de student. Als de student zijn naam of avatar in Brightspace wijzigt, synchroniseert de volgende LTI-launch de wijziging.

#### Iframe-hoogte en grootte-aanpassing

FastComments stuurt het `org.imsglobal.lti.frameResize` postMessage bij elke thread-render en bij inhoudswijzigingen (nieuwe reactie, uitklappen van antwoorden). Brightspace luistert naar dit bericht en past de iframe-hoogte aan zodat de thread niet wordt afgekapt en er geen interne scrollbar verschijnt.

Als het iframe op een vaste korte hoogte blijft:

- Controleer of de cursus via HTTPS wordt geladen. De postMessage-listener van Brightspace blokkeert mixed-content frames.
- Controleer of geen browserextensie het postMessage-kanaal blokkeert.
- Voor inline embeds in een HTML-onderwerp mag de omliggende HTML het iframe niet insluiten in een container met vaste hoogte. Verwijder eventuele inline `style="height: ..."` van het bovenliggende element.

#### Brightspace-specifieke valkuilen

**Tool wordt niet weergegeven in de Add Existing-kiezer.** De deployment is niet ingeschakeld voor de org-eenheid van deze cursus. Een beheerder moet de org-eenheid (of een ouder) toevoegen aan de Org Units-lijst van de deployment. Alleen tool-registratie is niet voldoende; de deployment bepaalt welke cursussen toegang hebben tot de tool.

**`deployment_id` mismatch bij launch.** FastComments ‘TOFU’-pint het eerste `deployment_id` dat het ziet voor een registratie. Als een beheerder de oorspronkelijke deployment verwijdert en een nieuwe maakt, worden launches vanaf de nieuwe deployment geweigerd met een deployment mismatch-fout. De oplossing is FastComments opnieuw te registreren (genereer een nieuwe registratie-URL en voer Dynamic Registration opnieuw uit); de oude configuratierecord wordt vervangen.

**Tool start wel maar toont “Invalid LTI launch”.** De cursus bevindt zich in een andere tenant/org-structuur dan de deployment dekt, of de deployment is na registratie uitgeschakeld. Controleer opnieuw **Admin Tools** > **Manage Extensibility** > **LTI Advantage** > FastComments > de **Enabled**-schakelaar en de org-eenhedenlijst van de deployment.

**Namen en rollen ontbreken binnen FastComments.** Brightspace verstuurt LTI-launches met Names and Role Provisioning Services (NRPS)-claims. Als een cursus is geüpgraded vanaf een oudere LTI 1.1-link, mist de launch `name` en `email` claims. Voeg het FastComments-onderwerp opnieuw toe via **Add Existing** (migreer de oude link niet) zodat de launch LTI 1.3 gebruikt.

**Embed toont een inlogscherm in plaats van auto-SSO.** Het HTML-onderwerp is als een gewone `<iframe>` ingevoegd die naar FastComments verwijst in plaats van via **Insert Stuff** > **LTI Advantage**. Gewone iframes slaan de LTI-launch over en leiden gebruikers naar de publiekelijke FastComments-pagina. Verwijder het iframe en voeg het opnieuw in via de Insert Stuff-flow.
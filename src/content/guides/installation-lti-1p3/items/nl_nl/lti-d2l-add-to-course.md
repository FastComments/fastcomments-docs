Deze pagina behandelt het toevoegen van FastComments aan een Brightspace-cursus nadat een beheerder de tool heeft geregistreerd en een deployment heeft gemaakt. Als de tool nog niet is geregistreerd, raadpleeg dan eerst de D2L-registratiehandleiding.

<div class="screenshot white-bg">
    <div class="title">FastComments ingebed als een onderwerp binnen een Brightspace-eenheid</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-d2l-comments-in-unit.png" alt="FastComments uitgevoerd binnen een Brightspace-eenheid, met gethreadde opmerkingen en een @-mention picker" />
</div>

Brightspace levert twee content-creatie-ervaringen: **Classic Content** en de **New Content Experience** (ook wel **Lessons** genoemd). Beide bieden toegang tot FastComments, maar de menupaden verschillen. Elke sectie hieronder behandelt beide waar ze afwijken.

#### Locate the FastComments Tool

De FastComments-tool verschijnt op twee plaatsen binnen de cursuscontent-editor:

1. De activity picker, bereikbaar via de **Add Existing**-knop van een module/eenhied (in oudere Brightspace-versies gelabeld **Add Existing Activities**). FastComments verschijnt rechtstreeks in de picker in recente Brightspace-versies; oudere versies plaatsen het onder een submenu **External Learning Tools**. Beide paden voegen FastComments toe als een op zichzelf staand onderwerp.
2. Het **Insert Stuff**-dialoogvenster in de HTML-editor, onder **LTI Advantage**. Dit embedt FastComments inline in een HTML-onderwerp via de LTI deep linking-stroom.

Als FastComments niet in een van beide pickers verschijnt, is de deployment niet ingeschakeld voor de org-eenheid die de cursus bevat. Vraag je Brightspace-beheerder om naar **Admin Tools** > **Manage Extensibility** > **LTI Advantage** > FastComments tool > **View Deployments** te gaan, de deployment te openen en de org-eenheid van de cursus (of een bovenliggende org-eenheid) toe te voegen onder **Org Units**.

#### Add FastComments as a Topic in a Module

Classic Content:

1. Open de cursus en klik op **Content** in de navigatiebalk.
2. Selecteer de module die de discussie moet bevatten (of maak er een aan via **Add a module**).
3. Klik op **Add Existing** (oudere Brightspace: **Add Existing Activities** > **External Learning Tools**).
4. Klik in de picker op **FastComments**. Brightspace maakt een onderwerp in de module aan en brengt je terug naar de content-weergave.
5. Klik op het nieuwe onderwerp. Hernoem het naar iets beschrijvends zoals `FastComments Discussion` met de inline titel-editor.

New Content Experience (Lessons):

1. Open de cursus en klik op **Content**.
2. Open de unit en lesson die de discussie moeten bevatten.
3. Klik op **Add** > **Existing Activity** en selecteer **FastComments** (oudere Brightspace: genest onder **External Learning Tools**).
4. De activiteit wordt aan de lesson toegevoegd.
5. Klik op de activiteitstitel om deze te hernoemen.

De eerste keer dat een gebruiker (instructeur of student) het onderwerp opent, initialiseert FastComments de thread voor die resource link. De thread is gebonden aan de resource link ID, dus het hernoemen of verplaatsen van het onderwerp verandert niet welke thread wordt geladen.

#### Embed FastComments Inline in an HTML Topic

Gebruik deze flow wanneer je wilt dat opmerkingen onder een tekst, video of andere inhoud binnen dezelfde onderwerppagina verschijnen in plaats van als een afzonderlijk onderwerp.

1. Open of maak een HTML-onderwerp in de module/lesson.
2. Klik op **Edit HTML** om de Brightspace HTML-editor te openen.
3. Plaats de cursor waar de comment thread moet verschijnen.
4. Klik op de **Insert Stuff**-knop (puzzelstuk-icoon in de editorwerkbalk).
5. Scrol in het Insert Stuff-dialoogvenster naar **LTI Advantage** en klik op **FastComments**.
6. FastComments opent een deep linking-picker. Bevestig de plaatsing (de standaardopties werken voor contentdiscussies); klik op **Insert** of **Continue**.
7. Brightspace keert terug naar de HTML-editor met een tijdelijke aanduiding die de LTI-launch vertegenwoordigt. Klik op **Save and Close** in het onderwerp.

Wanneer het onderwerp wordt geladen, vervangt Brightspace de tijdelijke aanduiding door een iframe dat FastComments automatisch launcht via LTI. Studenten zien de discussiedraad inline.

Een enkel HTML-onderwerp kan meerdere deep-linked FastComments-embeds bevatten. Elke embed krijgt zijn eigen thread omdat elke deep link een unieke resource link ID produceert.

#### Module Topic vs Inline Quicklink

Kies de benadering van het **module topic** wanneer:

- De discussie de primaire activiteit is voor die stap in de module.
- Je wilt dat het onderwerp in Brightspace’s inhoudsopgave, voltooiingstracking en Class Progress verschijnt.

Kies de benadering van de **inline embed** wanneer:

- Reacties onder andere inhoud op dezelfde pagina moeten staan.
- Je geen afzonderlijk, voor voltooiing trackbaar item in de inhoudsopgave wilt.

#### Visibility, Draft, and Release Conditions

Een nieuw FastComments-onderwerp is standaard zichtbaar voor studenten. Om het te verbergen terwijl je het instelt:

1. Klik in de content-editor op de onderwerpstitel (Classic) of het driepuntjesmenu van de activiteit (New Content Experience).
2. Zet de status op **Draft** (Classic) of schakel **Visibility** uit (New Content Experience).

Draft-onderwerpen zijn onzichtbaar voor studenten. Instructeurs en TA’s zien ze nog wel met een "Draft"-badge.

Om het onderwerp te beperken tot een specifieke groep of sectie:

1. Open het onderwerp.
2. Klik op het onderwerptitelmenu > **Edit Properties In-place** (Classic) of **Edit** > **Restrictions** (New Content Experience).
3. Onder **Release Conditions** klik je op **Create**.
4. Kies **Group enrollment** of **Section enrollment**, selecteer de groep/sectie en sla op.

Release conditions stapelen zich op met FastComments’ eigen roltoewijzing. Studenten die het onderwerp niet kunnen zien, krijgen geen LTI-launch.

#### What Students See on First Launch

Wanneer een student op het onderwerp klikt (of een HTML-onderwerp met een embed laadt):

1. Brightspace voert de LTI 1.3-launch op de achtergrond uit.
2. FastComments ontvangt de naam van de student, e-mail, avatar-URL en LMS-rol, en logt de student automatisch in. Er verschijnt geen inlogprompt van FastComments.
3. De comment thread voor die resource link wordt weergegeven binnen het Brightspace-iframe.

Roltoewijzing bij launch:

- Brightspace `Administrator` wordt een FastComments **admin** voor de thread (volledige moderatie, verwijderen, bannen en configuratietoegang).
- Brightspace `Instructor` wordt een FastComments **moderator** (vastzetten, verbergen, verwijderen, bannen).
- Alle andere rollen (`Learner`, `TeachingAssistant`, enz.) worden standaard commentatoren.

Opmerkingen worden toegeschreven aan het Brightspace-account van de student. Als de student zijn/haar naam of avatar in Brightspace wijzigt, synchroniseert de volgende LTI-launch de wijziging.

#### Lock Down Public Access (Recommended)

Standaard zijn FastComments-opmerkingsgegevens openbaar leesbaar. Iedereen die de URL of het API-endpoint van een thread kan raden, kan de opmerkingen bekijken, zelfs buiten Brightspace. Voor cursusdiscussies wil je bijna zeker het bekijken beperken tot ingeschreven deelnemers.

Open je <a href="https://fastcomments.com/auth/my-account/customize-widget" target="_blank">pagina voor het aanpassen van de widget</a> en maak een regel met **Require SSO To View Comments** ingeschakeld, zet vervolgens het beveiligingsniveau op **Secure SSO** zodat threads alleen via de ondertekende LTI-launch kunnen worden geladen.

Zie [Protecting Comment Threads With Single-Sign-On](/guide-customizations-and-configuration.html#sso-require-to-view-comments) voor de volledige walkthrough, inclusief hoe je de regel specificeert voor een enkele domein of pagina.

#### Iframe Height and Resize

FastComments verzendt de `org.imsglobal.lti.frameResize` postMessage bij elke thread-render en bij inhoudsveranderingen (nieuwe opmerking, uitklappen van reacties). Brightspace luistert naar dit bericht en past de iframe-hoogte aan zodat de thread niet wordt afgekapt en er geen interne schuifbalk verschijnt.

Als het iframe op een vaste korte hoogte blijft:

- Controleer of de cursus via HTTPS wordt geladen. Brightspace’s postMessage-listener weigert mixed-content frames.
- Controleer of geen browserextensie het postMessage-kanaal blokkeert.
- Voor inline embeds in een HTML-onderwerp mag de omliggende HTML het iframe niet in een container met vaste hoogte wikkelen. Verwijder eventuele inline `style="height: ..."` van het bovenliggende element.

#### Brightspace-Specific Gotchas

**Tool not showing in the Add Existing picker.** De deployment is niet ingeschakeld voor de org-eenheid van deze cursus. Een beheerder moet de org-eenheid (of een bovenliggende) toevoegen aan de Org Units-lijst van de deployment. Alleen tool-registratie is niet voldoende; de deployment bepaalt welke cursussen de tool zien.

**`deployment_id` mismatch on launch.** FastComments pinnt bij first-use het eerste `deployment_id` dat het ziet voor een registratie. Als een beheerder de originele deployment verwijdert en een nieuwe creëert, worden launches vanaf de nieuwe deployment geweigerd met een deployment mismatch-fout. De oplossing is FastComments opnieuw te registreren (genereer een nieuwe registratie-URL (<a href="https://fastcomments.com/auth/my-account/lti-config" target="_blank">get it here</a>) en voer Dynamic Registration opnieuw uit); het oude configuratierecord wordt overschreven.

**Tool launches but shows "Invalid LTI launch".** De cursus bevindt zich in een andere tenant/org-structuur dan de deployment dekt, of de deployment is uitgeschakeld na registratie. Controleer opnieuw **Admin Tools** > **Manage Extensibility** > **LTI Advantage** > FastComments > **Enabled**-schakelaar en de org-eenhedenlijst van de deployment.

**Names and roles missing inside FastComments.** Brightspace levert LTI-launches met Names and Role Provisioning Services (NRPS)-claims. Als een cursus is geüpgraded van een oudere LTI 1.1-link, mist de launch `name`- en `email`-claims. Voeg het FastComments-onderwerp opnieuw toe via **Add Existing** (migreer de oude link niet) zodat de launch LTI 1.3 gebruikt.

**Embed shows a login screen instead of auto-SSO.** Het HTML-onderwerp is ingevoegd als een gewone `<iframe>` die naar FastComments wijst in plaats van via **Insert Stuff** > **LTI Advantage**. Gewone iframes slaan de LTI-launch over en leiden gebruikers naar de publieke FastComments-pagina. Verwijder het iframe en voeg het opnieuw in via de Insert Stuff-flow.

---
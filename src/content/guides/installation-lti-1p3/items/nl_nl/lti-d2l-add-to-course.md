Deze pagina behandelt het toevoegen van FastComments aan een Brightspace-cursus nadat een beheerder de tool heeft geregistreerd en een deployment heeft aangemaakt. Als de tool nog niet is geregistreerd, raadpleeg dan eerst de D2L-registratiegids.

<div class="screenshot white-bg">
    <div class="title">FastComments ingebed als eenseenheidsonderwerp in Brightspace</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-d2l-comments-in-unit.png" alt="FastComments draaiend binnen een Brightspace-eenheid, met geneste reacties en een @-mention-kiezer" />
</div>

Brightspace biedt twee content-authoring-ervaringen: **Classic Content** en de **New Content Experience** (ook wel **Lessons** genoemd). Beide bieden FastComments aan, maar de menupaden verschillen. Elke sectie hieronder behandelt beide waar ze uiteenlopen.

#### De FastComments-tool vinden

De FastComments-tool verschijnt op twee plaatsen binnen een cursuscontent-editor:

1. De activiteitenkiezer, bereikbaar via de knop **Add Existing** van een module/eenheid (gelabeld **Add Existing Activities** in oudere Brightspace-versies). FastComments verschijnt direct in de kiezer in huidige Brightspace-builds; oudere versies nesten het onder een submenu **External Learning Tools**. Beide paden voegen FastComments toe als een zelfstandig onderwerp.
2. Het dialoogvenster **Insert Stuff** binnen de HTML-editor, onder **LTI Advantage**. Dit embedt FastComments inline in een HTML-onderwerp via de LTI deep linking-flow.

Als FastComments in geen van beide kiezers verschijnt, is de deployment niet ingeschakeld voor de org-eenheid die de cursus bevat. Vraag uw Brightspace-beheerder om **Admin Tools** > **Manage Extensibility** > **LTI Advantage** > FastComments-tool > **View Deployments** te openen, de deployment te openen en de org-eenheid van de cursus (of een bovenliggende org-eenheid) toe te voegen onder **Org Units**.

#### FastComments toevoegen als onderwerp in een module

Classic Content:

1. Open de cursus en klik op **Content** in de navigatiebalk.
2. Selecteer de module die de discussie moet bevatten (of maak er een aan via **Add a module**).
3. Klik op **Add Existing** (oudere Brightspace: **Add Existing Activities** > **External Learning Tools**).
4. Klik in de kiezer op **FastComments**. Brightspace maakt een onderwerp aan in de module en brengt u terug naar de contentweergave.
5. Klik op het nieuwe onderwerp. Hernoem het naar iets beschrijvends zoals `FastComments Discussion` via de inline titel-editor.

New Content Experience (Lessons):

1. Open de cursus en klik op **Content**.
2. Open de eenheid en les die de discussie moeten bevatten.
3. Klik op **Add** > **Existing Activity** en selecteer **FastComments** (oudere Brightspace: genest onder **External Learning Tools**).
4. De activiteit wordt toegevoegd aan de les.
5. Klik op de activiteitstitel om deze te hernoemen.

De eerste keer dat een gebruiker (instructeur of student) het onderwerp opent, initialiseert FastComments de thread voor die resource link. De thread is gebonden aan de resource link-ID, dus het hernoemen of verplaatsen van het onderwerp verandert niet welke thread wordt geladen.

#### FastComments inline embedden in een HTML-onderwerp

Gebruik deze flow wanneer u reacties onder een leestekst, video of andere content binnen dezelfde onderwerppagina wilt laten verschijnen in plaats van als een apart onderwerp.

1. Open of maak een HTML-onderwerp aan in de module/les.
2. Klik op **Edit HTML** om de Brightspace HTML-editor te openen.
3. Plaats de cursor waar de reactiethread moet verschijnen.
4. Klik op de knop **Insert Stuff** (puzzelstukje-icoon in de werkbalk van de editor).
5. Scroll in het Insert Stuff-dialoogvenster naar **LTI Advantage** en klik op **FastComments**.
6. FastComments opent een deep linking-kiezer. Bevestig de plaatsing (de standaardopties werken voor contentdiscussies); klik op **Insert** of **Continue**.
7. Brightspace keert terug naar de HTML-editor met een placeholder-blok dat de LTI-launch vertegenwoordigt. Klik op **Save and Close** bij het onderwerp.

Wanneer het onderwerp laadt, vervangt Brightspace de placeholder door een iframe dat FastComments automatisch via LTI lanceert. Studenten zien de discussiethread inline.

Eén HTML-onderwerp kan meerdere deep-linked FastComments-embeds bevatten. Elke embed krijgt zijn eigen thread, omdat elke deep link een unieke resource link-ID oplevert.

#### Module-onderwerp vs. inline quicklink

Kies de aanpak **module-onderwerp** wanneer:

- De discussie de primaire activiteit is voor die stap in de module.
- U wilt dat het onderwerp verschijnt in de inhoudsopgave van Brightspace, de voortgangsbijhouding en Class Progress.

Kies de aanpak **inline embed** wanneer:

- Reacties onder andere content op dezelfde pagina moeten verschijnen.
- U geen apart, op volledigheid bij te houden item in de inhoudsopgave wilt.

#### Zichtbaarheid, concept en releasevoorwaarden

Een nieuw FastComments-onderwerp is standaard zichtbaar voor studenten. Om het te verbergen tijdens het instellen:

1. Klik in de content-editor op de onderwerptitel (Classic) of het drie-stippen-menu bij de activiteit (New Content Experience).
2. Zet de status op **Draft** (Classic) of schakel **Visibility** uit (New Content Experience).

Conceptonderwerpen zijn onzichtbaar voor studenten. Instructeurs en TA's zien ze nog steeds met een "Draft"-badge.

Om het onderwerp te beperken tot een specifieke groep of sectie:

1. Open het onderwerp.
2. Klik op het menu van de onderwerptitel > **Edit Properties In-place** (Classic) of **Edit** > **Restrictions** (New Content Experience).
3. Klik onder **Release Conditions** op **Create**.
4. Kies **Group enrollment** of **Section enrollment**, selecteer de groep/sectie en sla op.

Releasevoorwaarden stapelen op met de eigen roltoewijzing van FastComments. Studenten die het onderwerp niet kunnen zien, krijgen geen LTI-launch.

#### Wat studenten zien bij de eerste launch

Wanneer een student op het onderwerp klikt (of een HTML-onderwerp met een embed laadt):

1. Brightspace voert de LTI 1.3-launch op de achtergrond uit.
2. FastComments ontvangt de naam, het e-mailadres, de avatar-URL en de LMS-rol van de student en meldt deze automatisch aan. Er is geen FastComments-inlogprompt.
3. De reactiethread voor die resource link wordt weergegeven binnen het Brightspace-iframe.

Roltoewijzing bij launch:

- Brightspace `Administrator` wordt een FastComments **admin** voor de thread (volledige moderatie-, verwijder-, ban- en configuratietoegang).
- Brightspace `Instructor` wordt een FastComments **moderator** (vastpinnen, verbergen, verwijderen, bannen).
- Alle andere rollen (`Learner`, `TeachingAssistant`, enz.) worden standaardreactieplacers.

Reacties worden toegeschreven aan het Brightspace-account van de student. Als de student zijn naam of avatar in Brightspace wijzigt, synchroniseert de volgende LTI-launch de wijziging.

#### Publieke toegang vergrendelen (aanbevolen)

Standaard zijn FastComments-reactiegegevens publiek leesbaar. Iedereen die de URL of het API-eindpunt van een thread kan raden, kan de reacties bekijken, zelfs buiten Brightspace. Voor cursusdiscussies wilt u vrijwel zeker het bekijken beperken tot alleen ingeschreven studenten.

Open uw <a href="https://fastcomments.com/auth/my-account/customize-widget" target="_blank">widget-aanpassingspagina</a> en maak een regel aan met **Require SSO To View Comments** ingeschakeld, en stel vervolgens het beveiligingsniveau in op **Secure SSO**, zodat threads alleen via de ondertekende LTI-launch kunnen worden geladen.

Zie [Protecting Comment Threads With Single-Sign-On](/guide-customizations-and-configuration.html#sso-require-to-view-comments) voor de volledige uitleg, inclusief hoe u de regel kunt beperken tot één domein of pagina.

#### Iframe-hoogte en formaatwijziging

FastComments zendt het `org.imsglobal.lti.frameResize` postMessage uit bij elke thread-render en bij contentwijzigingen (nieuwe reactie, reacties uitklappen). Brightspace luistert naar dit bericht en past de iframe-hoogte aan, zodat de thread niet wordt afgesneden en geen interne scrollbar toont.

Als het iframe op een vaste, korte hoogte blijft:

- Controleer of de cursus via HTTPS wordt geladen. De postMessage-listener van Brightspace wijst mixed-content-frames af.
- Controleer of geen enkele browserextensie het postMessage-kanaal blokkeert.
- Voor inline embeds in een HTML-onderwerp mag de omliggende HTML het iframe niet in een container met vaste hoogte wikkelen. Verwijder eventuele inline `style="height: ..."` van het bovenliggende element.

#### Brightspace-specifieke valkuilen

**Tool verschijnt niet in de Add Existing-kiezer.** De deployment is niet ingeschakeld voor de org-eenheid van deze cursus. Een beheerder moet de org-eenheid (of een bovenliggende) toevoegen aan de lijst **Org Units** van de deployment. Alleen toolregistratie is niet voldoende; de deployment bepaalt welke cursussen de tool zien.

**`deployment_id`-mismatch bij launch.** FastComments pinnt de eerste `deployment_id` die het ziet voor een registratie (TOFU). Als een beheerder de oorspronkelijke deployment verwijdert en een nieuwe aanmaakt, worden launches vanaf de nieuwe deployment afgewezen met een deployment-mismatch-fout. De oplossing is om FastComments opnieuw te registreren (genereer een nieuwe registratie-URL (<a href="https://fastcomments.com/auth/my-account/lti-config" target="_blank">verkrijg die hier</a>) en voer Dynamic Registration opnieuw uit); het oude configuratierecord wordt vervangen.

**Tool lanceert maar toont "Invalid LTI launch".** De cursus bevindt zich in een andere tenant/org-structuur dan de deployment dekt, of de deployment is na registratie uitgeschakeld. Controleer opnieuw de **Enabled**-schakelaar onder **Admin Tools** > **Manage Extensibility** > **LTI Advantage** > FastComments en de org-eenheidlijst van de deployment.

**Namen en rollen ontbreken in FastComments.** Brightspace stuurt LTI-launches met Names and Role Provisioning Services (NRPS)-claims. Als een cursus is opgewaardeerd vanaf een oudere LTI 1.1-link, ontbreken de `name`- en `email`-claims in de launch. Voeg het FastComments-onderwerp opnieuw toe via **Add Existing** (migreer de oude link niet) zodat de launch LTI 1.3 gebruikt.

**Embed toont een inlogscherm in plaats van auto-SSO.** Het HTML-onderwerp is ingevoegd als een plat `<iframe>` dat naar FastComments verwijst in plaats van via **Insert Stuff** > **LTI Advantage**. Platte iframes slaan de LTI-launch over en leiden gebruikers naar de publieke FastComments-pagina. Verwijder het iframe en voeg het opnieuw in via de Insert Stuff-flow.
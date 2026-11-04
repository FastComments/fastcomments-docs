Zodra een beheerder FastComments als LTI 1.3 Advantage-tool heeft geregistreerd en het instellingsbeleid heeft goedgekeurd, voegen docenten het toe aan cursussen via de standaard Blackboard-plaatsingspunten. De exacte stappen verschillen tussen Ultra Course View en Original Course View, dus beide worden hieronder behandeld.

#### Ultra Course View

Ultra Course View is sinds 2026 de standaard in Blackboard Learn SaaS.

1. Open de cursus en ga naar de pagina **Course Content**.
2. Beweeg met de muis of tik op de plek waar je de commentaardraad in de structuur wilt plaatsen en klik op de paarse knop **+** (Add content).
3. Kies **Content Market**. Het Content Market-paneel toont elke goedgekeurde LTI-tool en Building Block-plaatsing voor jouw instelling.
4. Zoek de tegel **FastComments** en klik erop. Blackboard maakt een content-item aan op de positie waar je het menu **+** opende.
5. Het item verschijnt standaard in de structuur als vermelding "Visible to students" voor docenten die **Hide from students** uit hebben staan als persoonlijke standaardinstelling. Als jouw standaardinstelling **Hidden** is, wordt het item verborgen aangemaakt en kun je de zichtbaarheidskeuze op de itemrij omschakelen wanneer je er klaar voor bent.
6. Om het item te hernoemen, klik je op de titel in de structuur en typ je een nieuw label. De titel die studenten in de structuur zien is onafhankelijk van de FastComments-thread-identificatie, dus hernoemen is op elk moment veilig.

Als je **Content Market** niet als optie ziet, heeft jouw instelling de plaatsing verborgen. Je bereikt dezelfde kiezer ook via **More tools** in hetzelfde menu **+** onder de groep **LTI Tools**.

#### Original Course View

Original Course View wordt nog steeds ondersteund in Learn SaaS en blijft de primaire ervaring voor zelf-gehoste Learn 9.1-sites op de Q4 2024 CU-releaselijn.

1. Open de cursus en ga naar een **Content Area** (bijvoorbeeld het standaard **Information**- of **Content**-gebied in het cursusmenu).
2. Zet **Edit Mode** aan met de schakelaar rechtsboven op de pagina.
3. Klik op **Build Content** in de actiebalk.
4. Klik onder het submenu **Learning Tools** op **FastComments**. Het submenu Learning Tools wordt gevuld vanuit LTI 1.3-toolplaatsingen nadat een beheerder de tool heeft geregistreerd. Als je het niet ziet, raadpleeg dan de sectie met valkuilen hieronder.
5. Stel in het formulier **Create FastComments** het volgende in:
   - **Name**: het label dat studenten in het contentgebied zien.
   - **Description**: optionele tekst die boven de ingesloten draad wordt getoond.
   - **Permit Users to View this Content**: Ja/Nee-beschikbaarheidsschakelaar.
   - **Track Number of Views**: schakel in als je de statistieken per item van Blackboard wilt. FastComments draait zijn eigen analytics onafhankelijk.
   - **Date and Time Restrictions**: optionele vensters **Display After** / **Display Until**.
6. Verzenden. De tool verschijnt als klikbaar item in het contentgebied.

#### Insluiten in een Item of Document

In beide cursusweergaven sluiten docenten FastComments inline in de body van een Item, Document of elk rich-text-veld in via de LTI Advantage-knop van de Content Editor.

Ultra Course View:

1. Maak een **Document** aan of bewerk er een.
2. Klik op **Add content** in de documentbody waar je de draad wilt laten verschijnen.
3. Open in de werkbalk van de editor het menu **Insert content** en klik op **Content Market** (het LTI Advantage / Deep Linking-toegangspunt).
4. Kies **FastComments**. FastComments retourneert een deep-link-payload en Blackboard voegt een ingesloten blok toe in de documentbody op de cursorpositie.
5. Sla het document op. Studenten zien de draad inline weergegeven terwijl ze erlangs scrollen.

Original Course View:

1. Bewerk elk item met een rich-text-body.
2. Klik in de werkbalk van de Content Editor op het plus-icoon **Add Content** en kies **Content Market** (gelabeld **Add Content from External Tool** in oudere Q4 2024 CU's).
3. Kies **FastComments**. De editor voegt een placeholder-blok in dat verwijst naar de deep-linked resource.
4. Verzend het item.

Elke deep-link-embed produceert zijn eigen FastComments-thread, dus een Item met twee ingesloten FastComments-blokken heeft twee onafhankelijke commentaarstromen.

#### Zichtbaarheid, Release Conditions en Groepsbeperkingen

FastComments-content-items gedragen zich zoals elk ander Blackboard-content-item wat betreft de toegangscontrolleregels die erbovenop worden toegepast.

- Ultra: klik op de zichtbaarheidskeuze op de rij (**Visible to students**, **Hidden from students**, **Conditional availability**). Conditional availability ondersteunt datum/tijd-vensters, prestatieregels op gradebook-items en ledenregels op cursusgroepen.
- Original: open het contextmenu van het item en kies **Adaptive Release** of **Adaptive Release: Advanced** om de tool te beperken op datum, lidmaatschap, cijfer of beoordelingsstatus. Gebruik **Set Group Availability** op het item om te beperken tot specifieke cursusgroepen.

FastComments respecteert wat de gate van Blackboard beslist. Als Blackboard het item voor een student verbergt, vindt de LTI-launch nooit plaats voor die student en verschijnt hij niet in de moderatorweergave.

#### Gedrag van het Gradebook

FastComments rapporteert geen cijfers terug via LTI Advantage Assignment and Grade Services. Er wordt automatisch geen cijferkolom aangemaakt voor FastComments-content-items.

Als je Blackboard-tenant is geconfigureerd om automatisch een gradebook-kolom aan te maken voor elk nieuw content-item, ongeacht de beoordelingsmetadata, verschijnt er toch een lege kolom. Om die te verbergen:

- Ultra: open de **Gradebook**, klik op de kolomkop, kies **Edit** en zet **Show to students** en **Include in calculations** uit. Of gebruik **Delete** als jouw instelling het verwijderen van kolommen voor niet-beoordeelde items toestaat.
- Original: open de **Grade Center**, klik op het chevron-pictogram van de kolom, kies **Hide from Users (on/off)** en optioneel **Hide from Instructor View** onder **Column Organization**.

#### Wat Studenten Zien

Wanneer een student het FastComments-item opent of naar een ingesloten blok scrolt:

1. Blackboard start het LTI 1.3-bericht naar FastComments. De student is aangemeld via SSO met zijn Blackboard-identiteit (naam, e-mail, avatar, rol) zonder een inlogformulier te zien.
2. De commentaardraad wordt weergegeven in de iframe. Threading, reacties, vermeldingen en reacties zijn allemaal beschikbaar op basis van de widgetinstellingen die in FastComments zijn geconfigureerd.
3. Hun opmerkingen worden toegeschreven aan hun Blackboard-account. Als de student later zijn naam of foto in Blackboard wijzigt, wordt het FastComments-profiel bij de volgende launch bijgewerkt.

Roltoewijzing van Blackboard naar FastComments:

- **System Administrator** en **Course Builder** worden gemapt naar FastComments **admin**.
- **Instructor** en **Teaching Assistant** worden gemapt naar FastComments **moderator**.
- **Student**, **Guest** en **Observer** worden gemapt naar FastComments **commenter**.

Moderators zien moderatieopties (pin, hide, ban, delete) inline bij elke opmerking in de draad.

#### Openbare Toegang Blokkeren (Aanbevolen)

Standaard zijn FastComments-commentaargegevens openbaar leesbaar. Iedereen die de URL van een draad of het API-eindpunt kan raden, kan de opmerkingen bekijken, zelfs buiten Blackboard. Voor cursusdiscussies wil je het bekijken vrijwel zeker beperken tot alleen ingeschreven studenten.

Open je <a href="https://fastcomments.com/auth/my-account/customize-widget" target="_blank">widget-aanpassingspagina</a> en maak een regel aan met **Require SSO To View Comments** ingeschakeld, stel vervolgens het beveiligingsniveau in op **Secure SSO** zodat draden alleen via de ondertekende LTI-launch kunnen worden geladen.

Zie [Commentaardraden beveiligen met Single-Sign-On](/guide-customizations-and-configuration.html#sso-require-to-view-comments) voor de volledige uitleg, inclusief hoe je de regel beperkt tot één domein of pagina.

#### Thread-Scoping

FastComments scope elke draad op **(Blackboard-host, course ID, resource link ID)**. Twee FastComments-items in dezelfde cursus produceren twee draden. Hetzelfde item dat over twee cursusshells wordt gekopieerd (bijvoorbeeld via course copy) produceert twee draden, omdat Blackboard tijdens het kopiëren een nieuwe resource link ID uitgeeft. Om een gedeelde draad te behouden over cursuskopieën, gebruik je Deep Linking met een expliciete thread-URN die in FastComments is geconfigureerd vóór het starten van de kopie.

#### Blackboard-Specifieke Valkuilen

**FastComments-tegel ontbreekt in het Build Content-menu (Original) of Content Market (Ultra).** De beheerder heeft de tool goedgekeurd, maar er blokkeert een instellingsbeleid de betreffende plaatsing. Ga naar **Administrator Panel** > **Integrations** > **LTI Tool Providers**, bewerk de FastComments-vermelding en bevestig dat zowel **Course Content Tool** (Original) als **Course Content Tool - allow students** / **Deep Linking content tool** (Ultra) plaatsingen zijn ingeschakeld. Sla op en ververs de cursuspagina.

**Foutmelding "Tool not configured for this context" of "Tool is not deployed" bij launch.** Het tijdens dynamische registratie geregistreerde deployment-scope komt niet overeen met de instellingscontext waartoe de cursus behoort. Controleer in de tool provider-vermelding van Blackboard of de **Deployment ID** overeenkomt met wat FastComments toont op zijn LTI 1.3 Configuration-pagina voor deze tenant. Als ze verschillen, verwijder dan de plaatsing en voer de dynamische registratie opnieuw uit vanaf een verse registratie-URL (<a href="https://fastcomments.com/auth/my-account/lti-config" target="_blank">verkrijg die hier</a>).

**Iframe-hoogte lijkt vast te staan of content wordt afgesneden.** Sommige Blackboard-tenants worden geleverd met een strikt Content Security Policy dat de standaard LTI iframe-resize postMessage blokkeert. FastComments zendt zowel het Canvas-stijl `lti.frameResize`-bericht als het IMS-spec-formaat `org.imsglobal.lti.frameResize`-bericht uit voor maximale compatibiliteit, maar een CSP-override op tenantniveau blokkeert de listener aan de ouderzijde. Vraag je beheerder te bevestigen dat `*.fastcomments.com` op de LTI-tool-allowlist staat en dat er geen aangepaste CSP-header postMessage-events verwijdert. Het formaat wijzigen werkt daarna zonder verdere configuratie.

**Course copy dupliceert draden.** Blackboard course copy geeft nieuwe resource link ID's uit voor LTI-plaatsingen, dus gekopieerde cursussen beginnen met lege draden. Dit is verwacht. Als je wilt dat de gekopieerde cursus de oorspronkelijke draad overneemt, stel dan Deep Linking in met een expliciete thread-URN vóór het kopiëren, of neem contact op met FastComments-ondersteuning om thread-ID's bulksgewijs te remappen.

**Student ziet een generieke Blackboard-foutmelding bij launch.** De oorzaak is een ontbrekende of verouderde `email`-claim. Bevestig dat het instellingsbeleid voor FastComments **Role**, **Name** en **Email Address** heeft ingeschakeld onder **User Fields to Send**. Sla op en start daarna opnieuw in een verse browsersessie.
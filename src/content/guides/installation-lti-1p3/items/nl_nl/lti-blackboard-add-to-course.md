Zodra een beheerder FastComments heeft geregistreerd als een LTI 1.3 Advantage-tool en de instellingbeleid heeft goedgekeurd, voegen instructeurs het toe aan cursussen via de standaard Blackboard-plaatsingspunten. De exacte stappen verschillen tussen Ultra Course View en Original Course View, dus beide worden hieronder behandeld.

#### Ultra Course View

Ultra Course View is de standaard in Blackboard Learn SaaS vanaf 2026.

1. Open de cursus en ga naar de **Course Content**-pagina.
2. Zweef of tik op de plek in de outline waar je de commentaardraad wilt plaatsen en klik op de paarse **+** (Inhoud toevoegen)-knop.
3. Kies **Content Market**. Het Content Market-paneel toont alle goedgekeurde LTI-tools en Building Block-plaatsen voor jouw instelling.
4. Zoek de **FastComments** tegel en klik erop. Blackboard maakt een inhoudsitem op de positie waar je het **+**-menu hebt geopend.
5. Het item verschijnt standaard in de outline als een vermelding "Visible to students" voor instructeurs die **Hide from students** als persoonlijke standaard uit hebben staan. Als jouw standaard **Hidden** is, wordt het item verborgen aangemaakt en zet je de zichtbaarheidsschakelaar in de itemrij aan wanneer je er klaar voor bent.
6. Om het item een andere naam te geven, klik je op de titel in de outline en typ je een nieuw label. De titel die studenten in de outline zien is onafhankelijk van de FastComments-threadidentifier, dus het hernoemen is op elk moment veilig.

Als je **Content Market** niet als optie ziet, heeft jouw instelling de plaatsing verborgen. Je bereikt dezelfde picker ook via **More tools** in hetzelfde **+**-menu onder de groep **LTI Tools**.

#### Original Course View

Original Course View wordt nog steeds ondersteund in Learn SaaS en blijft de primaire ervaring voor zelf-gehoste Learn 9.1-sites op de Q4 2024 CU-releaselijn.

1. Open de cursus en ga naar een **Content Area** (bijvoorbeeld het standaard **Information** of **Content**-gebied in het cursussenmenu).
2. Zet **Edit Mode** aan met de schakelaar rechtsboven op de pagina.
3. Klik op **Build Content** in de actiebalk.
4. Klik onder het submenu **Learning Tools** op **FastComments**. Het submenu Learning Tools wordt gevuld vanuit LTI 1.3-toolplaatsingen nadat een beheerder de tool registreert. Als je het niet ziet, zie de sectie met aandachtspunten hieronder.
5. Vul op het formulier **Create FastComments** in:
   - **Name**: het label dat studenten in het contentgebied zien.
   - **Description**: optionele tekst die boven de ingesloten thread wordt weergegeven.
   - **Permit Users to View this Content**: Ja/Nee beschikbaarheidsschakelaar.
   - **Track Number of Views**: inschakelen als je Blackboard's per-item weergave-statistieken wilt. FastComments gebruikt onafhankelijk eigen analytics.
   - **Date and Time Restrictions**: optionele **Display After** / **Display Until** vensters.
6. Verzenden. De tool verschijnt als een aanklikbaar item in het contentgebied.

#### Insluiten binnen een item of document

In beide cursusweergaven kunnen instructeurs FastComments inline insluiten in de body van een Item, Document of elk rich-text veld via de Content Editor's LTI Advantage-knop.

Ultra Course View:

1. Maak of bewerk een **Document**.
2. Klik op **Add content** binnen de documentbody op de plek waar je de thread wilt laten verschijnen.
3. Open in de editorwerkbalk het menu **Insert content** en klik op **Content Market** (de LTI Advantage / Deep Linking ingang).
4. Kies **FastComments**. FastComments retourneert een deep-link payload en Blackboard voegt een ingesloten blok in de documentbody op de cursorpositie in.
5. Sla het document op. Studenten zien de thread inline gerenderd terwijl ze erlangs scrollen.

Original Course View:

1. Bewerk elk item met een rich-text body.
2. Klik in de Content Editor-werkbalk op het pluspictogram **Add Content** en kies **Content Market** (gelabeld als **Add Content from External Tool** in oudere Q4 2024 CUs).
3. Kies **FastComments**. De editor voegt een tijdelijke aanduidingblok in dat verwijst naar de deep-linked resource.
4. Dien het item in.

Elke deep-link embed maakt zijn eigen FastComments-thread, dus een Item met twee ingesloten FastComments-blokken heeft twee onafhankelijke commentaarstromen.

#### Zichtbaarheid, vrijgaveregels en groepsbeperkingen

FastComments-inhoudsitems gedragen zich als elk ander Blackboard-inhoudsitem voor de toegangscontroles die erop worden toegepast.

- Ultra: klik op de zichtbaarheidsschakelaar in de rij (**Visible to students**, **Hidden from students**, **Conditional availability**). Conditional availability ondersteunt datum-/tijdvensters, prestatieregels tegen gradebook-items en lidmaatschapregels tegen cursusgroepen.
- Original: open het contextmenu van het item en kies **Adaptive Release** of **Adaptive Release: Advanced** om de tool te begrenzen op datum, lidmaatschap, cijfer of reviewstatus. Gebruik **Set Group Availability** op het item om te beperken tot specifieke cursusgroepen.

FastComments respecteert wat het Blackboard-poort stelt. Als Blackboard het item voor een student verbergt, wordt de LTI-launch nooit voor die student uitgevoerd en verschijnt die student niet in de moderatorweergave.

#### Gedrag in het Gradebook

FastComments rapporteert geen cijfers terug via LTI Advantage Assignment and Grade Services. Er wordt geen cijferkolom automatisch aangemaakt voor FastComments-inhoudsitems.

Als jouw Blackboard-tenant is geconfigureerd om voor elk nieuw inhoudsitem automatisch een gradebook-kolom aan te maken ongeacht grading-metadata, verschijnt er toch een lege kolom. Om deze te verbergen:

- Ultra: open het **Gradebook**, klik op de kolomkop, kies **Edit**, en zet **Show to students** en **Include in calculations** uit. Of gebruik **Delete** als jouw instelling het verwijderen van kolommen voor niet-gegrade items toestaat.
- Original: open het **Grade Center**, klik op de chevron van de kolom, kies **Hide from Users (on/off)**, en eventueel **Hide from Instructor View** onder **Column Organization**.

#### Wat studenten zien

Wanneer een student het FastComments-item opent of naar een ingesloten blok scrollt:

1. Blackboard lanceert het LTI 1.3-bericht naar FastComments. De student wordt via SSO ingelogd met zijn/haar Blackboard-identiteit (naam, e-mail, avatar, rol) zonder een loginformulier te zien.
2. De commentaardraad wordt weergegeven in de iframe. Threading, reacties, mentions en reacties zijn allemaal beschikbaar op basis van de comment widget-instellingen die in FastComments zijn geconfigureerd.
3. Hun opmerkingen worden toegeschreven aan hun Blackboard-account. Als de student later zijn/haar naam of foto in Blackboard wijzigt, werkt de volgende launch het FastComments-profiel bij.

Rolmapping van Blackboard naar FastComments:

- **System Administrator** en **Course Builder** worden gemapt naar FastComments **admin**.
- **Instructor** en **Teaching Assistant** worden gemapt naar FastComments **moderator**.
- **Student**, **Guest**, en **Observer** worden gemapt naar FastComments **commenter**.

Moderators zien moderatiebedieningselementen (pin, hide, ban, delete) inline bij elk commentaar in de thread.

#### Thread-afbakening

FastComments beperkt elke thread per **(Blackboard host, course ID, resource link ID)**. Twee FastComments-items in dezelfde cursus produceren twee threads. Hetzelfde item gekopieerd naar twee cursusomgevingen (bijvoorbeeld via course copy) produceert twee threads, omdat Blackboard tijdens het kopiëren een nieuwe resource link ID uitgeeft. Om een gedeelde thread te behouden bij course copies, gebruik Deep Linking met een expliciete thread-URN die in FastComments is geconfigureerd vóór het starten van de copy.

#### Blackboard-specifieke aandachtspunten

**FastComments-tegel ontbreekt in het Build Content-menu (Original) of Content Market (Ultra).** De beheerder heeft de tool goedgekeurd maar een instellingbeleid heeft de relevante plaatsing geblokkeerd. Ga naar **Administrator Panel** > **Integrations** > **LTI Tool Providers**, bewerk de FastComments-vermelding en bevestig dat zowel **Course Content Tool** (Original) als **Course Content Tool - allow students** / **Deep Linking content tool** (Ultra) plaatsingen zijn ingeschakeld. Sla op en vernieuw de cursuspagina.

**"Tool not configured for this context" of "Tool is not deployed" fout bij launch.** De deployment scope die tijdens dynamische registratie is geregistreerd komt niet overeen met de instellingcontext waartoe de cursus behoort. Controleer in de tool provider-vermelding van Blackboard of de **Deployment ID** overeenkomt met wat FastComments op zijn LTI 1.3 Configuration-pagina voor deze tenant toont. Als ze verschillen, verwijder de plaatsing en voer de dynamische registratie opnieuw uit vanaf een verse registratie-URL.

**Iframe-hoogte lijkt vast of content wordt afgesneden.** Sommige Blackboard-tenants leveren een strikte Content Security Policy die de standaard LTI iframe-resize postMessage blokkeert. FastComments zendt zowel het Canvas-stijl `lti.frameResize`-bericht als het IMS-spec-form `org.imsglobal.lti.frameResize`-bericht om compatibiliteit te maximaliseren, maar een tenant-niveau CSP-override blokkeert de parent-listener. Vraag je beheerder te bevestigen dat `*.fastcomments.com` op de LTI-tool allowlist staat en dat geen aangepaste CSP-header postMessage-events wegfiltert. Daarna werkt het schalen zonder verdere configuratie.

**Course copy dupliceert threads.** Blackboard course copy geeft nieuwe resource link IDs uit voor LTI-plaatsingen, dus gekopieerde cursussen beginnen met lege threads. Dit is verwacht gedrag. Als je wilt dat de gekopieerde cursus de oorspronkelijke thread erft, stel Deep Linking in met een expliciete thread-URN vóór het kopiëren, of neem contact op met FastComments-ondersteuning om thread-ID's in bulk te remappen.

**Student ziet een generieke Blackboard-fout bij launch.** De oorzaak is een ontbrekende of verouderde `email`-claim. Controleer het instellingbeleid voor FastComments en zorg dat **Role**, **Name**, en **Email Address** zijn ingeschakeld onder **User Fields to Send**. Sla op en start daarna opnieuw in een verse browsersessie.
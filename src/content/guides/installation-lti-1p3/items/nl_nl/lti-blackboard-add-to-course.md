Zodra een beheerder FastComments heeft geregistreerd als een LTI 1.3 Advantage-tool en de institutionele beleidsregels heeft goedgekeurd, voegen docenten het toe aan cursussen via de standaard Blackboard-plaatsingspunten. De exacte stappen verschillen tussen Ultra Course View en Original Course View, dus beide worden hieronder behandeld.

#### Ultra Course View

Ultra Course View is de standaard in Blackboard Learn SaaS vanaf 2026.

1. Open de cursus en ga naar de pagina **Course Content**.
2. Zweef of tik waar je de discussiedraad in de inhoudsopgave wilt plaatsen en klik op de paarse **+** (Add content) knop.
3. Kies **Content Market**. Het Content Market-paneel toont elk goedgekeurd LTI-gereedschap en Building Block-plaatsing voor uw instelling.
4. Zoek de tegel **FastComments** en klik erop. Blackboard maakt een inhoudsitem op de positie waar je het **+**-menu opende.
5. Het item verschijnt standaard in de inhoudsopgave als een vermelding "Visible to students" voor docenten die **Hide from students** uit hebben staan als hun persoonlijke standaard. Als jouw standaard **Hidden** is, wordt het item verborgen aangemaakt en zet je de zichtbaarheidsschakelaar op de itemrij aan wanneer je er klaar voor bent.
6. Om het item een andere naam te geven, klik je op de titel in de inhoudsopgave en typ je een nieuw label. De titel die studenten in de inhoudsopgave zien is onafhankelijk van de FastComments-threadidentifier, dus hernoemen is op elk moment veilig.

Als je **Content Market** niet als optie ziet, heeft jouw instelling de plaatsing verborgen. Je bereikt dezelfde keuze ook via **More tools** in hetzelfde **+**-menu onder de groep **LTI Tools**.

#### Original Course View

Original Course View wordt nog steeds ondersteund in Learn SaaS en blijft de primaire ervaring voor zelf-gehoste Learn 9.1-sites op de Q4 2024 CU-releaselijn.

1. Open de cursus en ga naar een **Content Area** (bijvoorbeeld het standaard **Information** of **Content**-gebied in het cursusmenu).
2. Zet **Edit Mode** aan met de schakelaar rechtsboven op de pagina.
3. Klik op **Build Content** in de actiebalk.
4. Klik onder het submenu **Learning Tools** op **FastComments**. Het Learning Tools-submenu wordt gevuld vanuit LTI 1.3-toolplaatsingen nadat een beheerder de tool registreert. Als je het niet ziet, zie de sectie met veelvoorkomende problemen hieronder.
5. Op het formulier **Create FastComments**, stel in:
   - **Name**: het label dat studenten in het contentgebied zien.
   - **Description**: optionele tekst die boven de ingesloten draad wordt getoond.
   - **Permit Users to View this Content**: Ja/Nee beschikbaarheidsschakelaar.
   - **Track Number of Views**: inschakelen als je Blackboard's statistieken per item wilt. FastComments draait zijn eigen analytics onafhankelijk.
   - **Date and Time Restrictions**: optionele **Display After** / **Display Until** vensters.
6. Verzenden. De tool verschijnt als een klikbaar item in het contentgebied.

#### Embedding Inside an Item or Document

In beide course views integreren docenten FastComments inline in de body van een Item, Document of elk rich-textveld via de LTI Advantage-knop in de Content Editor.

Ultra Course View:

1. Maak of bewerk een **Document**.
2. Klik op **Add content** in de documenttekst waar je de draad wilt laten verschijnen.
3. Open in de editorwerkbalk het menu **Insert content** en klik op **Content Market** (de LTI Advantage / Deep Linking toegangspoort).
4. Kies **FastComments**. FastComments retourneert een deep-link payload en Blackboard plaatst een ingesloten blok in de documenttekst op de cursorpositie.
5. Sla het document op. Studenten zien de draad inline gerenderd terwijl ze er langs scrollen.

Original Course View:

1. Bewerk elk item met een rich-text body.
2. Klik in de Content Editor-werkbalk op het plus-icoon **Add Content** en kies **Content Market** (gelabeld **Add Content from External Tool** in oudere Q4 2024 CUs).
3. Kies **FastComments**. De editor voegt een tijdelijke plaatsaanduiding toe die verwijst naar de deep-linked resource.
4. Verstuur het item.

Elke deep-link embed maakt zijn eigen FastComments-thread, dus een Item met twee ingesloten FastComments-blokken heeft twee onafhankelijke commentstreamen.

#### Visibility, Release Conditions, and Group Restrictions

FastComments-inhoudsitems gedragen zich als elk ander Blackboard-inhoudsitem wat betreft de toegangscontroles die erop worden toegepast.

- Ultra: klik op de zichtbaarheidsschakelaar op de rij (**Visible to students**, **Hidden from students**, **Conditional availability**). Conditional availability ondersteunt datum/tijd-vensters, prestatieregels tegen gradebook-items en ledenregels tegen cursusgroepen.
- Original: open het contextmenu van het item en kies **Adaptive Release** of **Adaptive Release: Advanced** om de tool te beperken op datum, lidmaatschap, cijfer of beoordelingsstatus. Gebruik **Set Group Availability** op het item om te beperken tot specifieke cursusgroepen.

FastComments respecteert wat het Blackboard-poort besluit. Als Blackboard het item voor een student verbergt, gebeurt de LTI-launch nooit voor die student en verschijnt die niet in de moderatorweergave.

#### Gradebook Behavior

FastComments rapporteert geen cijfers terug via LTI Advantage Assignment and Grade Services. Er wordt geen cijferkolom automatisch aangemaakt voor FastComments-inhoudsitems.

Als jouw Blackboard-tenant is geconfigureerd om automatisch een gradebook-kolom aan te maken voor elk nieuw inhoudsitem ongeacht grading-metadata, verschijnt er toch een lege kolom. Om deze te verbergen:

- Ultra: open het **Gradebook**, klik op de kolomkop, kies **Edit**, en zet **Show to students** plus **Include in calculations** uit. Of gebruik **Delete** als jouw instelling het verwijderen van kolommen voor niet-gecodeerde items toestaat.
- Original: open het **Grade Center**, klik op de chevron van de kolom, kies **Hide from Users (on/off)**, en optioneel **Hide from Instructor View** onder **Column Organization**.

#### What Students See

Wanneer een student het FastComments-item opent of naar een ingesloten blok scrolt:

1. Blackboard lanceert het LTI 1.3-bericht naar FastComments. De student wordt aangemeld via SSO met hun Blackboard-identiteit (naam, e-mail, avatar, rol) zonder een inlogformulier te zien.
2. De commentdraad rendert in de iframe. Threading, reacties, mentions en reacties zijn allemaal beschikbaar op basis van de commentwidget-instellingen geconfigureerd in FastComments.
3. Hun opmerkingen worden toegeschreven aan hun Blackboard-account. Als de student later zijn naam of foto in Blackboard aanpast, werkt de volgende launch het FastComments-profiel bij.

Rolmapping van Blackboard naar FastComments:

- **System Administrator** en **Course Builder** mappen naar FastComments **admin**.
- **Instructor** en **Teaching Assistant** mappen naar FastComments **moderator**.
- **Student**, **Guest**, en **Observer** mappen naar FastComments **commenter**.

Moderators zien moderatiecontroles (pin, hide, ban, delete) inline bij elke opmerking in de draad.

#### Thread Scoping

FastComments scopeert elke thread by **(Blackboard host, course ID, resource link ID)**. Twee FastComments-items in dezelfde cursus produceren twee threads. Hetzelfde item gekopieerd naar twee cursusomgevingen (bijvoorbeeld via course copy) produceert twee threads, omdat Blackboard tijdens de kopie een nieuw resource link ID uitgeeft. Om een gedeelde thread te behouden over cursuskopieën, gebruik Deep Linking met een expliciete thread URN geconfigureerd in FastComments vóór het starten van de kopie.

#### Blackboard-Specific Gotchas

**FastComments-tegel ontbreekt in het Build Content-menu (Original) of Content Market (Ultra).** De beheerder heeft de tool goedgekeurd maar een institutioneel beleid achtergelaten dat de relevante plaatsing blokkeert. Ga naar **Administrator Panel** > **Integrations** > **LTI Tool Providers**, bewerk de FastComments-vermelding en bevestig dat zowel **Course Content Tool** (Original) als **Course Content Tool - allow students** / **Deep Linking content tool** (Ultra) plaatsingen zijn ingeschakeld. Sla op en vernieuw de cursuspagina.

**"Tool not configured for this context" of "Tool is not deployed" fout bij lancering.** De deployment scope die tijdens dynamische registratie is geregistreerd, komt niet overeen met de institutionele context waartoe de cursus behoort. Controleer in de tool provider-vermelding van Blackboard of de **Deployment ID** overeenkomt met wat FastComments op zijn LTI 1.3 Configuration-pagina voor deze tenant toont. Als ze verschillen, verwijder dan de plaatsing en voer dynamische registratie opnieuw uit vanaf een verse registratie-URL (<a href="https://fastcomments.com/auth/my-account/lti-config" target="_blank">get it here</a>).

**Iframe-hoogte lijkt vast of inhoud wordt afgekapt.** Sommige Blackboard-tenants leveren een strikte Content Security Policy die de standaard LTI iframe-resize postMessage blokkeert. FastComments verzendt zowel het Canvas-stijl `lti.frameResize`-bericht als het IMS-spec-form `org.imsglobal.lti.frameResize`-bericht om compatibiliteit te maximaliseren, maar een tenant-niveau CSP-override blokkeert de parent-listener. Vraag je beheerder te bevestigen dat `*.fastcomments.com` op de LTI-tool-allowlist staat en dat geen aangepaste CSP-header postMessage-events wegfiltert. Daarna werkt het resizen zonder verdere configuratie.

**Course copy dupliceert threads.** Blackboard course copy geeft nieuwe resource link IDs uit voor LTI-plaatsingen, dus gekopieerde cursussen starten met lege threads. Dit is verwacht gedrag. Als je wilt dat de gekopieerde cursus de oorspronkelijke thread erft, stel dan Deep Linking in met een expliciete thread URN vóór het kopiëren, of neem contact op met FastComments-ondersteuning om thread-ID's in bulk te remappen.

**Student ziet een generieke Blackboard-fout bij lancering.** De oorzaak is een ontbrekende of verouderde `email`-claim. Bevestig dat het institutionele beleid voor FastComments **Role**, **Name**, en **Email Address** heeft ingeschakeld onder **User Fields to Send**. Sla op en start daarna opnieuw in een verse browsersessie.
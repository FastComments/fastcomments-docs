Zodra FastComments bij het platform is geregistreerd, voegen instructeurs het toe aan cursusinhoud via de standaardstroom voor externe tools van het platform. Deze pagina behandelt Sakai 23.x en Schoology Enterprise.

#### Sakai

**1. Voeg FastComments aan een site toe**

De sitebeheerder schakelt de tool per site in:

1. Open de site en klik op **Site Info** in de linker navigatie.
2. Klik op **Manage Tools**.
3. Scrol naar de lijst **External Tools** en zet **FastComments** aan.
4. Klik op **Continue**, controleer de lijst met tools en klik vervolgens op **Finish**.

FastComments verschijnt nu als een item in de linker navigatie van de site.

**2. De positie in de linker navigatie wijzigen**

Ga naar **Site Info** > **Tool Order**. Sleep **FastComments** naar de gewenste positie en klik op **Save**. Vanaf dit scherm kunt u ook het navigatielabel hernoemen en het voor studenten verbergen.

**3. Inline insluiten op een Lessons-pagina**

Om FastComments direct in een Lessons-pagina te plaatsen in plaats van als een op zichzelf staande link in de linker navigatie:

1. Open de **Lessons**-tool in de site.
2. Klik op **Add Content** > **Add External Tool**.
3. Selecteer **FastComments** in de lijst.
4. Als FastComments tijdens de registratie Deep Linking heeft geadverteerd, opent Sakai de inhoudsselector van de tool zodat u de thread kunt kiezen of labelen. Als Deep Linking niet was geadverteerd, voegt Sakai een standaard launch-link in.
5. Sla het Lessons-item op.

Elke ingesloten instantie krijgt zijn eigen thread, gescopeerd op die resource link.

**4. Machtigingen aanpassen voor studenten toegang**

Sakai beheert externe tool-launches via Realms. Om te bevestigen dat studenten FastComments kunnen starten:

1. Meld u aan als Sakai-admin en open **Administration Workspace** > **Realms**.
2. Open het relevante realm (bijvoorbeeld `!site.template.course` of het specifieke site-realm).
3. Bevestig dat de rol `access` `lti.launch` ingeschakeld heeft en dat de rolmachtigingen in de **external.tools** groep zijn toegekend.
4. Sla het realm op.

Voor site-niveau overschrijvingen kan de maintainer per rol de zichtbaarheid van de tool aanpassen via **Site Info** > **Tool Order** door FastComments per rol te verbergen of te tonen.

**5. Wat studenten zien**

Studenten klikken op het FastComments-item in de linker navigatie (of scrollen naar het ingesloten blok in Lessons) en komen direct in de threadweergave terecht. SSO is automatisch: Sakai verzendt de identiteit van de gebruiker in de LTI-launch en FastComments meldt ze aan onder hun Sakai-account.

Role mapping:

- Sakai `Instructor` -> FastComments moderator
- Sakai `Admin` (admin in Administration Workspace) -> FastComments admin
- Sakai `Student` / `access` -> FastComments commenter

**6. Sakai aandachtspunten**

- **Tool niet zichtbaar in Manage Tools.** Als FastComments niet verschijnt in de lijst External Tools, moet de Sakai-admin het toolregister openen (**Administration Workspace** > **External Tools** > **FastComments**) en **Stealthed** op `false` zetten. Stealthed-tools zijn verborgen in de per-site Manage Tools-picker.
- **Launches die falen in gedeelde-sessie browsers.** De portal CSRF-token van Sakai is gebonden aan de browsersessie. Als een student in twee Sakai-sites is aangemeld in verschillende tabbladen of een verouderde sessie heeft, geeft de launch een 403 terug. Oplossing: sluit andere Sakai-tabbladen, log uit, log opnieuw in en start de launch opnieuw. Admins kunnen ook `sakai.csrf.token.cache.ttl` verhogen als dit clusterbreed gebeurt.
- **Frame-insluiting.** Controleer of `lti.frameheight` in `sakai.properties` groot genoeg is (600 of hoger) zodat de comment-thread niet wordt afgesneden binnen een Lessons-pagina.

#### Schoology

Schoology Enterprise kent twee installatiescenario's. Bevestig welk scenario van toepassing is voordat u de tool aan een cursus toevoegt.

**1. Twee installatiescenario's**

- **(a) Installatie op ondernemingsniveau.** De Schoology System Administrator heeft FastComments op organisatieniveau geïnstalleerd en toegewezen aan alle cursussen of aan specifieke cursus-sjablonen. Instructeurs slaan de installatie over en gaan direct naar "Add Materials".
- **(b) Zelfinstallatie door de instructeur.** De instructeur installeert de tool in een enkele cursus via **Course Options** > **External Tools** > **Install LTI Apps**. Zelfinstallatie vereist dat de System Administrator de FastComments-app eerst op organisatieniveau heeft goedgekeurd.

**2. Voeg FastComments toe als cursusmateriaal**

Binnen de cursus:

1. Open de cursus en ga naar **Materials**.
2. Klik op **Add Materials** > **Add File/Link/External Tool**.
3. Kies **External Tool**.
4. Selecteer **FastComments** uit de lijst met geregistreerde tools.
5. Stel een **Name** in (dit is wat studenten zien in de materialenlijst) en een optionele **Description**.
6. Laat **Enable Grading** (grade passback) **OFF**. FastComments rapporteert geen cijfers terug aan Schoology, dus het inschakelen van grade passback creëert een lege kolom in het cijferboek.
7. Klik op **Submit**.

Het materiaal verschijnt nu in de materialenlijst van de cursus en opent de FastComments-thread wanneer erop wordt geklikt.

**3. Inline insluiten via de Rich Text-editor**

Als de System Administrator tijdens de registratie Deep Linking-plaatsing voor FastComments heeft ingeschakeld, kunnen instructeurs de comment-thread insluiten in elk Rich Text-veld (opdrachtaantekeningen, paginainhoud, discussie-inleidingen):

1. Open de Rich Text-editor op de doelpagina.
2. Klik op het pictogram **External Tool** (puzzelstuk) in de werkbalk.
3. Kies **FastComments**.
4. Configureer de insluiting in het deep-linking dialoog en klik op **Insert**.
5. Sla de pagina op.

Als de knop External Tool niet verschijnt in de Rich Text-editor, is Deep Linking uitgeschakeld voor deze tool op deze tenant. Zie de aandachtspunten hieronder.

**4. Zichtbaarheid en sectietoewijzingen**

Schoology bepaalt de beschikbaarheid van tools per sectie via Course Options:

1. Klik vanuit de cursus op **Course Options** > **External Tools**.
2. Voor elke geïnstalleerde LTI-app bepaalt u of deze beschikbaar is voor alle secties in de cursus of voor specifieke secties.
3. Om FastComments te beperken tot bepaalde secties, haalt u de vinkjes weg bij de secties die de tool niet mogen zien.
4. Sectie-niveau toegang bepaalt ook welke secties de **Add Materials** > **External Tool**-vermelding voor FastComments zien.

**5. Wat studenten zien**

Studenten klikken op het FastComments-materiaal (of scrollen naar de inline-insluiting) en komen in de threaded discussie terecht. SSO is automatisch via de Schoology LTI-launch onder hun Schoology-account.

Role mapping:

- Schoology `Administrator` -> FastComments admin
- Schoology `Instructor` -> FastComments moderator
- Schoology `Student` -> FastComments commenter

**6. Schoology aandachtspunten**

- **Alleen Enterprise.** Persoonlijke en gratis Schoology-accounts kunnen geen LTI 1.3-tools installeren. Als uw tenant op het gratis niveau zit, ontbreekt de optie **External Tools** in Course Options. Upgrade naar Schoology Enterprise om FastComments te gebruiken.
- **Deep Linking standaard uitgeschakeld door tenant.** Sommige Schoology-tenants beperken Deep Linking-plaatsing op organisatieniveau. In dat geval zien instructeurs alleen de workflow **Add Materials** > **External Tool** en niet de knop External Tool in de Rich Text-editor. Om inline-insluiting mogelijk te maken, gaat de System Administrator naar **System Settings** > **Integration** > **LTI 1.3** > **FastComments** en schakelt de **Content Item / Deep Linking**-plaatsing in, en slaat vervolgens op.
- **Sectie-toewijzing overschreven.** Als FastComments op ondernemingsniveau is toegewezen maar de instructeur ziet het niet in **Add Materials**, dan is de sectie van de cursus uitgesloten in de toewijzing op organisatieniveau. Vraag de System Administrator om de sectie toe te voegen aan de toewijzing van de FastComments-app.
- **Naam van materiaal versus identiteit van de thread.** Het hernoemen van het materiaal in Schoology verplaatst de comment-thread niet. Threads zijn gekoppeld aan de LTI resource link ID, dus een naamswijziging behoudt dezelfde thread; het verwijderen en opnieuw aanmaken van het materiaal creëert een nieuwe, lege thread.
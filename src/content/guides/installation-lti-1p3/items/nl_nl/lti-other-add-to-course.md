Zodra FastComments met het platform is geregistreerd, voegen instructeurs het toe aan cursusinhoud met de standaard externe tool-workflows van het platform. Deze pagina behandelt Sakai 23.x en Schoology Enterprise.

#### Beperk openbare toegang (aanbevolen)

Standaard zijn FastComments-reactiegegevens op beide platforms openbaar leesbaar. Iedereen die de URL of API-endpoint van een thread kan raden, kan de reacties bekijken, zelfs buiten Sakai of Schoology. Voor cursusdiscussies wil je vrijwel zeker de weergave beperken tot ingeschreven studenten alleen.

Open je <a href="https://fastcomments.com/auth/my-account/customize-widget" target="_blank">widget customization page</a> en maak een regel aan met **Require SSO To View Comments** ingeschakeld, stel vervolgens het beveiligingsniveau in op **Secure SSO** zodat threads alleen via de gesigneerde LTI-launch kunnen worden geladen.

Zie [Protecting Comment Threads With Single-Sign-On](/guide-customizations-and-configuration.html#sso-require-to-view-comments) voor de volledige walkthrough, inclusief hoe je de regel kunt beperken tot één domein of pagina.

#### Sakai

**1. Voeg FastComments toe aan een site**

De sitebeheerder schakelt de tool per site in:

1. Open de site en klik in de linker navigatie op **Site Info**.
2. Klik op **Manage Tools**.
3. Scroll naar de lijst **External Tools** en zet **FastComments** aan.
4. Klik op **Continue**, controleer de lijst met tools en klik vervolgens op **Finish**.

FastComments verschijnt nu als een item in de linker navigatie van de site.

**2. Hernoem of verplaats de linker-navigatievermelding**

Ga naar **Site Info** > **Tool Order**. Sleep **FastComments** naar de gewenste positie en klik op **Save**. Je kunt het navigatielabel hier ook hernoemen en verbergen voor studenten.

**3. Inline insluiten op een Lessons-pagina**

Om FastComments direct in een Lessons-pagina te plaatsen in plaats van als een op zichzelf staande linker-navigatietool:

1. Open de **Lessons**-tool in de site.
2. Klik op **Add Content** > **Add External Tool**.
3. Selecteer **FastComments** uit de lijst.
4. Als FastComments tijdens registratie Deep Linking adverteerde, opent Sakai de contentselector van de tool zodat je de thread kunt kiezen of labelen. Als Deep Linking niet werd geadverteerd, plaatst Sakai een standaard launch-link.
5. Sla het Lessons-item op.

Elke ingesloten instantie krijgt zijn eigen thread, gekoppeld aan die resource link.

**4. Machtigingsaanpassingen voor studententoegang**

Sakai regelt externe tool-launches via Realms. Om te bevestigen dat studenten FastComments kunnen starten:

1. Meld je aan als Sakai-admin en open **Administration Workspace** > **Realms**.
2. Open het relevante realm (bijvoorbeeld `!site.template.course` of het specifieke site-realm).
3. Bevestig dat de `access`-rol `lti.launch` heeft ingeschakeld en dat de rolpermissies in de groep **external.tools** zijn toegekend.
4. Sla het realm op.

Voor site-niveau overschrijvingen kan de beheerder per rol de zichtbaarheid van de tool aanpassen via **Site Info** > **Tool Order** door FastComments per rol te verbergen of weer te geven.

**5. Wat studenten zien**

Studenten klikken op het FastComments-item in de linker-navigatie (of scrollen naar het ingesloten Lessons-blok) en komen direct in de threaded comment-weergave. SSO gaat automatisch: Sakai stuurt de identiteit van de gebruiker in de LTI-launch en FastComments logt hen in onder hun Sakai-account.

Role mapping:

- Sakai `Instructor` -> FastComments moderator
- Sakai `Admin` (admin in Administration Workspace) -> FastComments admin
- Sakai `Student` / `access` -> FastComments commenter

**6. Sakai-valkuilen**

- **Tool niet zichtbaar in Manage Tools.** Als FastComments niet verschijnt in de lijst External Tools, moet de Sakai-admin het toolregister openen (**Administration Workspace** > **External Tools** > **FastComments**) en **Stealthed** op `false` zetten. Stealthed tools zijn verborgen in de per-site Manage Tools-picker.
- **Launches breken in gedeelde-sessie browsers.** De portal CSRF-token van Sakai is gebonden aan de browsersessie. Als een student in twee Sakai-sites in verschillende tabbladen is aangemeld of een verouderde sessie heeft, geeft de launch een 403 terug. Oplossing: sluit andere Sakai-tabbladen, log uit, log opnieuw in en start de launch opnieuw. Admins kunnen ook `sakai.csrf.token.cache.ttl` verhogen als dit cluster-breed gebeurt.
- **Frame-embedding.** Bevestig dat `lti.frameheight` in `sakai.properties` groot genoeg is (600 of meer) zodat de comment-thread niet wordt afgesneden binnen een Lessons-pagina.

#### Schoology

Schoology Enterprise kent twee installatiescenario's. Bevestig welk scenario van toepassing is voordat je de tool aan een cursus toevoegt.

**1. Twee installatiescenario's**

- **(a) Enterprise-level installatie.** De Schoology System Administrator heeft FastComments op organisatieniveau geïnstalleerd en toegewezen aan alle cursussen of aan specifieke cursus-sjablonen. Instructeurs slaan de installatie over en gaan direct naar "Add Materials".
- **(b) Instructeur zelf-installatie.** De instructeur installeert de tool in een enkele cursus via **Course Options** > **External Tools** > **Install LTI Apps**. Zelf-installatie vereist dat de System Administrator de FastComments-app op organisatieniveau heeft goedgekeurd.

**2. Voeg FastComments toe als cursusmateriaal**

Binnen de cursus:

1. Open de cursus en ga naar **Materials**.
2. Klik op **Add Materials** > **Add File/Link/External Tool**.
3. Kies **External Tool**.
4. Selecteer **FastComments** uit de lijst met geregistreerde tools.
5. Stel een **Name** in (dit is wat studenten in de materialenlijst zien) en een optionele **Description**.
6. Laat **Enable Grading** (grade passback) **OFF**. FastComments rapporteert geen cijfers terug aan Schoology, dus het inschakelen van grade passback maakt een lege kolom in het gradebook.
7. Klik op **Submit**.

Het materiaal verschijnt nu in de cursusmaterialenlijst en opent de FastComments-thread wanneer erop geklikt wordt.

**3. Inline insluiten via de Rich Text-editor**

Als de System Administrator tijdens registratie Deep Linking-plaatsing voor FastComments heeft ingeschakeld, kunnen instructeurs de comment-thread insluiten in elk Rich Text-veld (opdrachtinstructies, pagina-inhoud, discussieprikkels):

1. Open de Rich Text-editor op de doelpagina.
2. Klik op het pictogram **External Tool** (puzzelstuk) in de werkbalk.
3. Kies **FastComments**.
4. Configureer de insluiting in het deep-linking dialoogvenster en klik op **Insert**.
5. Sla de pagina op.

Als de knop External Tool niet verschijnt in de Rich Text-editor, is Deep Linking uitgeschakeld voor deze tool op deze tenant. Zie de onderstaande gotchas.

**4. Zichtbaarheid en sectietoewijzingen**

Schoology regelt de beschikbaarheid van tools per sectie via Course Options:

1. Klik vanuit de cursus op **Course Options** > **External Tools**.
2. Voor elke geïnstalleerde LTI-app bepaal je of deze beschikbaar is voor alle secties in de cursus of voor specifieke secties.
3. Om FastComments te beperken tot bepaalde secties, haal je de vinkjes weg bij de secties die de tool niet mogen zien.
4. Sectie-niveau toegang bepaalt ook welke secties de vermelding **Add Materials** > **External Tool** voor FastComments zien.

**5. Wat studenten zien**

Studenten klikken op het FastComments-materiaal (of scrollen naar de inline embed) en belanden in de threaded discussie. SSO gaat automatisch via de Schoology LTI-launch onder hun Schoology-account.

Role mapping:

- Schoology `Administrator` -> FastComments admin
- Schoology `Instructor` -> FastComments moderator
- Schoology `Student` -> FastComments commenter

**6. Schoology-valkuilen**

- **Alleen Enterprise.** Persoonlijke en gratis Schoology-accounts kunnen geen LTI 1.3-tools installeren. Als jouw tenant op het gratis niveau zit, ontbreekt de optie **External Tools** in Course Options. Upgrade naar Schoology Enterprise om FastComments te gebruiken.
- **Deep Linking standaard uitgeschakeld door tenant.** Sommige Schoology-tenants beperken Deep Linking-plaatsing op organisatieniveau. In dat geval zien instructeurs alleen de flow **Add Materials** > **External Tool** en niet de knop External Tool in de Rich Text-editor. Om inline insluiting mogelijk te maken, gaat de System Administrator naar **System Settings** > **Integration** > **LTI 1.3** > **FastComments** en schakelt de plaatsing **Content Item / Deep Linking** in, en slaat op.
- **Sectie-override van toewijzing.** Als FastComments op ondernemingsniveau is toegewezen maar de instructeur kan het niet zien in **Add Materials**, dan is de sectie van de cursus uitgesloten in de organisatiebrede toewijzing. Vraag de System Administrator de sectie toe te voegen aan de FastComments-app-toewijzing.
- **Naam van materiaal versus identiteit van thread.** Het hernoemen van het materiaal in Schoology verplaatst de comment-thread niet. Threads zijn gekoppeld aan de LTI resource link ID, dus een hernoeming houdt dezelfde thread; het verwijderen en opnieuw aanmaken van het materiaal creëert een nieuwe, lege thread.
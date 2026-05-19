Deze handleiding behandelt het toevoegen van FastComments aan een Moodle 4.x-cursus nadat een sitebeheerder de tool heeft geregistreerd en heeft ingesteld om te tonen in de activity chooser. Als FastComments nog niet geregistreerd is, raadpleeg eerst de Moodle-registratiehandleiding.

#### Open de cursus in de bewerkingsmodus

1. Meld u aan bij Moodle als een Editing Teacher (of een hoger niveau) voor de cursus.
2. Open de cursus.
3. Schakel **Edit mode** in met de schakelaar in de rechterbovenhoek van de cursuskop.

Moodle 4.x heeft de verouderde dropdown "Add an activity or resource" uit 3.x vervangen door een full-screen activity chooser dialog. Moodle 4.5 behoudt dezelfde chooser maar voegt een starred/favorites-rij bovenaan toe, dus het vastpinnen van FastComments maakt het later sneller bereikbaar in andere secties.

#### Voeg de FastComments-activiteit toe

1. Scroll naar de cursussectie (topic of week) waar de discussie hoort.
2. Klik onderaan die sectie op **Add an activity or resource**.
3. Selecteer in het chooser-venster **FastComments**. Als u het niet ziet, ga naar de gotchas-sectie hieronder.

Het instellingenformulier voor de activiteit wordt geopend. De velden die van belang zijn:

- **Activity name** (vereist). Wordt weergegeven op de cursuspagina en in het cijferschema. Voorbeeld: `Week 3 Discussion`.
- **Activity description**. Optionele inleidende tekst die boven de commentaardraad wordt weergegeven.
- **Show description on course page**. Vink dit aan als u de beschrijving zichtbaar wilt hebben zonder in de activiteit te hoeven klikken.
- **Preconfigured tool**. Ingesteld op `FastComments` (automatisch geselecteerd wanneer gestart vanuit de chooser). Niet wijzigen.
- **Launch container**. Instellen op **New window**. Zie de gotchas-sectie voor waarom "Same window" in sommige Moodle-implementaties breekt.
- **Tool URL**, **Public key**, **Shared secret**, **Custom parameters**. Laat leeg. Dynamic Registration heeft deze op siteniveau afgehandeld.

Scroll naar beneden en klik op **Save and return to course** (of **Save and display** om de activiteit direct te openen).

De activiteit verschijnt als een rij in de sectie met het FastComments-pictogram. Studenten klikken op de rij om de commentaardraad te openen.

#### FastComments inline insluiten met de editor

Voor een draad binnen een Page, Book-hoofdstuk, Lesson of een andere resource die de Atto- of TinyMCE-editor gebruikt:

1. Open de resource in de bewerkingsmodus.
2. Plaats de cursor waar de draad moet verschijnen.
3. Klik in de editorwerkbalk op de **LTI** / **External tool**-knop. In Atto staat deze gelabeld als "Insert LTI Advantage content". In TinyMCE (standaard in Moodle 4.3+) staat het onder het **More**-menu als **External tools**.
4. Kies **FastComments** uit de lijst met tools.
5. FastComments opent een deep-linking picker. Bevestig de draadtitel en klik op **Embed**.
6. De editor voegt een LTI-placeholderblok in. Sla de resource op.

Elke ingebedde instantie is een aparte draad die is gekoppeld aan de deep-link content item ID, dus een Page met drie FastComments-embeds krijgt drie onafhankelijke draden.

#### Beperk toegang en groepsinstellingen

De standaardinstellingen voor Moodle-activiteiten zijn van toepassing op FastComments-activiteiten:

- **Common module settings** > **Group mode**. Het instellen hiervan op **Separate groups** of **Visible groups** splitst FastComments niet automatisch in per-groep draden. Moodle's groepsmodus filtert alleen het cijferschema en de ledenlijst. Om een aparte draad per groep te draaien, voeg per groep één FastComments-activiteit toe en gebruik **Restrict access** om elke activiteit te beperken.
- **Restrict access** > **Add restriction**. Ondersteunt de standaard Moodle-condities: **Date**, **Grade**, **Group**, **Grouping**, **User profile**, en geneste restrictiesets. Gebruik **Group** om een FastComments-activiteit aan één groep te koppelen.
- **Activity completion**. Stel in op **Students must view this activity to complete it** als u voltooiingstracking wilt. FastComments rapporteert momenteel geen voltooiingsgebeurtenis terug naar Moodle behalve de launch.

#### Roltoewijzing

FastComments leest de LTI `roles` claim die Moodle bij elke launch verzendt en mapt deze als volgt:

- Moodle **Manager** of **Site administrator** -> FastComments **admin**
- Moodle **Editing teacher** of **Non-editing teacher** -> FastComments **moderator**
- Moodle **Student** -> FastComments **commenter**
- Moodle **Guest** -> read-only

Admins kunnen elke opmerking verwijderen, gebruikers verbannen en draadinstellingen bewerken. Moderators kunnen opmerkingen verwijderen en goedkeuren binnen de draad waarin ze geopend zijn. Aangepaste Moodle-rollen erven de mapping van het archetype waaruit ze gekloond zijn.

#### Wat studenten zien

Studenten klikken op de FastComments-activiteit (of scrollen naar het ingebedde blok binnen een Page of Book). Moodle stuurt hun identiteit naar FastComments via de LTI-launch:

- Geen inlogscherm. FastComments logt hen in met het Moodle-account.
- Hun weergavenaam, e-mail en avatar komen van Moodle.
- De draad is geschaald naar (Moodle site, course, resource link ID), dus dezelfde activiteit die naar een andere cursus wordt gekopieerd, krijgt een nieuwe draad.
- Gelaagde reacties, stemmen en notificaties werken hetzelfde als in een zelfstandige FastComments-draad.

#### Beperk publieke toegang (aanbevolen)

Standaard zijn FastComments-commentaargegevens publiekelijk leesbaar. Iedereen die de URL of API-endpoint van een draad kan raden, kan de opmerkingen bekijken, zelfs buiten Moodle. Voor cursusdiscussies wilt u vrijwel zeker het bekijken beperken tot alleen ingeschreven studenten.

Open uw <a href="https://fastcomments.com/auth/my-account/customize-widget" target="_blank">widget customization page</a> en maak een regel aan met **Require SSO To View Comments** ingeschakeld, en zet vervolgens het beveiligingsniveau op **Secure SSO** zodat draden alleen via de ondertekende LTI-launch kunnen worden geladen.

Zie [Protecting Comment Threads With Single-Sign-On](/guide-customizations-and-configuration.html#sso-require-to-view-comments) voor de volledige walkthrough, inclusief hoe u de regel kunt beperken tot een enkele domein of pagina.

#### Moodle gotchas

**FastComments ontbreekt in de activity chooser.** De sitebeheerder heeft de tool geregistreerd maar heeft **Tool configuration usage** niet ingesteld op **Show in activity chooser and as a preconfigured tool**. Los dit op via **Site administration** > **Plugins** > **Activity modules** > **External tool** > **Manage tools** > tandwielpictogram op de FastComments-tegel.

**Launch faalt of toont een leeg frame wanneer ingesteld op "Same window".** Moodle's sessiecookies gebruiken standaard `SameSite=Lax`, en sommige browsers verwijderen ze bij de cross-site POST die LTI 1.3 gebruikt om terug te keren van FastComments. Stel **Launch container** in op **New window** voor de activiteit. Dit is een harde eis voor ingebedde FastComments binnen een Page of Book, aangezien het in de editor ingebedde launch-pad altijd een nieuw venster opent.

**De `iss` claim is de Moodle-site-URL, niet een tenant-ID.** FastComments gebruikt de Moodle-site-URL (de `wwwroot` config value) als de LTI-issuer. Als uw Moodle-instance naar een nieuw domein verhuist of u `wwwroot` wijzigt, blijven bestaande FastComments-draden gekoppeld aan de oude issuer en komen ze niet overeen met nieuwe launches. Registreer de tool opnieuw tegen de nieuwe URL en migreer draden via de FastComments-admin indien nodig.

**Activity backup en restore.** Het back-uppen van een cursus en het herstellen in een nieuwe cursus maakt nieuwe resource link IDs aan, dus de herstelde FastComments-activiteiten starten met lege draden. De originele cursus behoudt de originele draden. Dit is het bedoelde gedrag, geen bug.

**Moodle 4.5 TinyMCE-standaard.** Moodle 4.5 wordt geleverd met TinyMCE als standaardeditor voor nieuwe installaties. De External tool-knop bevindt zich onder het **More** (`...`) menu in plaats van de hoofdwerkbalk. Oudere sites die zijn geüpgraded vanaf 4.1 behouden Atto tenzij een beheerder de standaard heeft gewijzigd.

---
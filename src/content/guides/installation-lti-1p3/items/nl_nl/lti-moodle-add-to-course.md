Deze gids behandelt het toevoegen van FastComments aan een Moodle 4.x-cursus nadat een sitebeheerder de tool heeft geregistreerd en ingesteld om te tonen in de activiteitenkiezer. Als FastComments nog niet is geregistreerd, raadpleeg eerst de Moodle-registratiegids.

#### Open de cursus in bewerkmodus

1. Meld je aan bij Moodle als een bewerkende docent (of hoger) voor de cursus.
2. Open de cursus.
3. Zet **Bewerkmodus** aan met de schakelaar in de rechterbovenhoek van de cursuskop.

Moodle 4.x heeft de verouderde dropdown "Voeg een activiteit of bron toe" die in 3.x werd gebruikt, vervangen door een full-screen activiteitenkiezer-dialoog. Moodle 4.5 behoudt dezelfde kiezer maar voegt een rij met favorieten bovenaan toe, dus FastComments één keer vastpinnen maakt het later sneller bereikbaar in secties.

#### Voeg de FastComments-activiteit toe

1. Scroll naar de cursussectie (onderwerp of week) waar de discussie hoort.
2. Klik onderaan die sectie op **Voeg een activiteit of bron toe**.
3. Selecteer in de kiezer **FastComments**. Zie je het niet, ga dan naar de valkuilen-sectie hieronder.

Het formulier voor activiteitinstellingen opent. De velden die ertoe doen:

- **Activiteitsnaam** (vereist). Wordt weergegeven op de cursuspagina en in het cijferboek. Voorbeeld: `Week 3 Discussion`.
- **Activiteitsbeschrijving**. Optionele introductietekst die boven de commentaargespreksdraad wordt weergegeven.
- **Toon beschrijving op cursuspagina**. Vink dit aan als je de beschrijving zichtbaar wilt hebben zonder op de activiteit te klikken.
- **Vooraf geconfigureerde tool**. Instellen op `FastComments` (automatisch geselecteerd wanneer gestart vanuit de kiezer). Wijzig dit niet.
- **Startcontainer**. Instellen op **Nieuw venster**. Zie de valkuilen-sectie voor waarom "Zelfde venster" in sommige Moodle-implementaties problemen veroorzaakt.
- **Tool-URL**, **Publieke sleutel**, **Gedeeld geheim**, **Aangepaste parameters**. Laat leeg. Dynamische registratie regelt deze op siteniveau.

Scroll naar beneden en klik **Opslaan en terugkeren naar cursus** (of **Opslaan en weergeven** om de activiteit direct te openen).

De activiteit verschijnt als een rij in de sectie met het FastComments-pictogram. Studenten klikken op de rij om de commentaardraad te openen.

#### FastComments inline insluiten in de editor

Voor een draad binnen een Pagina, Boekhoofdstuk, Les of een andere bron die de Atto- of TinyMCE-editor gebruikt:

1. Open de bron in bewerkmodus.
2. Plaats de cursor waar de draad moet verschijnen.
3. Klik in de editorwerkbalk op de **LTI** / **Externe tool** knop. In Atto is deze gelabeld "Insert LTI Advantage content". In TinyMCE (standaard in Moodle 4.3+) staat het onder het **Meer**-menu als **Externe tools**.
4. Kies **FastComments** uit de lijst met tools.
5. FastComments opent een deep-linking picker. Bevestig de draadtitel en klik **Insluiten**.
6. De editor voegt een LTI-plaatshouderblok in. Sla de bron op.

Elke ingesloten instantie is een afzonderlijke draad, gekeyed op de deep-link content item ID, dus een Pagina met drie FastComments-insluitingen krijgt drie onafhankelijke draden.

#### Toegangsbeperking en groepsinstellingen

De standaardinstellingen voor activiteiten in Moodle gelden voor FastComments-activiteiten:

- **Algemene moduleinstellingen** > **Groepsmodus**. Dit instellen op **Aparte groepen** of **Zichtbare groepen** splitst FastComments niet automatisch in per-groepdraden. Moodles groepsmodus filtert alleen het cijferboek en de ledenlijst. Om per groep een aparte draad te draaien, voeg je voor elke groep één FastComments-activiteit toe en gebruik je **Toegangsbeperking** om elk ervan af te bakenen.
- **Toegangsbeperking** > **Voeg beperking toe**. Ondersteunt de standaard Moodle-condities: **Datum**, **Cijfer**, **Groep**, **Groepering**, **Gebruikersprofiel**, en geneste beperkingsets. Gebruik **Groep** om een FastComments-activiteit aan één groep te koppelen.
- **Voltooiing van activiteit**. Stel in op **Studenten moeten deze activiteit bekijken om hem als voltooid te markeren** als je voltooiingsregistratie wilt. FastComments rapporteert momenteel geen voltooiingsmelding terug naar Moodle behalve de lancering.

#### Rolverdeling

FastComments leest de LTI `roles` claim die Moodle bij elke start verzendt en mapt deze als volgt:

- Moodle **Manager** of **Site administrator** -> FastComments **admin**
- Moodle **Editing teacher** of **Non-editing teacher** -> FastComments **moderator**
- Moodle **Student** -> FastComments **commenter**
- Moodle **Guest** -> alleen-lezen

Beheerders kunnen elke reactie verwijderen, gebruikers verbannen en draadinstellingen bewerken. Moderators kunnen reacties verwijderen en goedkeuren binnen de draad waarin ze zijn gestart. Aangepaste Moodle-rollen erven de mapping van het archetype waarvan ze gekloond zijn.

#### Wat studenten zien

Studenten klikken de FastComments-activiteit (of scrollen naar het ingesloten blok binnen een Pagina of Boek). Moodle stuurt hun identiteit naar FastComments via de LTI-start:

- Geen inlogscherm. FastComments meldt hen aan met het Moodle-account.
- Hun weergavenaam, e-mail en avatar komen uit Moodle.
- De draad is gekoppeld aan (Moodle-site, cursus, resource link ID), dus dezelfde activiteit die in een andere cursus wordt gedupliceerd, krijgt een nieuwe draad.
- Geneste antwoorden, stemmen en meldingen werken hetzelfde als in een op zichzelf staande FastComments-draad.

#### Moodle-valkuilen

**FastComments ontbreekt in de activiteitenkiezer.** De sitebeheerder heeft de tool geregistreerd maar heeft **Gebruik van toolconfiguratie** niet ingesteld op **Weergeven in activiteitenkiezer en als vooraf geconfigureerde tool**. Los dit op via **Sitebeheer** > **Plug-ins** > **Activiteitsmodules** > **Externe tool** > **Tools beheren** > tandwielpictogram op de FastComments-tegel.

**Start mislukt of toont een leeg frame wanneer ingesteld op "Zelfde venster".** Moodles sessiecookies gebruiken standaard `SameSite=Lax`, en sommige browsers verwijderen deze bij de cross-site POST die LTI 1.3 gebruikt om terug te keren van FastComments. Stel **Startcontainer** in op **Nieuw venster** bij de activiteit. Dit is een harde vereiste voor ingesloten FastComments binnen een Pagina of Boek, omdat het door de editor ingesloten startpad altijd een nieuw venster opent.

**De `iss` claim is de Moodle-site-URL, niet een tenant-ID.** FastComments gebruikt de Moodle-site-URL (de `wwwroot` configwaarde) als de LTI-issuer. Als je Moodle-instance naar een nieuw domein verhuist of je `wwwroot` verandert, blijven bestaande FastComments-draden aan de oude issuer gekoppeld en komen ze niet overeen met nieuwe starts. Registreer de tool opnieuw tegen de nieuwe URL en migreer draden via de FastComments-beheerder indien nodig.

**Back-up en herstel van activiteiten.** Het maken van een back-up van een cursus en het herstellen in een nieuwe cursus creëert nieuwe resource link IDs, dus de herstelde FastComments-activiteiten beginnen met lege draden. De originele cursus behoudt de originele draden. Dit is bedoeld gedrag, geen bug.

**Moodle 4.5 TinyMCE-standaard.** Moodle 4.5 wordt geleverd met TinyMCE als standaardeditor voor nieuwe installaties. De knop Externe tool bevindt zich onder het **Meer** (`...`) menu in plaats van de hoofdbalk. Oudere sites die zijn geüpgraded vanaf 4.1 behouden Atto tenzij een beheerder de standaard heeft gewijzigd.
Når FastComments er registreret med platformen, tilføjer undervisere det til kursusindhold ved hjælp af platformens standard flows for eksterne værktøjer. Denne side dækker Sakai 23.x og Schoology Enterprise.

#### Lås offentlig adgang (anbefalet)

Som standard er FastComments-kommentardata offentligt læsbare på begge platforme. Enhver, der kan gætte en tråds URL eller API-endpoint, kan se dens kommentarer, selv uden for Sakai eller Schoology. For kursusdiskussioner vil du næsten altid begrænse visningen til kun indskrevne studerende.

Åbn din <a href="https://fastcomments.com/auth/my-account/customize-widget" target="_blank">widget-tilpasningsside</a> og opret en regel med **Require SSO To View Comments** aktiveret, og sæt derefter sikkerhedsniveauet til **Secure SSO**, så tråde kun kan indlæses gennem den signerede LTI-start.

Se [Protecting Comment Threads With Single-Sign-On](/guide-customizations-and-configuration.html#sso-require-to-view-comments) for den fulde gennemgang, inklusive hvordan du kan afgrænse reglen til et enkelt domæne eller en side.

#### Sakai

**1. Tilføj FastComments til et site**

Site-vedligeholderen aktiverer værktøjet på en per-site basis:

1. Åbn sitet og klik på **Site Info** i venstre navigation.
2. Klik **Manage Tools**.
3. Rul til listen **External Tools** og slå **FastComments** til.
4. Klik **Continue**, gennemgå værktøjslisten, og klik derefter **Finish**.

FastComments vises nu som et punkt i venstre navigation på sitet.

**2. Omarranger venstre navigation-posten**

Gå til **Site Info** > **Tool Order**. Træk **FastComments** til den ønskede position og klik **Save**. Du kan også omdøbe navigationsetiketten og skjule den for studerende fra denne skærm.

**3. Indlejring inline på en Lessons-side**

For at placere FastComments direkte inde i en Lessons-side i stedet for som et selvstændigt venstre-nav værktøj:

1. Åbn værktøjet **Lessons** i sitet.
2. Klik **Add Content** > **Add External Tool**.
3. Vælg **FastComments** fra listen.
4. Hvis FastComments annoncerede Deep Linking under registreringen, åbner Sakai værktøjets indholdsselector, så du kan vælge eller mærke tråden. Hvis Deep Linking ikke blev annonceret, indsætter Sakai et standard launch-link.
5. Gem Lessons-elementet.

Hver indlejret instans får sin egen tråd, afgrænset til det pågældende ressource-link.

**4. Rettighedsjusteringer for studerendes adgang**

Sakai styrer lanceringer af eksterne værktøjer gennem Realms. For at bekræfte, at studerende kan starte FastComments:

1. Log ind som Sakai-admin og åbn **Administration Workspace** > **Realms**.
2. Åbn den relevante realm (for eksempel `!site.template.course` eller den specifikke site-realm).
3. Bekræft, at rollen `access` har `lti.launch` aktiveret, og at rolle-tilladelserne i gruppen **external.tools** er givet.
4. Gem realmen.

For site-niveau overskrivninger kan vedligeholderen justere værktøjsynlighed per rolle fra **Site Info** > **Tool Order** ved at skjule eller vise FastComments per rolle.

**5. Hvad studerende ser**

Studerende klikker på FastComments-posten i venstre navigation (eller ruller til det indlejrede Lessons-blok) og lander direkte i den trådede kommentarside. SSO er automatisk: Sakai sender brugerens identitet i LTI-launchen, og FastComments logger dem ind under deres Sakai-konto.

Rollekortlægning:

- Sakai `Instructor` -> FastComments moderator
- Sakai `Admin` (admin i Administration Workspace) -> FastComments admin
- Sakai `Student` / `access` -> FastComments commenter

**6. Sakai-faldgruber**

- **Værktøjet er ikke synligt i Manage Tools.** Hvis FastComments ikke vises i External Tools-listen, skal Sakai-admin åbne værktøjsregistret (**Administration Workspace** > **External Tools** > **FastComments**) og sætte **Stealthed** til `false`. Stealthed værktøjer er skjult i den per-site Manage Tools-picker.
- **Lanceringer fejler i browsere med delt session.** Sakais portal CSRF-token er bundet til browsersessionen. Hvis en studerende er logget ind på to Sakai-sites i forskellige faner eller har en forældet session, returnerer launchen en 403. Løsning: luk andre Sakai-faner, log ud, log ind igen og start på ny. Administratorer kan også hæve `sakai.csrf.token.cache.ttl`, hvis dette sker på tværs af klyngen.
- **Indlejring i iframe.** Bekræft, at `lti.frameheight` i `sakai.properties` er stor nok (600 eller højere), så kommentarsamtalen ikke bliver afskåret inde i en Lessons-side.

#### Schoology

Schoology Enterprise har to installationsscenarier. Bekræft hvilket der gælder, før du tilføjer værktøjet til et kursus.

**1. To installationsscenarier**

- **(a) Enterprise-level installation.** Schoology System Administrator installerede FastComments på organisationsniveau og tildelte det til alle kurser eller til specifikke kursskabeloner. Undervisere springer installationen over og går direkte til "Add Materials".
- **(b) Underviser self-install.** Underviseren installerer værktøjet i et enkelt kursus fra **Course Options** > **External Tools** > **Install LTI Apps**. Self-install kræver, at System Administrator først har godkendt FastComments-app'en på organisationsniveau.

**2. Tilføj FastComments som kursusmateriale**

Inde i kurset:

1. Åbn kurset og gå til **Materials**.
2. Klik **Add Materials** > **Add File/Link/External Tool**.
3. Vælg **External Tool**.
4. Vælg **FastComments** fra listen over registrerede værktøjer.
5. Sæt et **Name** (dette er, hvad studerende ser i materialelisten) og en valgfri **Description**.
6. Lad **Enable Grading** (grade passback) være **OFF**. FastComments rapporterer ikke karakterer tilbage til Schoology, så aktivering af grade passback opretter en tom karakterkolonne.
7. Klik **Submit**.

Materialet vises nu i kursets materialeliste og åbner FastComments-tråden, når det klikkes.

**3. Inline-indlejring via Rich Text-editoren**

Hvis System Administrator aktiverede Deep Linking-placering for FastComments under registreringen, kan undervisere indlejre kommentartråden inde i enhver Rich Text-field (opgaveinstruktioner, sideindhold, diskussionsopslag):

1. Åbn Rich Text-editoren på målsiden.
2. Klik på ikonet **External Tool** (puslespilsbrik) i værktøjslinien.
3. Vælg **FastComments**.
4. Konfigurer indlejringen i deep-linking-dialogen og klik **Insert**.
5. Gem siden.

Hvis knappen External Tool ikke vises i Rich Text-editoren, er Deep Linking deaktiveret for dette værktøj på denne tenant. Se faldgruberne nedenfor.

**4. Synlighed og sektionstildelinger**

Schoology styrer værktøjets tilgængelighed per sektion gennem Course Options:

1. Fra kurset klik **Course Options** > **External Tools**.
2. For hver installeret LTI-app kontrollerer du, om den er tilgængelig for alle sektioner i kurset eller kun for specifikke sektioner.
3. For at begrænse FastComments til bestemte sektioner, fjern markeringen af de sektioner, som ikke skal se værktøjet.
4. Sektion-niveau adgang styrer også hvilke sektioner, der ser posten **Add Materials** > **External Tool** for FastComments.

**5. Hvad studerende ser**

Studerende klikker på FastComments-materialet (eller ruller til den indlejrede inline) og lander i den trådede diskussion. SSO er automatisk via Schoology LTI-launch under deres Schoology-konto.

Rollekortlægning:

- Schoology `Administrator` -> FastComments admin
- Schoology `Instructor` -> FastComments moderator
- Schoology `Student` -> FastComments commenter

**6. Schoology-faldgruber**

- **Kun Enterprise.** Personlige og gratis Schoology-konti kan ikke installere LTI 1.3-værktøjer. Hvis din tenant er på gratisniveau, er valgmuligheden **External Tools** fraværende i Course Options. Opgrader til Schoology Enterprise for at bruge FastComments.
- **Deep Linking deaktiveret som standard af tenant.** Nogle Schoology-tenants begrænser Deep Linking-placering på organisationsniveau. Når dette er tilfældet, ser undervisere kun flowet **Add Materials** > **External Tool** og ikke External Tool-knappen i Rich Text-editoren. For at aktivere inline-indlejring går System Administrator til **System Settings** > **Integration** > **LTI 1.3** > **FastComments** og aktiverer placeringen **Content Item / Deep Linking**, og gemmer derefter.
- **Sektionstildelings-override.** Hvis FastComments er tildelt på enterprise-niveau, men underviseren ikke kan se det i **Add Materials**, er kursets sektion udelukket i org-niveau tildelingen. Bed System Administrator om at tilføje sektionen til FastComments-app-tildelingen.
- **Materialenavn vs. trådidentitet.** Omdøbning af materialet i Schoology flytter ikke kommentartråden. Tråde er nøglede på LTI resource link ID, så en omdøbning bevarer den samme tråd; sletning og genoprettelse af materialet opretter en ny, tom tråd.
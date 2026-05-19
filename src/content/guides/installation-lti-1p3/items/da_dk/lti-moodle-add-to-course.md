Denne guide dækker tilføjelse af FastComments til et Moodle 4.x-kursus, efter at en site-administrator har registreret værktøjet og sat det til at vise i aktivitetsvælgeren. Hvis FastComments endnu ikke er registreret, se først Moodle-registreringsguiden.

#### Åbn kurset i redigeringstilstand

1. Log ind på Moodle som en Editing Teacher (eller højere) for kurset.
2. Åbn kurset.
3. Slå **Redigeringstilstand** til ved hjælp af kontakten i øverste højre hjørne af kursushovedet.

Moodle 4.x erstattede den ældre dropdown "Add an activity or resource", som 3.x brugte, med en fuldskærms aktivitetsvælgerdialog. Moodle 4.5 beholder den samme vælger, men tilføjer en række med stjernemarkerede/favoritter øverst, så det gør det hurtigere at få adgang til FastComments i senere afsnit, hvis du pinn­er det én gang.

#### Tilføj FastComments-aktiviteten

1. Rul til det kursusafsnit (emne eller uge), hvor diskussionen hører til.
2. Klik på **Tilføj en aktivitet eller ressource** nederst i det afsnit.
3. I vælgerdialogen skal du vælge **FastComments**. Hvis du ikke ser det, gå til afsnittet om faldgruber nedenfor.

Aktivitetsindstillingsformularen åbnes. Felterne der betyder noget:

- **Activity name** (påkrævet). Vises på kursussiden og i karakterbogen. Eksempel: `Week 3 Discussion`.
- **Activity description**. Valgfri introtekst, der gengives over kommentartråden.
- **Show description on course page**. Sæt kryds her, hvis du vil have beskrivelsen synlig uden at klikke ind i aktiviteten.
- **Preconfigured tool**. Sæt til `FastComments` (valgt automatisk, når den startes fra vælgeren). Ændr ikke.
- **Launch container**. Sæt til **Nyt vindue**. Se afsnittet om faldgruber for hvorfor "Samme vindue" kan bryde i nogle Moodle-udrulninger.
- **Tool URL**, **Public key**, **Shared secret**, **Custom parameters**. Lad stå tomme. Dynamic Registration håndterede disse på siteniveau.

Rul til bunden og klik **Gem og vend tilbage til kurset** (eller **Gem og vis** for at åbne aktiviteten med det samme).

Aktiviteten vises som en række i afsnittet med FastComments-ikonet. Studerende klikker rækken for at åbne kommentartråden.

#### Indlejre FastComments inline i editoren

For en tråd inde i en Page, Book-kapitel, Lesson eller en anden ressource, der bruger Atto- eller TinyMCE-editoren:

1. Åbn ressourcen i redigeringstilstand.
2. Placer markøren hvor tråden skal vises.
3. I editorens værktøjslinje skal du klikke på **LTI** / **External tool**-knappen. I Atto er den mærket "Insert LTI Advantage content". I TinyMCE (standard i Moodle 4.3+) ligger den under **Mere**-menuen som **External tools**.
4. Vælg **FastComments** fra værktøjslisten.
5. FastComments åbner en deep-linking-vælger. Bekræft trådens titel og klik **Indsæt**.
6. Editoren indsætter en LTI-pladsholderblok. Gem ressourcen.

Hver indlejret instans er en selvstændig tråd, der er nøglebundet til deep-link content item ID, så en Page med tre FastComments-indlejringer får tre uafhængige tråde.

#### Begræns adgang og gruppeindstillinger

De normale Moodle-aktivitetsindstillinger gælder for FastComments-aktiviteter:

- **Common module settings** > **Group mode**. At sætte dette til **Separate groups** eller **Visible groups** splitter ikke automatisk FastComments i tråde per gruppe. Moodles gruppetilstand filtrerer kun karakterbogen og medlemslisten. For at køre en separat tråd per gruppe, tilføj én FastComments-aktivitet per gruppe og brug **Restrict access** til at afgrænse hver enkelt.
- **Restrict access** > **Add restriction**. Understøtter de standard Moodle-betingelser: **Date**, **Grade**, **Group**, **Grouping**, **User profile**, og indlejrede begrænsningssæt. Brug **Group** til at låse en FastComments-aktivitet til en enkelt gruppe.
- **Activity completion**. Sæt til **Students must view this activity to complete it**, hvis du vil have fuldførelsessporing. FastComments rapporterer i øjeblikket ikke en fuldførelseshændelse tilbage til Moodle ud over launchen.

#### Rollekortlægning

FastComments læser LTI `roles`-påstanden, som Moodle sender ved hver launch, og kortlægger den som følger:

- Moodle **Manager** eller **Site administrator** -> FastComments **admin**
- Moodle **Editing teacher** eller **Non-editing teacher** -> FastComments **moderator**
- Moodle **Student** -> FastComments **commenter**
- Moodle **Guest** -> skrivebeskyttet

Admins kan slette enhver kommentar, udelukke brugere og redigere trådindstillinger. Moderators kan slette og godkende kommentarer inde i den tråd, de blev lanceret ind i. Tilpassede Moodle-roller arver kortlægningen af den archetype, de blev klonet fra.

#### Hvad studerende ser

Studerende klikker på FastComments-aktiviteten (eller ruller til den indlejrede blok inde i en Page eller Book). Moodle sender deres identitet til FastComments via LTI-launchen:

- Ingen login-skærm. FastComments logger dem ind ved hjælp af Moodle-kontoen.
- Deres visningsnavn, e-mail og avatar kommer fra Moodle.
- Tråden er scoped til (Moodle site, course, resource link ID), så den samme aktivitet duplikeret i et andet kursus får en frisk tråd.
- Trådede svar, stemmegivning og notifikationer fungerer på samme måde som i en selvstændig FastComments-tråd.

#### Begræns offentlig adgang (anbefalet)

Som standard er FastComments-kommentardata offentligt læsbare. Enhver, der kan gætte en tråds URL eller API-endpoint, kan se dens kommentarer, også uden for Moodle. For kursusdiskussioner vil du næsten altid ønske at begrænse visning til kun tilmeldte studerende.

Åbn din <a href="https://fastcomments.com/auth/my-account/customize-widget" target="_blank">widget-tilpasningsside</a> og opret en regel med **Require SSO To View Comments** aktiveret, og sæt derefter sikkerhedsniveauet til **Secure SSO**, så tråde kun kan indlæses gennem den signerede LTI-launch.

Se [Protecting Comment Threads With Single-Sign-On](/guide-customizations-and-configuration.html#sso-require-to-view-comments) for hele gennemgangen, inklusive hvordan du afgrænser reglen til et enkelt domæne eller side.

#### Moodle-faldgruber

**FastComments mangler i aktivitetsvælgeren.** Site-administratoren har registreret værktøjet, men satte ikke **Tool configuration usage** til **Show in activity chooser and as a preconfigured tool**. Ret dette under **Site administration** > **Plugins** > **Activity modules** > **External tool** > **Manage tools** > tandhjulsikonet på FastComments-flisen.

**Launch fejler eller viser en tom ramme, når den er sat til "Samme vindue".** Moodles sessionscookies bruger `SameSite=Lax` som standard, og nogle browsere fjerner dem ved det cross-site POST, som LTI 1.3 bruger til at vende tilbage fra FastComments. Sæt **Launch container** til **Nyt vindue** på aktiviteten. Dette er et hårdt krav for indlejrede FastComments inde i en Page eller Book, da den editor-indlejrede launch-sti altid åbner et nyt vindue.

**`iss`-påstanden er Moodle-site-URL'en, ikke et tenant-ID.** FastComments bruger Moodle-site-URL'en (konfigurationsværdien `wwwroot`) som LTI-issuer. Hvis din Moodle-instances flytter til et nyt domæne eller du ændrer `wwwroot`, forbliver eksisterende FastComments-tråde knyttet til den gamle issuer og matcher ikke nye launches. Genregistrer værktøjet imod den nye URL og migrer tråde gennem FastComments-admin om nødvendigt.

**Backup og gendannelse af aktiviteter.** At tage backup af et kursus og gendanne det i et nyt kursus skaber nye resource link IDs, så de gendannede FastComments-aktiviteter starter med tomme tråde. Det oprindelige kursus bevarer de oprindelige tråde. Dette er tilsigtet adfærd, ikke en fejl.

**Moodle 4.5 TinyMCE som standard.** Moodle 4.5 leveres med TinyMCE som standardeditor for nye installationer. External tool-knappens placering er under **Mere** (`...`) menuen snarere end i hovedværktøjslinjen. Ældre sites, der opgraderede fra 4.1, beholder Atto, medmindre en administrator har ændret standarden.
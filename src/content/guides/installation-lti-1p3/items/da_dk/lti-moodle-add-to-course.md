Denne vejledning dækker tilføjelse af FastComments til et Moodle 4.x-kursus efter at en site-administrator har registreret værktøjet og sat det til at vises i aktivitetsvælgeren. Hvis FastComments endnu ikke er registreret, se først Moodle-registreringsvejledningen.

#### Åbn kurset i redigeringstilstand

1. Log ind på Moodle som en Redigerende underviser (eller højere) for kurset.
2. Åbn kurset.
3. Skift **Redigeringstilstand** til ved hjælp af kontakten i øverste højre hjørne af kursushovedet.

Moodle 4.x erstattede den ældre "Tilføj en aktivitet eller ressource"-rullemenu, som 3.x brugte, med en fuldskærms aktivitetsvælger-dialog. Moodle 4.5 beholder den samme vælger, men tilføjer en stjerne/favoritrække øverst, så fastgørelse af FastComments én gang gør det hurtigere at finde i senere afsnit.

#### Tilføj FastComments-aktiviteten

1. Rul til det kursusafsnit (emne eller uge), hvor diskussionen hører hjemme.
2. Klik på **Tilføj en aktivitet eller ressource** nederst i det afsnit.
3. I vælgerdialogen skal du vælge **FastComments**. Hvis du ikke kan se det, hop til afsnittet om faldgruber nedenfor.

Aktivitetsindstillingsformularen åbnes. Felterne der betyder noget:

- **Aktivitetsnavn** (påkrævet). Vises på kursussiden og i karakterbogen. Eksempel: `Week 3 Discussion`.
- **Aktivitetsbeskrivelse**. Valgfri introduktionstekst, der gengives over kommentertråden.
- **Vis beskrivelse på kursusside**. Sæt kryds her hvis du vil have beskrivelsen synlig uden at klikke ind i aktiviteten.
- **Forhåndskonfigureret værktøj**. Sættes til `FastComments` (vælges automatisk når den startes fra vælgeren). Ændr ikke.
- **Launch container**. Sæt til **New window**. Se afsnittet om faldgruber for hvorfor "Same window" bryder i nogle Moodle-implementeringer.
- **Tool URL**, **Public key**, **Shared secret**, **Custom parameters**. Lad stå tomme. Dynamisk registrering håndterede disse på siteniveau.

Rul til bunden og klik på **Gem og returner til kurset** (eller **Gem og vis** for at åbne aktiviteten med det samme).

Aktiviteten vises som en række i afsnittet med FastComments-ikonet. Studerende klikker rækken for at åbne kommentertråden.

#### Indlejre FastComments inline med editoren

For en tråd inde i en Side, Bog-kapitlet, Lektion eller en hvilken som helst anden ressource, der bruger Atto eller TinyMCE-editoren:

1. Åbn ressourcen i redigeringstilstand.
2. Placer markøren hvor tråden skal vises.
3. I editorens værktøjslinje skal du klikke på knappen **LTI** / **External tool**. I Atto er den mærket "Insert LTI Advantage content". I TinyMCE (standard i Moodle 4.3+) ligger den under **Mere**-menuen som **External tools**.
4. Vælg **FastComments** fra værktøjslisten.
5. FastComments åbner en deep-linking-vælger. Bekræft trådens titel og klik **Embed**.
6. Editor indsætter en LTI-pladsholderblok. Gem ressourcen.

Hver indlejret instans er en separat tråd nøglebundet på deep-link content item ID, så en Side med tre FastComments-indlejringer får tre uafhængige tråde.

#### Begræns adgang og gruppetilgange

De almindelige Moodle-aktivitetindstillinger gælder for FastComments-aktiviteter:

- **Common module settings** > **Group mode**. At sætte dette til **Separate groups** eller **Visible groups** deler ikke automatisk FastComments i tråde pr. gruppe. Moodles gruppetilstand filtrerer kun karakterbogen og medlemslisten. For at køre en separat tråd per gruppe, tilføj én FastComments-aktivitet per gruppe og brug **Restrict access** til at afgrænse hver enkelt.
- **Restrict access** > **Add restriction**. Understøtter de standard Moodle-betingelser: **Date**, **Grade**, **Group**, **Grouping**, **User profile**, og indlejrede begrænsningssæt. Brug **Group** til at låse en FastComments-aktivitet til en enkelt gruppe.
- **Activity completion**. Sæt til **Students must view this activity to complete it** hvis du vil have fuldførelsessporing. FastComments rapporterer i øjeblikket ikke en fuldførelsesbegivenhed tilbage til Moodle ud over selve launch.

#### Rollekortlægning

FastComments læser LTI `roles`-claimet som Moodle sender ved hver launch og kortlægger det som følger:

- Moodle **Manager** eller **Site administrator** -> FastComments **admin**
- Moodle **Editing teacher** eller **Non-editing teacher** -> FastComments **moderator**
- Moodle **Student** -> FastComments **commenter**
- Moodle **Guest** -> skrivebeskyttet

Administratorer kan slette enhver kommentar, udelukke brugere og redigere trådindstillinger. Moderatorer kan slette og godkende kommentarer inde i den tråd, de er startet ind i. Brugerdefinerede Moodle-roller arver kortlægningen fra det arketype, de blev klonet fra.

#### Hvad studerende ser

Studerende klikker FastComments-aktiviteten (eller ruller til den indlejrede blok inde i en Side eller Bog). Moodle sender deres identitet til FastComments via LTI-launch:

- Ingen loginskærm. FastComments logger dem ind ved hjælp af Moodle-kontoen.
- Deres visningsnavn, e-mail og avatar kommer fra Moodle.
- Tråden er afgrænset til `(Moodle site, course, resource link ID)`, så samme aktivitet duplikeret i et andet kursus får en frisk tråd.
- Trådede svar, afstemninger og notifikationer fungerer på samme måde som en selvstændig FastComments-tråd.

#### Moodle-faldgruber

**FastComments mangler i aktivitetsvælgeren.** Site-administratoren har registreret værktøjet men satte ikke **Tool configuration usage** til **Show in activity chooser and as a preconfigured tool**. Ret dette under **Site administration** > **Plugins** > **Activity modules** > **External tool** > **Manage tools** > tandhjulsikonet på FastComments-flisen.

**Launch fejler eller viser en tom ramme når den er sat til "Same window".** Moodles sessioncookies bruger `SameSite=Lax` som standard, og nogle browsere fjerner dem på det cross-site POST som LTI 1.3 bruger til at returnere fra FastComments. Sæt **Launch container** til **New window** på aktiviteten. Dette er et hårdt krav for indlejret FastComments inde i en Side eller Bog, da editorens indlejrede launch-sti altid åbner et nyt vindue.

**`iss`-claimet er Moodle-site URL'en, ikke et tenant-id.** FastComments bruger Moodle-site URL'en (konfigurationsværdien `wwwroot`) som LTI-issuer. Hvis din Moodle-instans flytter til et nyt domæne eller du ændrer `wwwroot`, forbliver eksisterende FastComments-tråde knyttet til den gamle issuer og vil ikke matche nye launches. Genregistrer værktøjet mod den nye URL og migrer tråde gennem FastComments-admin hvis nødvendigt.

**Backup og gendannelse af aktivitet.** Backup af et kursus og gendannelse til et nyt kursus skaber nye resource link IDs, så de gendannede FastComments-aktiviteter starter med tomme tråde. Det oprindelige kursus bevarer de oprindelige tråde. Dette er tilsigtet adfærd, ikke en fejl.

**Moodle 4.5 TinyMCE som standard.** Moodle 4.5 leveres med TinyMCE som standardeditor for nye installationer. External tool-knappens placering er under **More** (`...`) menuen snarere end i hovedværktøjslinjen. Ældre sites, der er opgraderet fra 4.1, beholder Atto medmindre en administrator har skiftet standarden.
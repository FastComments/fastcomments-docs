Denne side beskriver, hvordan du tilføjer FastComments til et Brightspace-kursus, efter en administrator har registreret værktøjet og oprettet en deployment. Hvis værktøjet ikke er registreret endnu, se først D2L-registreringsguiden.

Brightspace leverer to indholdsskabelsesoplevelser: **Classic Content** og **New Content Experience** (også kaldet **Lessons**). Begge giver adgang til FastComments, men menustierne adskiller sig. Hvert afsnit nedenfor dækker begge, hvor de adskiller sig.

#### Find FastComments-værktøjet

FastComments-værktøjet vises to steder inde i en kursusindholdseditor:

1. Aktivitetspickeret, som nås fra en moduls/enheds **Add Existing**-knap (mærket **Add Existing Activities** i ældre Brightspace-versioner). FastComments vises direkte i pickeren i aktuelle Brightspace-builds; ældre versioner har det under en **External Learning Tools**-undermenu. Begge veje tilføjer FastComments som et selvstændigt emne.
2. **Insert Stuff**-dialogen inde i HTML-editoren, under **LTI Advantage**. Dette indlejrer FastComments inline i et HTML-emne via LTI deep linking-flowet.

Hvis FastComments ikke vises i nogen af pickerne, er deployment ikke aktiveret for den org-enhed, der indeholder kurset. Bed din Brightspace-administrator om at åbne **Admin Tools** > **Manage Extensibility** > **LTI Advantage** > FastComments tool > **View Deployments**, åbne deploymenten og tilføje kursets org-enhed (eller en overordnet org-enhed) under **Org Units**.

#### Tilføj FastComments som et emne i et modul

Classic Content:

1. Åbn kurset og klik **Content** i navigationslinjen.
2. Vælg det modul, der skal indeholde diskussionen (eller opret et via **Add a module**).
3. Klik **Add Existing** (ældre Brightspace: **Add Existing Activities** > **External Learning Tools**).
4. I pickeren klikker du **FastComments**. Brightspace opretter et emne i modulet og returnerer dig til indholdsoversigten.
5. Klik på det nye emne. Omdøb det til noget beskrivende som `FastComments Discussion` ved hjælp af inline-titleditoren.

New Content Experience (Lessons):

1. Åbn kurset og klik **Content**.
2. Åbn den enhed og lektion, der skal indeholde diskussionen.
3. Klik **Add** > **Existing Activity** og vælg **FastComments** (ældre Brightspace: placeret under **External Learning Tools**).
4. Aktiviteten tilføjes lektionen.
5. Klik på aktivitetens titel for at omdøbe den.

Første gang en bruger (instruktør eller studerende) åbner emnet, initialiserer FastComments tråden for det resource link. Tråden er bundet til resource link ID'et, så omdøbning eller flytning af emnet ændrer ikke, hvilken tråd der indlæses.

#### Embedd FastComments inline i et HTML-emne

Brug dette flow, når du ønsker, at kommentarer skal vises under en læsning, video eller andet indhold i samme emne-side frem for som et separat emne.

1. Åbn eller opret et HTML-emne i modulet/lektionen.
2. Klik **Edit HTML** for at åbne Brightspace HTML-editoren.
3. Placér markøren, hvor kommentartråden skal vises.
4. Klik på **Insert Stuff**-knappen (puslespilsikonet i editorens værktøjslinje).
5. I Insert Stuff-dialogen rul til **LTI Advantage** og klik **FastComments**.
6. FastComments åbner en deep linking-picker. Bekræft placeringen (standardindstillingerne fungerer til indholdsdiskussioner); klik **Insert** eller **Continue**.
7. Brightspace vender tilbage til HTML-editoren med et pladsholderblok, der repræsenterer LTI-launch. Klik **Save and Close** på emnet.

Når emnet indlæses, erstatter Brightspace pladsholderen med en iframe, der autolauncher FastComments via LTI. Studerende ser diskussionstråden inline.

Et enkelt HTML-emne kan indeholde flere deep-linked FastComments-embeds. Hvert embed får sin egen tråd, fordi hvert deep link producerer et distinkt resource link ID.

#### Modul-emne vs Inline Quicklink

Vælg tilgangen med **module topic**, når:

- Diskussionen er den primære aktivitet for det trin i modulet.
- Du vil have emnet til at fremgå i Brightspaces indholdsfortegnelse, færdiggørelsessporing og Class Progress.

Vælg **inline embed**-tilgangen, når:

- Kommentarer skal placeres under andet indhold på samme side.
- Du ikke ønsker en separat item i indholdsfortegnelsen, der kan spores for færdiggørelse.

#### Synlighed, kladde og frigivelsesbetingelser

Et nyt FastComments-emne er som standard synligt for studerende. For at skjule det, mens du sætter det op:

1. I indholdseditoren klikker du emnets titel (Classic) eller tre-punkts-menuen på aktiviteten (New Content Experience).
2. Sæt status til **Draft** (Classic) eller slå **Visibility** fra (New Content Experience).

Kladde-emner er usynlige for studerende. Instruktører og TA'er ser dem stadig med et "Draft"-mærke.

For at begrænse emnet til en bestemt gruppe eller sektion:

1. Åbn emnet.
2. Klik på emnets titelmenu > **Edit Properties In-place** (Classic) eller **Edit** > **Restrictions** (New Content Experience).
3. Under **Release Conditions**, klik **Create**.
4. Vælg **Group enrollment** eller **Section enrollment**, vælg gruppen/sektionen, og gem.

Frigivelsesbetingelser stabler oveni FastComments' egen rolle-mapping. Studerende, som ikke kan se emnet, får ikke en LTI-launch.

#### Hvad studerende ser ved første launch

Når en studerende klikker emnet (eller indlæser et HTML-emne med et embed):

1. Brightspace udfører LTI 1.3-launch i baggrunden.
2. FastComments modtager den studerendes navn, e-mail, avatar-URL og LMS-rolle og logger dem ind automatisk. Der er ingen FastComments-loginprompt.
3. Kommentartråden for det resource link renderer inden i Brightspace-iframe'en.

Rolle-mapping ved launch:

- Brightspace `Administrator` bliver en FastComments **admin** for tråden (fuld moderation, sletning, udelukkelse og konfigurationsadgang).
- Brightspace `Instructor` bliver en FastComments **moderator** (pin, skjul, slet, udeluk).
- Alle andre roller (`Learner`, `TeachingAssistant`, osv.) bliver almindelige kommentatorer.

Kommentarer tilskrives den studerendes Brightspace-konto. Hvis den studerende ændrer deres navn eller avatar i Brightspace, synkroniserer næste LTI-launch ændringen.

#### Iframe-højde og resize

FastComments udsender `org.imsglobal.lti.frameResize` postMessage ved hver trådrendering og ved indholdsændringer (ny kommentar, udvid svar). Brightspace lytter efter denne besked og justerer iframe-højden, så tråden ikke bliver afkortet eller viser en indre scrollbar.

Hvis iframe'en forbliver i en fast lav højde:

- Bekræft, at kurset indlæses over HTTPS. Brightspaces postMessage-listener afviser mixed-content frames.
- Bekræft, at ingen browserudvidelse blokerer postMessage-kanalen.
- For inline embeds i et HTML-emne må det omgivende HTML ikke pakke iframe'en ind i en container med fast højde. Fjern enhver inline `style="height: ..."` fra parent-elementet.

#### Brightspace-specifikke faldgruber

**Værktøj vises ikke i Add Existing-pickeren.** Deployment er ikke aktiveret for dette kursus' org-enhed. En administrator skal tilføje org-enheden (eller en overordnet) til deploymentens **Org Units**-liste. Værktøjsregistrering alene er ikke nok; deploymenten bestemmer, hvilke kurser der kan se værktøjet.

**`deployment_id` mismatch ved launch.** FastComments TOFU-pinner det første `deployment_id`, det ser for en registration. Hvis en administrator sletter den oprindelige deployment og opretter en ny, afvises launches fra den nye deployment med en deployment mismatch-fejl. Løsningen er at genregistrere FastComments (generere en ny registration URL og køre Dynamic Registration igen); den gamle konfigurationspost erstattes.

**Værktøjet lancerer men viser "Invalid LTI launch".** Kurset ligger i en anden tenant/org-struktur end deployment dækker, eller deploymenten blev deaktiveret efter registrering. Tjek igen **Admin Tools** > **Manage Extensibility** > **LTI Advantage** > FastComments > **Enabled**-toggle og deploymentens org-enhedsliste.

**Navne og roller mangler inde i FastComments.** Brightspace sender LTI-launches med Names and Role Provisioning Services (NRPS)-claims. Hvis et kursus er opgraderet fra et ældre LTI 1.1-link, mangler launch `name`- og `email`-claims. Genindsæt FastComments-emnet via **Add Existing** (migrer ikke det gamle link), så launch bruger LTI 1.3.

**Embed viser en loginskærm i stedet for auto-SSO.** HTML-emnet blev indsat som en almindelig <iframe> pegende direkte på FastComments i stedet for via **Insert Stuff** > **LTI Advantage**. Almindelige iframes springer LTI-launch over og lander brugere på den offentligt tilgængelige FastComments-side. Slet iframe'en og indsæt den igen via Insert Stuff-flowet.
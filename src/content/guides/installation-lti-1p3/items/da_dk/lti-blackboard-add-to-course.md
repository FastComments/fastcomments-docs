Når en administrator har registreret FastComments som et LTI 1.3 Advantage-værktøj og godkendt institutionens politikker, tilføjer undervisere det til kurser via Blackboard's standard placeringspunkter. De præcise trin adskiller sig mellem Ultra Course View og Original Course View, så begge er dækket nedenfor.

#### Ultra Course View

Ultra Course View er standarden i Blackboard Learn SaaS fra og med 2026.

1. Åbn kurset og gå til siden **Course Content**.
2. Hold musen over eller tryk dér, hvor du vil have kommentartråden placeret i oversigten, og klik på den lilla **+** (Add content)-knap.
3. Vælg **Content Market**. Content Market-panelet viser alle godkendte LTI-værktøjer og Building Block-placeringer for din institution.
4. Find **FastComments**-flisen og klik på den. Blackboard opretter et indholdselement på det sted, hvor du åbnede **+**-menuen.
5. Elementet lander i oversigten som en post "Visible to students" som standard for undervisere, der har **Hide from students** slået fra som deres personlige standard. Hvis din standard er **Hidden**, bliver elementet oprettet skjult, og du slår synlighedsvælgeren på elementets række til, når du er klar.
6. For at omdøbe elementet skal du klikke på titlen i oversigten og skrive en ny etiket. Den titel, som studerende ser i oversigten, er uafhængig af FastComments-trådens identifikator, så omdøbning er sikker når som helst.

Hvis du ikke kan se **Content Market** som en mulighed, har din institution skjult placeringsmuligheden. Du kan også få adgang til den samme vælger via **More tools** i den samme **+**-menu under gruppen **LTI Tools**.

#### Original Course View

Original Course View er stadig understøttet i Learn SaaS og forbliver den primære oplevelse for selv-hostede Learn 9.1-sider på Q4 2024 CU-udgivelseslinjen.

1. Åbn kurset og gå ind i et **Content Area** (for eksempel standardområdet **Information** eller **Content** i kursusmenuen).
2. Sæt **Edit Mode** til med skiftet øverst til højre på siden.
3. Klik på **Build Content** i handlingsbjælken.
4. Under undermenuen **Learning Tools** klikker du på **FastComments**. Undermenuen Learning Tools udfyldes fra LTI 1.3-værtsplaceringer efter en administrator har registreret værktøjet. Hvis du ikke kan se det, se afsnittet om faldgruber nedenfor.
5. På formularen **Create FastComments** skal du indstille:
   - **Name**: den etiket, studerende ser i indholdsområdet.
   - **Description**: valgfri tekst vist over den indlejrede tråd.
   - **Permit Users to View this Content**: Ja/Nej-tilgængelighedsskifter.
   - **Track Number of Views**: aktiver, hvis du vil have Blackboards per-element visningsstatistikker. FastComments kører sine egne analyser uafhængigt.
   - **Date and Time Restrictions**: valgfrie vinduer **Display After** / **Display Until**.
6. Indsend. Værktøjet vises som et klikbart element i indholdsområdet.

#### Indlejring inde i et element eller dokument

I begge kursusvisninger indlejrer undervisere FastComments inline inde i kroppen af et Item, Document eller ethvert rich-text-felt via Content Editorens LTI Advantage-knap.

Ultra Course View:

1. Opret eller rediger et **Document**.
2. Klik **Add content** inde i dokumentets indhold, dér hvor du vil have tråden vist.
3. I editorens værktøjslinje åbner du menuen **Insert content** og klikker **Content Market** (LTI Advantage / Deep Linking-indspringningspunktet).
4. Vælg **FastComments**. FastComments returnerer en deep-link-payload, og Blackboard indsætter en indlejret blok i dokumentets krop ved markørens position.
5. Gem dokumentet. Studerende ser tråden gengivet inline, når de scroller forbi den.

Original Course View:

1. Rediger et hvilket som helst element med en rich-text-krop.
2. I Content Editor-værktøjslinjen klikker du på plusikonet **Add Content** og vælger **Content Market** (mærket **Add Content from External Tool** i ældre Q4 2024 CUs).
3. Vælg **FastComments**. Editor indsætter en pladsholderblok, der refererer til den deep-linked ressource.
4. Indsend elementet.

Hver deep-link-indlejring opretter sin egen FastComments-tråd, så et element med to indlejrede FastComments-blokke har to uafhængige kommenterstrømme.

#### Synlighed, frigivelsesbetingelser og gruppebegrænsninger

FastComments-indholdselementer opfører sig som alle andre Blackboard-indholdselementer med hensyn til adgangskontrolreglerne, der lægges oven på dem.

- Ultra: klik på synlighedsvælgeren på rækken (**Visible to students**, **Hidden from students**, **Conditional availability**). Conditional availability understøtter dato-/tidsvinduer, performance-regler mod gradebook-elementer og medlemsregler mod kursusgrupper.
- Original: åbn elementets kontekstmenu og vælg **Adaptive Release** eller **Adaptive Release: Advanced** for at låse værktøjet bag dato, medlemskab, karakter eller gennemgangsstatus. Brug **Set Group Availability** på elementet for at begrænse til specifikke kursusgrupper.

FastComments respekterer hvad end Blackboard's adgangsregler bestemmer. Hvis Blackboard skjuler elementet for en studerende, sker LTI-launch ikke for den studerende, og de vises ikke i moderatorvisningen.

#### Gradebook-adfærd

FastComments rapporterer ikke karakterer tilbage via LTI Advantage Assignment and Grade Services. Der oprettes ikke automatisk en karakterkolonne for FastComments-indholdselementer.

Hvis jeres Blackboard-tenant er konfigureret til automatisk at oprette en gradebook-kolonne for hvert nyt indholdselement uanset bedømmelsesmetadata, vises der alligevel en tom kolonne. For at skjule den:

- Ultra: åbn **Gradebook**, klik på kolonneoverskriften, vælg **Edit**, og slå **Show to students** og **Include in calculations** fra. Eller brug **Delete**, hvis din institution tillader sletning af kolonner for ikke-bedømte elementer.
- Original: åbn **Grade Center**, klik på kolonnens chevron, vælg **Hide from Users (on/off)**, og eventuelt **Hide from Instructor View** under **Column Organization**.

#### Hvad studerende ser

Når en studerende åbner FastComments-elementet eller scroller til en indlejret blok:

1. Blackboard lancerer LTI 1.3-beskeden til FastComments. Den studerende logges ind via SSO med deres Blackboard-identitet (navn, e-mail, avatar, rolle) uden at se en loginformular.
2. Kommentartråden gengives i iframe'en. Trådstruktur, svar, nævnelser og reaktioner er alle tilgængelige baseret på kommentar-widgetens indstillinger konfigureret i FastComments.
3. Deres kommentarer tilskrives deres Blackboard-konto. Hvis den studerende senere redigerer deres navn eller billede i Blackboard, opdaterer næste launch FastComments-profilen.

Rollekortlægning fra Blackboard til FastComments:

- **System Administrator** og **Course Builder** kortlægges til FastComments **admin**.
- **Instructor** og **Teaching Assistant** kortlægges til FastComments **moderator**.
- **Student**, **Guest**, og **Observer** kortlægges til FastComments **commenter**.

Moderatorer ser moderationskontroller (pin, hide, ban, delete) inline på hver kommentar i tråden.

#### Trådafgrænsning

FastComments afgrænser hver tråd efter **(Blackboard host, course ID, resource link ID)**. To FastComments-elementer i samme kursus producerer to tråde. Samme element kopieret på tværs af to kursusskaller (for eksempel via course copy) producerer to tråde, fordi Blackboard udsteder en ny resource link ID under kopieringen. For at bevare en delt tråd på tværs af kursuskopier skal du bruge Deep Linking med en eksplicit thread URN konfigureret i FastComments før du lancerer kopien.

#### Blackboard-specifikke faldgruber

**FastComments-flisen mangler i Build Content-menuen (Original) eller Content Market (Ultra).** Administratoren har godkendt værktøjet, men efterladt en institutionspolitik, der blokerer den relevante placering. Gå til **Administrator Panel** > **Integrations** > **LTI Tool Providers**, rediger FastComments-posten, og bekræft, at både **Course Content Tool** (Original) og **Course Content Tool - allow students** / **Deep Linking content tool** (Ultra) placeringer er aktiveret. Gem og opdater kursussiden.

**"Tool not configured for this context" eller "Tool is not deployed" fejl ved lancering.** Deploymentscopet, der blev registreret under dynamisk registrering, matcher ikke institutionskonteksten som kurset tilhører. I Blackboards værtsindgang for værktøjet, bekræft at **Deployment ID** matcher det, FastComments viser på sin LTI 1.3 Configuration-side for denne tenant. Hvis de adskiller sig, slet placeringen og kør dynamisk registrering igen fra en frisk registrerings-URL.

**Iframe-højden virker fast eller indhold bliver afskåret.** Nogle Blackboard-tenants leveres med en streng Content Security Policy, som blokerer den standard LTI iframe-resize postMessage. FastComments udsender både den Canvas-lignende `lti.frameResize`-besked og IMS-specifikationen `org.imsglobal.lti.frameResize`-beskeden for at maksimere kompatibiliteten, men et tenant-niveau CSP-override blokerer for lytter i parent. Bed din administrator om at bekræfte, at `*.fastcomments.com` er på LTI-værktøjets allowlist, og at ingen brugerdefineret CSP-header fjerner postMessage-beskeder. Resize virker derefter uden yderligere konfiguration.

**Course copy duplikerer tråde.** Blackboard course copy udsteder nye resource link IDs for LTI-placeringer, så kopierede kurser starter med tomme tråde. Dette er forventet. Hvis du har brug for, at det kopierede kursus arver den originale tråd, opsæt Deep Linking med en eksplicit thread URN før kopiering, eller kontakt FastComments-support for at omkortlægge tråd-ID'er i bulk.

**Studerende ser en generisk Blackboard-fejl ved lancering.** Årsagen er en manglende eller forældet `email`-claim. Bekræft institutionens politik for FastComments har **Role**, **Name**, og **Email Address** aktiveret under **User Fields to Send**. Gem, og lancer derefter igen i en frisk browsersession.
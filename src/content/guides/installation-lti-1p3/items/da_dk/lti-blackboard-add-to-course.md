Once an administrator has registered FastComments as an LTI 1.3 Advantage tool and approved the institution policies, instructors add it to courses through the standard Blackboard placement points. The exact steps differ between Ultra Course View and Original Course View, so both are covered below.

#### Ultra Course View

Ultra Course View er standarden i Blackboard Learn SaaS fra og med 2026.

1. Åbn kurset og gå til **Course Content**-siden.
2. Hold musen over eller tryk der, hvor du vil indsætte tråden i oversigten, og klik på den lilla **+** (Add content)-knap.
3. Vælg **Content Market**. Content Market-panelet viser alle godkendte LTI-værktøjer og Building Block-placeringer for din institution.
4. Find **FastComments**-flisen og klik på den. Blackboard opretter et indholdselement på den position, hvor du åbnede **+**-menuen.
5. Elementet lander i oversigten som en post "Visible to students" som standard for undervisere, der har **Hide from students** slået fra som deres personlige standard. Hvis din standard er **Hidden**, oprettes elementet som skjult, og du tænder synlighedsselektoren på elementets række, når du er klar.
6. For at omdøbe elementet skal du klikke på titlen i oversigten og indtaste en ny etiket. Den titel, som studerende ser i oversigten, er uafhængig af FastComments-trådens identifikator, så det er sikkert at omdøbe når som helst.

Hvis du ikke kan se **Content Market** som en mulighed, har din institution skjult placeringen. Du kan også få adgang til samme vælger gennem **More tools** i den samme **+**-menu under **LTI Tools**-gruppen.

#### Original Course View

Original Course View understøttes stadig i Learn SaaS og forbliver den primære oplevelse for self-hosted Learn 9.1-sider på Q4 2024 CU-udgivelseslinjen.

1. Åbn kurset og gå ind i et **Content Area** (for eksempel standardområdet **Information** eller **Content** i kursusmenuen).
2. Slå **Edit Mode** til med toggle-knappen øverst til højre på siden.
3. Klik på **Build Content** i handlingslinjen.
4. Under undermenuen **Learning Tools** skal du klikke på **FastComments**. Undermenuen Learning Tools udfyldes ud fra LTI 1.3-værtsplaceringer efter en administrator har registreret værktøjet. Hvis du ikke kan se det, se afsnittet om problemer nedenfor.
5. På formularen **Create FastComments** skal du sætte:
   - **Name**: etiketten, som studerende ser i indholdsområdet.
   - **Description**: valgfri tekst vist over den indlejrede tråd.
   - **Permit Users to View this Content**: Ja/Nej-tilgængelighedskontakt.
   - **Track Number of Views**: aktiver, hvis du vil bruge Blackboards statistik for visninger per element. FastComments kører sine egne analyser uafhængigt.
   - **Date and Time Restrictions**: valgfrie **Display After** / **Display Until**-vinduer.
6. Indsend. Værktøjet vises som et klikbart element i indholdsområdet.

#### Embedding Inside an Item or Document

I begge course views indlejrer undervisere FastComments inline inde i brødteksten af et Item, Document eller ethvert rich-text-felt via Content Editorens LTI Advantage-knap.

Ultra Course View:

1. Opret eller rediger et **Document**.
2. Klik **Add content** inde i dokumentets brødtekst, hvor du ønsker tråden skal vises.
3. I editorens værktøjslinje skal du åbne menuen **Insert content** og klikke på **Content Market** (LTI Advantage / Deep Linking-indgangspunktet).
4. Vælg **FastComments**. FastComments returnerer en deep-link payload, og Blackboard indsætter en indlejret blok i dokumentets brødtekst på markørens position.
5. Gem dokumentet. Studerende ser tråden gengivet inline, når de scroller forbi den.

Original Course View:

1. Rediger ethvert element med en rich-text-brødtekst.
2. I Content Editor-værktøjslinjen skal du klikke på plusikonet **Add Content** og vælge **Content Market** (mærket **Add Content from External Tool** i ældre Q4 2024 CUs).
3. Vælg **FastComments**. Editor indsætter en pladsholderblok, der refererer til den deep-linked ressource.
4. Indsend elementet.

Hver deep-link-embed opretter sin egen FastComments-tråd, så et Item med to indlejrede FastComments-blokke har to uafhængige kommenterstrømme.

#### Visibility, Release Conditions, and Group Restrictions

FastComments-indholdselementer opfører sig som ethvert andet Blackboard-indholdselement med hensyn til adgangskontrolreglerne, der lægges ovenpå dem.

- Ultra: klik på synlighedsselektoren på rækken (**Visible to students**, **Hidden from students**, **Conditional availability**). Conditional availability understøtter dato-/tidsvinduer, performance-regler mod karakterbogselementer og medlemsregler mod kursusgrupper.
- Original: åbn elementets kontekstmenu og vælg **Adaptive Release** eller **Adaptive Release: Advanced** for at styre værktøjet efter dato, medlemskab, karakter eller gennemgangsstatus. Brug **Set Group Availability** på elementet for at begrænse til specifikke kursusgrupper.

FastComments respekterer hvad end Blackboard-gatet beslutter. Hvis Blackboard skjuler elementet for en studerende, sker LTI-launch ikke for den studerende, og de vises ikke i moderatorvinduet.

#### Gradebook Behavior

FastComments rapporterer ikke karakterer tilbage via LTI Advantage Assignment and Grade Services. Der oprettes ikke automatisk en karakterkolonne for FastComments-indholdselementer.

Hvis din Blackboard-tenant er konfigureret til automatisk at oprette en karakterbogskolonne for hvert nyt indholdselement uanset bedømmelsesmetadata, vises der alligevel en tom kolonne. For at skjule den:

- Ultra: åbne **Gradebook**, klik på kolonneoverskriften, vælg **Edit**, og slå **Show to students** samt **Include in calculations** fra. Eller brug **Delete**, hvis din institution tillader sletning af kolonner for ikke-bedømte elementer.
- Original: åbne **Grade Center**, klik på kolonnens chevron, vælg **Hide from Users (on/off)**, og eventuelt **Hide from Instructor View** under **Column Organization**.

#### What Students See

Når en studerende åbner FastComments-elementet eller scroller til en indlejret blok:

1. Blackboard lancerer LTI 1.3-beskeden til FastComments. Den studerende logges ind via SSO med deres Blackboard-identity (navn, e-mail, avatar, rolle) uden at se en loginformular.
2. Kommentartråden gengives i iframe'en. Trådning, svar, mentions og reaktioner er alle tilgængelige baseret på comment widget-indstillingerne konfigureret i FastComments.
3. Deres kommentarer tilskrives deres Blackboard-konto. Hvis den studerende senere redigerer deres navn eller billede i Blackboard, opdateres FastComments-profilen ved næste launch.

Role mapping fra Blackboard til FastComments:

- **System Administrator** og **Course Builder** kortlægges til FastComments **admin**.
- **Instructor** og **Teaching Assistant** kortlægges til FastComments **moderator**.
- **Student**, **Guest**, og **Observer** kortlægges til FastComments **commenter**.

Moderatorer ser moderationskontroller (pin, hide, ban, delete) inline på hver kommentar i tråden.

#### Lock Down Public Access (Recommended)

Som standard er FastComments-kommentardata offentligt læsbare. Enhver, der kan gætte en tråds URL eller API-endpoint, kan se dens kommentarer, også uden for Blackboard. For kursusdiskussioner vil du næsten helt sikkert begrænse visning til kun tilmeldte studerende.

Åbn din <a href="https://fastcomments.com/auth/my-account/customize-widget" target="_blank">widget customization page</a> og opret en regel med **Require SSO To View Comments** aktiveret, og sæt derefter sikkerhedsniveauet til **Secure SSO**, så tråde kun kan indlæses gennem den signerede LTI-launch.

Se [Protecting Comment Threads With Single-Sign-On](/guide-customizations-and-configuration.html#sso-require-to-view-comments) for den fulde gennemgang, inklusive hvordan du afgrænser reglen til et enkelt domæne eller en enkelt side.

#### Thread Scoping

FastComments afgrænser hver tråd efter **(Blackboard host, course ID, resource link ID)**. To FastComments-elementer i samme kursus producerer to tråde. Samme element kopieret på tværs af to kursusskaller (for eksempel gennem course copy) producerer to tråde, fordi Blackboard udsteder et nyt resource link ID under kopieringen. For at bevare en delt tråd på tværs af kursuskopier skal du bruge Deep Linking med en eksplicit thread URN konfigureret i FastComments før du lancerer kopien.

#### Blackboard-Specific Gotchas

**FastComments tile missing from the Build Content menu (Original) or Content Market (Ultra).** Administratoren godkendte værktøjet, men lod en institutionel politik blokere den relevante placering. Gå til **Administrator Panel** > **Integrations** > **LTI Tool Providers**, rediger FastComments-posten, og bekræft at både **Course Content Tool** (Original) og **Course Content Tool - allow students** / **Deep Linking content tool** (Ultra) placeringer er aktiveret. Gem og opdater kursussiden.

**"Tool not configured for this context" or "Tool is not deployed" error on launch.** Deploymentscope registreret under dynamisk registrering matcher ikke den institutionskontekst, som kurset tilhører. I Blackboards værktøjsudbyderpost skal du bekræfte, at **Deployment ID** matcher det, FastComments viser på sin LTI 1.3 Configuration-side for denne tenant. Hvis de adskiller sig, slet placeringen og kør dynamisk registrering igen fra en frisk registrerings-URL (<a href="https://fastcomments.com/auth/my-account/lti-config" target="_blank">get it here</a>).

**Iframe height looks fixed or content gets cut off.** Nogle Blackboard-tenants leveres med en streng Content Security Policy, der blokerer den standard LTI iframe-resize postMessage. FastComments udsender både Canvas-stilen `lti.frameResize`-beskeden og IMS-spec-formen `org.imsglobal.lti.frameResize`-beskeden for at maksimere kompatibilitet, men en tenant-niveau CSP-override blokerer parent-listeneren. Bed din administrator om at bekræfte, at `*.fastcomments.com` er på LTI-værktøjets allowlist, og at ingen brugerdefinerede CSP-headere fjerner postMessage-hændelser. Resize virker derefter uden yderligere konfiguration.

**Course copy duplicates threads.** Blackboard course copy udsteder nye resource link IDs for LTI-placeringer, så kopierede kurser starter med tomme tråde. Dette er forventet. Hvis du har brug for, at det kopierede kursus arver den oprindelige tråd, skal du sætte Deep Linking op med en eksplicit thread URN før kopiering, eller kontakte FastComments-support for at remappe tråd-ID'er i bulk.

**Student sees a generic Blackboard error on launch.** Årsagen er en manglende eller forældet `email`-claim. Bekræft, at institutionens politik for FastComments har **Role**, **Name**, og **Email Address** aktiveret under **User Fields to Send**. Gem, og lancer igen i en frisk browsersession.
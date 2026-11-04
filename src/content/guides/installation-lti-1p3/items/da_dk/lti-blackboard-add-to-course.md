Når en administrator har registreret FastComments som et LTI 1.3 Advantage-værktøj og godkendt institutionens politikker, tilføjer undervisere det til kurser via Blackboards standardplaceringspunkter. De nøjagtige trin adskiller sig mellem Ultra Course View og Original Course View, så begge er dækket nedenfor.

#### Ultra Course View

Ultra Course View er standard i Blackboard Learn SaaS fra 2026.

1. Åbn kurset og gå til siden **Course Content**.
2. Hold musen over eller tryk der, hvor du ønsker, at kommentartråden skal placeres i strukturen, og klik på den lilla **+**-knap (Add content).
3. Vælg **Content Market**. Content Market-panelet viser alle godkendte LTI-værktøjer og Building Block-placeringer for din institution.
4. Find **FastComments**-flisen og klik på den. Blackboard opretter et indholdselement på den position, hvor du åbnede **+**-menuen.
5. Elementet lander i strukturen som en "Visible to students"-post som standard for undervisere, der har **Hide from students** slået fra som deres personlige standard. Hvis din standard er **Hidden**, oprettes elementet skjult, og du vender synlighedsvælgeren på elementrækken, når du er klar.
6. For at omdøbe elementet klikker du på titlen i strukturen og skriver en ny etiket. Titlen, som studerende ser i strukturen, er uafhængig af FastComments-trådidentifikatoren, så omdøbning er sikkert til enhver tid.

Hvis du ikke kan se **Content Market** som en mulighed, har din institution skjult placeringen. Du når også den samme vælger via **More tools** i samme **+**-menu under gruppen **LTI Tools**.

#### Original Course View

Original Course View understøttes stadig i Learn SaaS og forbliver den primære oplevelse for selv-hostede Learn 9.1-sider på Q4 2024 CU-udgivelseslinjen.

1. Åbn kurset og gå ind i et **Content Area** (for eksempel standardområdet **Information** eller **Content** i kursusmenuen).
2. Slå **Edit Mode** til med kontakten i øverste højre hjørne af siden.
3. Klik på **Build Content** i handlingslinjen.
4. Under undermenuen **Learning Tools** klikker du på **FastComments**. Undermenuen Learning Tools udfyldes fra LTI 1.3-værktøjsplaceringer, efter en administrator har registreret værktøjet. Hvis du ikke kan se det, se afsnittet om faldgruber nedenfor.
5. På formularen **Create FastComments** indstiller du:
   - **Name**: etiketten, som studerende ser i indholdsområdet.
   - **Description**: valgfri tekst, der vises over den indlejrede tråd.
   - **Permit Users to View this Content**: Ja/Nej-tilgængelighedskontakt.
   - **Track Number of Views**: aktivér, hvis du ønsker Blackboards visningsstatistik pr. element. FastComments kører sin egen analyse uafhængigt.
   - **Date and Time Restrictions**: valgfrie **Display After** / **Display Until**-vinduer.
6. Indsend. Værktøjet vises som et klikbart element i indholdsområdet.

#### Indlejring i et element eller dokument

I begge kursusvisninger indlejrer undervisere FastComments inline i brødteksten af et Item, Document eller ethvert rich-text-felt via Content Editorens LTI Advantage-knap.

Ultra Course View:

1. Opret eller rediger et **Document**.
2. Klik på **Add content** i dokumentets brødtekst, der hvor du ønsker, at tråden skal vises.
3. I editorværktøjslinjen åbner du menuen **Insert content** og klikker på **Content Market** (LTI Advantage / Deep Linking-indgangspunktet).
4. Vælg **FastComments**. FastComments returnerer en deep-link-payload, og Blackboard indsætter en indlejret blok i dokumentets brødtekst ved markørens position.
5. Gem dokumentet. Studerende ser tråden renderet inline, når de scroller forbi den.

Original Course View:

1. Rediger ethvert element med en rich-text-brødtekst.
2. I Content Editor-værktøjslinjen klikker du på plus-ikonet **Add Content** og vælger **Content Market** (mærket **Add Content from External Tool** i ældre Q4 2024 CU'er).
3. Vælg **FastComments**. Editoren indsætter en pladsholderblok, der refererer til den deep-linkede ressource.
4. Indsend elementet.

Hvert deep-link-embed producerer sin egen FastComments-tråd, så et Item med to indlejrede FastComments-blokke har to uafhængige kommentarstrømme.

#### Synlighed, frigivelsesbetingelser og gruppebegrænsninger

FastComments-indholdselementer opfører sig som ethvert andet Blackboard-indholdselement med hensyn til de adgangskontrolregler, der lægges ovenpå dem.

- Ultra: klik på synlighedsvælgeren på rækken (**Visible to students**, **Hidden from students**, **Conditional availability**). Conditional availability understøtter dato/tidsvinduer, præstationsregler mod karakterbogselementer og medlemsregler mod kursusgrupper.
- Original: åbn elementets kontekstmenu og vælg **Adaptive Release** eller **Adaptive Release: Advanced** for at begrænse værktøjet efter dato, medlemskab, karakter eller gennemsynsstatus. Brug **Set Group Availability** på elementet for at begrænse til specifikke kursusgrupper.

FastComments respekterer, hvad end Blackboards port beslutter. Hvis Blackboard skjuler elementet for en studerende, sker LTI-launchen aldrig for den studerende, og de vises ikke i moderatorvisningen.

#### Karakterbogsadfærd

FastComments rapporterer ikke karakterer tilbage via LTI Advantage Assignment and Grade Services. Der oprettes ikke automatisk en karakterkolonne for FastComments-indholdselementer.

Hvis din Blackboard-tenant er konfigureret til automatisk at oprette en karakterbogskolonne for hvert nyt indholdselement uanset vurderingsmetadata, vises en tom kolonne alligevel. For at skjule den:

- Ultra: åbn **Gradebook**, klik på kolonneoverskriften, vælg **Edit**, og slå **Show to students** samt **Include in calculations** fra. Eller brug **Delete**, hvis din institution tillader sletning af kolonner for ikke-vurderede elementer.
- Original: åbn **Grade Center**, klik på kolonnens chevron, vælg **Hide from Users (on/off)** og eventuelt **Hide from Instructor View** under **Column Organization**.

#### Hvad studerende ser

Når en studerende åbner FastComments-elementet eller scroller til en indlejret blok:

1. Blackboard lancerer LTI 1.3-beskeden til FastComments. Den studerende logges ind via SSO med deres Blackboard-identitet (navn, e-mail, avatar, rolle) uden at se en loginformular.
2. Kommentartråden renderes i iframen. Trådning, svar, omtaler og reaktioner er alle tilgængelige baseret på de kommentarwidget-indstillinger, der er konfigureret i FastComments.
3. Deres kommentarer tilskrives deres Blackboard-konto. Hvis den studerende senere redigerer deres navn eller foto i Blackboard, opdaterer den næste launch FastComments-profilen.

Rollemapping fra Blackboard til FastComments:

- **System Administrator** og **Course Builder** mappes til FastComments **admin**.
- **Instructor** og **Teaching Assistant** mappes til FastComments **moderator**.
- **Student**, **Guest** og **Observer** mappes til FastComments **commenter**.

Moderatorer ser moderationskontroller (fastgør, skjul, udeluk, slet) inline på hver kommentar i tråden.

#### Lås offentlig adgang (anbefalet)

Som standard er FastComments-kommentardata offentligt læsbare. Enhver, der kan gætte en tråds URL eller API-endpoint, kan se dens kommentarer, selv uden for Blackboard. For kursusdiskussioner vil du næsten helt sikkert begrænse visning til kun tilmeldte studerende.

Åbn din <a href="https://fastcomments.com/auth/my-account/customize-widget" target="_blank">widget-tilpasningsside</a> og opret en regel med **Require SSO To View Comments** aktiveret, og indstil derefter sikkerhedsniveauet til **Secure SSO**, så tråde kun kan indlæses via den signerede LTI-launch.

Se [Protecting Comment Threads With Single-Sign-On](/guide-customizations-and-configuration.html#sso-require-to-view-comments) for den fulde gennemgang, herunder hvordan du afgrænser reglen til et enkelt domæne eller en enkelt side.

#### Trådafgrænsning

FastComments afgrænser hver tråd efter **(Blackboard-vært, kursus-ID, resource link-ID)**. To FastComments-elementer i samme kursus producerer to tråde. Det samme element kopieret på tværs af to kursusskaller (for eksempel via kursuskopiering) producerer to tråde, fordi Blackboard udsteder et nyt resource link-ID under kopieringen. For at bevare en delt tråd på tværs af kursuskopier skal du bruge Deep Linking med en eksplicit tråd-URN konfigureret i FastComments, før du lancerer kopieringen.

#### Blackboard-specifikke faldgruber

**FastComments-flisen mangler i Build Content-menuen (Original) eller Content Market (Ultra).** Administratoren godkendte værktøjet, men efterlod en institutionspolitik, der blokerer den relevante placering. Gå til **Administrator Panel** > **Integrations** > **LTI Tool Providers**, rediger FastComments-posten, og bekræft, at både **Course Content Tool** (Original) og **Course Content Tool - allow students** / **Deep Linking content tool** (Ultra)-placeringer er aktiveret. Gem og opdater kurssiden.

**"Tool not configured for this context" eller "Tool is not deployed"-fejl ved launch.** Deployment-omfanget, der blev registreret under dynamisk registrering, matcher ikke den institutionskontekst, som kurset tilhører. I Blackboards tool provider-post skal du verificere, at **Deployment ID** matcher, hvad FastComments viser på sin LTI 1.3 Configuration-side for denne tenant. Hvis de adskiller sig, skal du slette placeringen og køre dynamisk registrering igen fra en frisk registrerings-URL (<a href="https://fastcomments.com/auth/my-account/lti-config" target="_blank">få den her</a>).

**Iframe-højden ser fast ud, eller indhold bliver klippet af.** Nogle Blackboard-tenants leveres med en streng Content Security Policy, der blokerer standard LTI iframe-resize postMessage. FastComments udsender både Canvas-stil `lti.frameResize`-beskeden og IMS spec-form `org.imsglobal.lti.frameResize`-beskeden for at maksimere kompatibiliteten, men en CSP-override på tenant-niveau blokerer forældrelytteren. Bed din administrator om at bekræfte, at `*.fastcomments.com` er på LTI-værktøjets allowlist, og at ingen brugerdefineret CSP-header fjerner postMessage-hændelser. Resize fungerer derefter uden yderligere konfiguration.

**Kursuskopiering duplikerer tråde.** Blackboard kursuskopiering udsteder nye resource link-ID'er for LTI-placeringer, så kopierede kurser starter med tomme tråde. Dette er forventet. Hvis du har brug for, at det kopierede kursus arver den originale tråd, skal du opsætte Deep Linking med en eksplicit tråd-URN før kopiering eller kontakte FastComments-support for at remappe tråd-ID'er samlet.

**Studerende ser en generisk Blackboard-fejl ved launch.** Årsagen er et manglende eller forældet `email`-claim. Bekræft, at institutionspolitikken for FastComments har **Role**, **Name** og **Email Address** aktiveret under **User Fields to Send**. Gem, og launch derefter igen i en frisk browsersession.
Once an administrator has registered FastComments as an LTI 1.3 Advantage tool and approved the institution policies, instructors add it to courses through the standard Blackboard placement points. The exact steps differ between Ultra Course View and Original Course View, so both are covered below.

#### Ultra Course View

Ultra Course View jest domyślnym widokiem w Blackboard Learn SaaS od 2026 roku.

1. Otwórz kurs i przejdź do strony **Course Content**.
2. Najedź kursorem lub dotknij miejsca, w którym chcesz, aby wątek komentarzy pojawił się w konspekcie, i kliknij fioletowy przycisk **+** (Add content).
3. Wybierz **Content Market**. Panel Content Market wyświetla wszystkie zatwierdzone narzędzia LTI i umiejscowienia Building Block dla Twojej instytucji.
4. Znajdź kafelek **FastComments** i kliknij go. Blackboard tworzy element treści w miejscu, w którym otworzyłeś menu **+**.
5. Element pojawia się w konspekcie jako pozycja „Visible to students” domyślnie dla instruktorów, którzy mają wyłączone **Hide from students** jako domyślne ustawienie osobiste. Jeśli Twoje domyślne ustawienie to **Hidden**, element zostaje utworzony jako ukryty i możesz przełączyć selektor widoczności w wierszu elementu, gdy będziesz gotowy.
6. Aby zmienić nazwę elementu, kliknij tytuł w konspekcie i wpisz nową etykietę. Tytuł, który widzą studenci w konspekcie, jest niezależny od identyfikatora wątku FastComments, więc zmiana nazwy jest bezpieczna w dowolnym momencie.

Jeśli nie widzisz opcji **Content Market**, Twoja instytucja ukryła to miejsce umieszczania. Ten sam wybór możesz uzyskać poprzez **More tools** w tym samym menu **+** w grupie **LTI Tools**.

#### Original Course View

Original Course View jest nadal wspierany w Learn SaaS i pozostaje głównym doświadczeniem dla samodzielnie hostowanych witryn Learn 9.1 w linii wydań Q4 2024 CU.

1. Otwórz kurs i wejdź do **Content Area** (na przykład domyślnego obszaru **Information** lub **Content** w menu kursu).
2. Włącz **Edit Mode** przy użyciu przełącznika w prawym górnym rogu strony.
3. Kliknij **Build Content** na pasku akcji.
4. W podmenu **Learning Tools** kliknij **FastComments**. Podmenu Learning Tools jest wypełniane z miejsc umieszczania narzędzi LTI 1.3 po zarejestrowaniu narzędzia przez administratora. Jeśli go nie widzisz, zobacz sekcję z pułapkami poniżej.
5. Na formularzu **Create FastComments** ustaw:
   - **Name**: etykieta, którą studenci widzą w obszarze treści.
   - **Description**: opcjonalny tekst wyświetlany nad osadzonym wątkiem.
   - **Permit Users to View this Content**: przełącznik dostępności Tak/Nie.
   - **Track Number of Views**: włącz, jeśli chcesz statystyki wyświetleń per element w Blackboard. FastComments prowadzi własną analizę niezależnie.
   - **Date and Time Restrictions**: opcjonalne okna **Display After** / **Display Until**.
6. Zatwierdź. Narzędzie pojawia się jako klikalny element w obszarze treści.

#### Embedding Inside an Item or Document

W obu widokach kursu instruktorzy osadzają FastComments w treści elementu Item, Document lub dowolnego pola rich‑text za pomocą przycisku LTI Advantage w edytorze treści.

**Ultra Course View**:

1. Utwórz lub edytuj **Document**.
2. Kliknij **Add content** w treści dokumentu w miejscu, w którym chcesz, aby pojawił się wątek.
3. W pasku narzędzi edytora otwórz menu **Insert content** i kliknij **Content Market** (punkt wejścia LTI Advantage / Deep Linking).
4. Wybierz **FastComments**. FastComments zwraca ładunek deep‑link, a Blackboard wstawia osadzony blok w treści dokumentu w miejscu kursora.
5. Zapisz dokument. Studenci widzą wątek renderowany w treści podczas przewijania.

**Original Course View**:

1. Edytuj dowolny element z treścią rich‑text.
2. W pasku narzędzi Content Editor kliknij ikonę plus **Add Content** i wybierz **Content Market** (oznaczone jako **Add Content from External Tool** w starszych wersjach Q4 2024 CU).
3. Wybierz **FastComments**. Edytor wstawia blok zastępczy odwołujący się do zasobu deep‑linked.
4. Zatwierdź element.

Każde osadzenie deep‑link tworzy własny wątek FastComments, więc element z dwoma osadzonymi blokami FastComments ma dwa niezależne strumienie komentarzy.

#### Visibility, Release Conditions, and Group Restrictions

Elementy treści FastComments zachowują się jak każdy inny element treści Blackboard pod względem reguł kontroli dostępu nakładanych na nie.

- Ultra: kliknij selektor widoczności w wierszu (**Visible to students**, **Hidden from students**, **Conditional availability**). Warunkowa dostępność obsługuje okna dat/czasów, reguły wydajności względem elementów dziennika ocen oraz reguły członkostwa względem grup kursu.
- Original: otwórz menu kontekstowe elementu i wybierz **Adaptive Release** lub **Adaptive Release: Advanced**, aby ograniczyć narzędzie według daty, członkostwa, oceny lub statusu przeglądu. Użyj **Set Group Availability** na elemencie, aby ograniczyć dostęp do konkretnych grup kursu.

FastComments respektuje wszelkie decyzje bramki Blackboard. Jeśli Blackboard ukryje element przed studentem, uruchomienie LTI nigdy nie nastąpi dla tego studenta i nie pojawi się on w widoku moderatora.

#### Gradebook Behavior

FastComments nie raportuje ocen z powrotem przez LTI Advantage Assignment i Grade Services. Żadna kolumna ocen nie jest automatycznie tworzona dla elementów treści FastComments.

Jeśli Twój najemca Blackboard jest skonfigurowany tak, aby automatycznie tworzyć kolumnę dziennika ocen dla każdego nowego elementu treści, niezależnie od metadanych ocen, i tak pojawia się pusta kolumna. Aby ją ukryć:

- Ultra: otwórz **Gradebook**, kliknij nagłówek kolumny, wybierz **Edit** i wyłącz **Show to students** oraz **Include in calculations**. Lub użyj **Delete**, jeśli Twoja instytucja zezwala na usuwanie kolumn dla nieocenianych elementów.
- Original: otwórz **Grade Center**, kliknij strzałkę kolumny, wybierz **Hide from Users (on/off)** i opcjonalnie **Hide from Instructor View** w sekcji **Column Organization**.

#### What Students See

Gdy student otwiera element FastComments lub przewija do osadzonego bloku:

1. Blackboard uruchamia wiadomość LTI 1.3 do FastComments. Student jest zalogowany przez SSO przy użyciu swojej tożsamości Blackboard (imię, e‑mail, awatar, rola) bez wyświetlania formularza logowania.
2. Wątek komentarzy renderuje się w iframe. Wątkowanie, odpowiedzi, wzmianki i reakcje są dostępne w zależności od ustawień widgetu komentarzy skonfigurowanych w FastComments.
3. Ich komentarze są przypisane do ich konta Blackboard. Jeśli student później edytuje swoje imię lub zdjęcie w Blackboard, przy następnym uruchomieniu profil FastComments zostanie zaktualizowany.

Mapowanie ról z Blackboard na FastComments:

- **System Administrator** i **Course Builder** mapują się na FastComments **admin**.
- **Instructor** i **Teaching Assistant** mapują się na FastComments **moderator**.
- **Student**, **Guest** i **Observer** mapują się na FastComments **commenter**.

Moderatorzy widzą kontrolki moderacji (przypinanie, ukrywanie, banowanie, usuwanie) w treści każdego komentarza w wątku.

#### Lock Down Public Access (Recommended)

Domyślnie dane komentarzy FastComments są publicznie czytelne. Każdy, kto odgadnie URL wątku lub punkt końcowy API, może zobaczyć jego komentarze, nawet poza Blackboard. W dyskusjach kursowych prawie na pewno chcesz ograniczyć podgląd tylko do zapisanych studentów.

Otwórz swoją <a href="https://fastcomments.com/auth/my-account/customize-widget" target="_blank">stronę dostosowywania widgetu</a> i utwórz regułę z włączonym **Require SSO To View Comments**, a następnie ustaw poziom zabezpieczeń na **Secure SSO**, aby wątki mogły być ładowane tylko przez podpisane uruchomienie LTI.

Zobacz [Protecting Comment Threads With Single-Sign-On](/guide-customizations-and-configuration.html#sso-require-to-view-comments) po pełną instrukcję, w tym jak ograniczyć regułę do jednej domeny lub strony.

#### Thread Scoping

FastComments określa zakres każdego wątku według **(Blackboard host, course ID, resource link ID)**. Dwa elementy FastComments w tym samym kursie tworzą dwa wątki. Ten sam element skopiowany do dwóch powłok kursu (na przykład poprzez kopiowanie kursu) tworzy dwa wątki, ponieważ Blackboard przy kopiowaniu generuje nowy resource link ID. Aby zachować wspólny wątek w kopiach kursu, użyj Deep Linking z wyraźnie skonfigurowanym URN wątku w FastComments przed uruchomieniem kopiowania.

#### Blackboard-Specific Gotchas

**FastComments tile missing from the Build Content menu (Original) or Content Market (Ultra).** Administrator zatwierdził narzędzie, ale pozostawił politykę instytucji blokującą odpowiednie miejsce umieszczania. Przejdź do **Administrator Panel** > **Integrations** > **LTI Tool Providers**, edytuj wpis FastComments i potwierdź, że zarówno **Course Content Tool** (Original), jak i **Course Content Tool - allow students** / **Deep Linking content tool** (Ultra) są włączone. Zapisz i odśwież stronę kursu.

**"Tool not configured for this context" or "Tool is not deployed" error on launch.** Zakres wdrożenia zarejestrowany podczas dynamicznej rejestracji nie pasuje do kontekstu instytucji, do którego należy kurs. W wpisie dostawcy narzędzia w Blackboard, sprawdź, czy **Deployment ID** odpowiada temu, co FastComments wyświetla na swojej stronie konfiguracji LTI 1.3 dla tego najemcy. Jeśli się różnią, usuń miejsce umieszczania i ponownie uruchom dynamiczną rejestrację z nowego URL rejestracji (<a href="https://fastcomments.com/auth/my-account/lti-config" target="_blank">get it here</a>).

**Iframe height looks fixed or content gets cut off.** Niektóre najemcy Blackboard mają restrykcyjną Content Security Policy, która blokuje domyślny postMessage `lti.frameResize`. FastComments wysyła zarówno wiadomość w stylu Canvas `lti.frameResize`, jak i wiadomość w formacie spec IMS `org.imsglobal.lti.frameResize`, aby maksymalizować kompatybilność, ale nadpisanie CSP na poziomie najemcy blokuje nasłuchiwacz rodzica. Poproś administratora, aby potwierdził, że `*.fastcomments.com` znajduje się na liście dozwolonych narzędzi LTI i że żaden niestandardowy nagłówek CSP nie usuwa zdarzeń postMessage. Zmiana rozmiaru będzie działać bez dalszej konfiguracji.

**Course copy duplicates threads.** Kopiowanie kursu w Blackboard generuje nowe resource link ID dla umiejscowień LTI, więc skopiowane kursy zaczynają się od pustych wątków. To jest oczekiwane. Jeśli potrzebujesz, aby skopiowany kurs odziedziczył oryginalny wątek, skonfiguruj Deep Linking z wyraźnym URN wątku przed kopiowaniem lub skontaktuj się z wsparciem FastComments, aby masowo przemapować ID wątków.

**Student sees a generic Blackboard error on launch.** Przyczyną jest brakujący lub przestarzały roszczenie `email`. Potwierdź, że polityka instytucji dla FastComments ma włączone **Role**, **Name** i **Email Address** w sekcji **User Fields to Send**. Zapisz, a następnie uruchom ponownie w nowej sesji przeglądarki.
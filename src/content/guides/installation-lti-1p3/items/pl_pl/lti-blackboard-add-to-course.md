Gdy administrator zarejestruje FastComments jako narzędzie LTI 1.3 Advantage i zatwierdzi polityki instytucji, wykładowcy dodają je do kursów poprzez standardowe punkty osadzania w Blackboard. Dokładne kroki różnią się między Ultra Course View a Original Course View, więc oba zostały opisane poniżej.

#### Ultra Course View

Ultra Course View jest domyślnym widokiem w Blackboard Learn SaaS od 2026 roku.

1. Otwórz kurs i przejdź do strony **Course Content**.
2. Najedź kursorem lub dotknij miejsca, w którym ma się znaleźć wątek komentarzy w konspekcie, i kliknij fioletowy przycisk **+** (Add content).
3. Wybierz **Content Market**. Panel Content Market wyświetla wszystkie zatwierdzone narzędzia LTI i osadzenia Building Block dla Twojej instytucji.
4. Znajdź kafelek **FastComments** i kliknij go. Blackboard tworzy element treści w miejscu, w którym otwarto menu **+**.
5. Element trafia do konspektu jako wpis "Visible to students" domyślnie dla wykładowców, którzy mają wyłączoną opcję **Hide from students** jako osobiste ustawienie domyślne. Jeśli Twoje ustawienie domyślne to **Hidden**, element zostanie utworzony jako ukryty, a przełącznik widoczności w wierszu elementu możesz zmienić, gdy będziesz gotowy.
6. Aby zmienić nazwę elementu, kliknij tytuł w konspekcie i wpisz nową etykietę. Tytuł widoczny dla studentów w konspekcie jest niezależny od identyfikatora wątku FastComments, więc zmiana nazwy jest bezpieczna w dowolnym momencie.

Jeśli nie widzisz opcji **Content Market**, Twoja instytucja ukryła ten punkt osadzania. Ten sam wybór narzędzi możesz osiągnąć przez **More tools** w tym samym menu **+** w grupie **LTI Tools**.

#### Original Course View

Original Course View jest nadal obsługiwany w Learn SaaS i pozostaje podstawowym doświadczeniem dla samodzielnie hostowanych stron Learn 9.1 w linii wydań Q4 2024 CU.

1. Otwórz kurs i wejdź do **Content Area** (na przykład domyślnego obszaru **Information** lub **Content** w menu kursu).
2. Włącz **Edit Mode** przełącznikiem w prawym górnym rogu strony.
3. Kliknij **Build Content** w pasku akcji.
4. W podmenu **Learning Tools** kliknij **FastComments**. Podmenu Learning Tools jest wypełniane na podstawie osadzeń narzędzi LTI 1.3 po zarejestrowaniu narzędzia przez administratora. Jeśli go nie widzisz, zobacz sekcję pułapek poniżej.
5. Na formularzu **Create FastComments** ustaw:
   - **Name**: etykieta widoczna dla studentów w obszarze treści.
   - **Description**: opcjonalny tekst wyświetlany nad osadzonym wątkiem.
   - **Permit Users to View this Content**: przełącznik dostępności Tak/Nie.
   - **Track Number of Views**: włącz, jeśli chcesz korzystać ze statystyk wyświetleń per element w Blackboard. FastComments prowadzi własną analitykę niezależnie.
   - **Date and Time Restrictions**: opcjonalne okna **Display After** / **Display Until**.
6. Zatwierdź. Narzędzie pojawi się jako klikalny element w obszarze treści.

#### Osadzanie wewnątrz elementu lub dokumentu

W obu widokach kursu wykładowcy osadzają FastComments w treści elementu, dokumentu lub dowolnego pola tekstu sformatowanego za pomocą przycisku LTI Advantage w edytorze treści.

Ultra Course View:

1. Utwórz lub edytuj **Document**.
2. Kliknij **Add content** w treści dokumentu w miejscu, w którym ma się pojawić wątek.
3. Na pasku narzędzi edytora otwórz menu **Insert content** i kliknij **Content Market** (punkt wejścia LTI Advantage / Deep Linking).
4. Wybierz **FastComments**. FastComments zwraca ładunek deep-link, a Blackboard wstawia osadzony blok w treści dokumentu w pozycji kursora.
5. Zapisz dokument. Studenci widzą wątek renderowany w treści, gdy przewijają dokument.

Original Course View:

1. Edytuj dowolny element z treścią tekstu sformatowanego.
2. Na pasku narzędzi edytora treści kliknij ikonę plus **Add Content** i wybierz **Content Market** (w starszych Q4 2024 CU oznaczone jako **Add Content from External Tool**).
3. Wybierz **FastComments**. Edytor wstawia blok zastępczy odwołujący się do zasobu deep-link.
4. Zatwierdź element.

Każde osadzenie deep-link tworzy własny wątek FastComments, więc element z dwoma osadzonymi blokami FastComments ma dwa niezależne strumienie komentarzy.

#### Widoczność, warunki udostępnienia i ograniczenia grupowe

Elementy treści FastComments zachowują się jak każdy inny element treści Blackboard w odniesieniu do reguł kontroli dostępu nakładanych na nie.

- Ultra: kliknij przełącznik widoczności w wierszu (**Visible to students**, **Hidden from students**, **Conditional availability**). Conditional availability obsługuje okna daty/czasu, reguły wydajności względem elementów dziennika ocen oraz reguły członkostwa względem grup kursu.
- Original: otwórz menu kontekstowe elementu i wybierz **Adaptive Release** lub **Adaptive Release: Advanced**, aby ograniczyć dostęp do narzędzia według daty, członkostwa, oceny lub statusu przeglądu. Użyj **Set Group Availability** na elemencie, aby ograniczyć dostęp do konkretnych grup kursu.

FastComments respektuje decyzję bramki Blackboard. Jeśli Blackboard ukryje element przed studentem, uruchomienie LTI nigdy nie następuje dla tego studenta i nie pojawia się on w widoku moderatora.

#### Zachowanie dziennika ocen

FastComments nie przesyła ocen z powrotem przez LTI Advantage Assignment and Grade Services. Żadna kolumna ocen nie jest automatycznie tworzona dla elementów treści FastComments.

Jeśli Twoja instancja Blackboard jest skonfigurowana tak, aby automatycznie tworzyć kolumnę dziennika ocen dla każdego nowego elementu treści niezależnie od metadanych oceniania, pusta kolumna i tak się pojawi. Aby ją ukryć:

- Ultra: otwórz **Gradebook**, kliknij nagłówek kolumny, wybierz **Edit** i wyłącz **Show to students** oraz **Include in calculations**. Albo użyj **Delete**, jeśli Twoja instytucja zezwala na usuwanie kolumn dla nieocenianych elementów.
- Original: otwórz **Grade Center**, kliknij strzałkę kolumny, wybierz **Hide from Users (on/off)** i opcjonalnie **Hide from Instructor View** w **Column Organization**.

#### Co widzą studenci

Gdy student otworzy element FastComments lub przewinie do osadzonego bloku:

1. Blackboard uruchamia komunikat LTI 1.3 do FastComments. Student jest logowany przez SSO przy użyciu swojej tożsamości Blackboard (imię i nazwisko, e-mail, awatar, rola) bez widzenia formularza logowania.
2. Wątek komentarzy renderuje się w iframe. Wątki, odpowiedzi, wzmianki i reakcje są dostępne zgodnie z ustawieniami widżetu komentarzy skonfigurowanymi w FastComments.
3. Ich komentarze są przypisane do konta Blackboard. Jeśli student później zmieni imię i nazwisko lub zdjęcie w Blackboard, następne uruchomienie zaktualizuje profil FastComments.

Mapowanie ról z Blackboard na FastComments:

- **System Administrator** i **Course Builder** mapują na **admin** FastComments.
- **Instructor** i **Teaching Assistant** mapują na **moderator** FastComments.
- **Student**, **Guest** i **Observer** mapują na **commenter** FastComments.

Moderatorzy widzą narzędzia moderacji (przypnij, ukryj, zbanuj, usuń) bezpośrednio przy każdym komentarzu w wątku.

#### Blokowanie dostępu publicznego (zalecane)

Domyślnie dane komentarzy FastComments są publicznie czytelne. Każdy, kto odgadnie adres URL wątku lub endpoint API, może wyświetlić jego komentarze, nawet poza Blackboard. W przypadku dyskusji kursowych niemal na pewno chcesz ograniczyć przeglądanie wyłącznie do zapisanych studentów.

Otwórz swoją <a href="https://fastcomments.com/auth/my-account/customize-widget" target="_blank">stronę dostosowywania widżetu</a> i utwórz regułę z włączoną opcją **Require SSO To View Comments**, a następnie ustaw poziom bezpieczeństwa na **Secure SSO**, aby wątki mogły być ładowane wyłącznie przez podpisane uruchomienie LTI.

Zobacz [Ochrona wątków komentarzy za pomocą Single-Sign-On](/guide-customizations-and-configuration.html#sso-require-to-view-comments) dla pełnego przewodnika, w tym jak zawęzić regułę do pojedynczej domeny lub strony.

#### Zakres wątków

FastComments określa zakres każdego wątku według **(host Blackboard, ID kursu, ID resource link)**. Dwa elementy FastComments w tym samym kursie tworzą dwa wątki. Ten sam element skopiowany do dwóch powłok kursu (na przykład przez kopiowanie kursu) tworzy dwa wątki, ponieważ Blackboard generuje nowy resource link ID podczas kopiowania. Aby zachować wspólny wątek między kopiami kursu, użyj Deep Linking z jawnym URN wątku skonfigurowanym w FastComments przed uruchomieniem kopiowania.

#### Pułapki specyficzne dla Blackboard

**Brak kafelka FastComments w menu Build Content (Original) lub Content Market (Ultra).** Administrator zatwierdził narzędzie, ale polityka instytucji blokuje odpowiednie osadzenie. Przejdź do **Administrator Panel** > **Integrations** > **LTI Tool Providers**, edytuj wpis FastComments i upewnij się, że zarówno osadzenia **Course Content Tool** (Original), jak i **Course Content Tool - allow students** / **Deep Linking content tool** (Ultra) są włączone. Zapisz i odśwież stronę kursu.

**Błąd "Tool not configured for this context" lub "Tool is not deployed" przy uruchomieniu.** Zakres wdrożenia zarejestrowany podczas rejestracji dynamicznej nie pasuje do kontekstu instytucji, do którego należy kurs. We wpisie dostawcy narzędzia w Blackboard sprawdź, czy **Deployment ID** zgadza się z tym, co FastComments pokazuje na swojej stronie konfiguracji LTI 1.3 dla tej instancji. Jeśli się różnią, usuń osadzenie i ponownie przeprowadź rejestrację dynamiczną z nowego adresu URL rejestracji (<a href="https://fastcomments.com/auth/my-account/lti-config" target="_blank">pobierz go tutaj</a>).

**Wysokość iframe wydaje się stała lub treść jest ucinana.** Niektóre instancje Blackboard mają ścisłą politykę Content Security Policy, która blokuje domyślny postMessage zmiany rozmiaru iframe LTI. FastComments wysyła zarówno komunikat w stylu Canvas `lti.frameResize`, jak i zgodny ze specyfikacją IMS `org.imsglobal.lti.frameResize`, aby zmaksymalizować kompatybilność, ale nadpisanie CSP na poziomie instancji blokuje nasłuchiwacz po stronie rodzica. Poproś administratora o potwierdzenie, że `*.fastcomments.com` znajduje się na liście dozwolonych narzędzi LTI i że żaden niestandardowy nagłówek CSP nie odfiltrowuje zdarzeń postMessage. Wtedy zmiana rozmiaru działa bez dalszej konfiguracji.

**Kopiowanie kursu duplikuje wątki.** Kopiowanie kursu w Blackboard generuje nowe resource link ID dla osadzeń LTI, więc skopiowane kursy zaczynają się z pustymi wątkami. To oczekiwane zachowanie. Jeśli potrzebujesz, aby skopiowany kurs dziedziczył oryginalny wątek, skonfiguruj Deep Linking z jawnym URN wątku przed kopiowaniem lub skontaktuj się z pomocą techniczną FastComments, aby masowo przemapować identyfikatory wątków.

**Student widzi ogólny błąd Blackboard przy uruchomieniu.** Przyczyną jest brakujący lub nieaktualny claim `email`. Potwierdź, że polityka instytucji dla FastComments ma włączone **Role**, **Name** i **Email Address** w sekcji **User Fields to Send**. Zapisz, a następnie uruchom ponownie w świeżej sesji przeglądarki.
Po tym, jak administrator zarejestruje FastComments jako narzędzie LTI 1.3 Advantage i zatwierdzi polityki instytucji, instruktorzy dodają je do kursów poprzez standardowe punkty umieszczania w Blackboard. Dokładne kroki różnią się między Widokiem kursu Ultra a Widokiem kursu Original, dlatego poniżej omówiono obie opcje.

#### Widok kursu Ultra

Widok kursu Ultra jest domyślny w Blackboard Learn SaaS od 2026 roku.

1. Otwórz kurs i przejdź do strony **Zawartość kursu**.
2. Najedź kursorem lub stuknij miejsce w konspekcie, gdzie chcesz umieścić wątek komentarzy, i kliknij fioletowy przycisk **+** (Dodaj treść).
3. Wybierz **Content Market**. Panel Content Market wyświetla wszystkie zatwierdzone narzędzia LTI i umiejscowienia Building Block dla Twojej instytucji.
4. Znajdź kafelek **FastComments** i kliknij go. Blackboard utworzy element zawartości w miejscu, gdzie otworzyłeś menu **+**.
5. Element pojawia się w konspekcie jako wpis „Widoczne dla studentów” domyślnie dla instruktorów, którzy mają ustawione jako osobisty domyślny stan **Ukryj przed studentami** wyłączony. Jeśli Twój domyślny stan to **Ukryte**, element zostanie utworzony jako ukryty i włączyć przełącznik widoczności na wierszu elementu, gdy będziesz gotowy.
6. Aby zmienić nazwę elementu, kliknij tytuł w konspekcie i wpisz nową etykietę. Tytuł widoczny dla studentów w konspekcie jest niezależny od identyfikatora wątku FastComments, więc zmiana nazwy jest bezpieczna w dowolnym momencie.

Jeśli nie widzisz **Content Market** jako opcji, Twoja instytucja ukryła to umiejscowienie. Możesz też otworzyć ten sam wybór przez **Więcej narzędzi** w tym samym menu **+** pod grupą **Narzędzia LTI**.

#### Widok kursu Original

Widok kursu Original jest wciąż obsługiwany w Learn SaaS i pozostaje głównym doświadczeniem dla samodzielnie hostowanych witryn Learn 9.1 na linii wydań CU z Q4 2024.

1. Otwórz kurs i wejdź do **Obszaru zawartości** (na przykład domyślnego obszaru **Informacje** lub **Zawartość** w menu kursu).
2. Włącz **Tryb edycji** za pomocą przełącznika w prawym górnym rogu strony.
3. Kliknij **Build Content** na pasku akcji.
4. W podmenu **Learning Tools** kliknij **FastComments**. Podmenu Learning Tools jest wypełniane z umiejscowień narzędzia LTI 1.3 po tym, jak administrator zarejestruje narzędzie. Jeśli go nie widzisz, zobacz sekcję „uwagi” poniżej.
5. W formularzu **Create FastComments** ustaw:
   - **Name**: etykieta widoczna dla studentów w obszarze zawartości.
   - **Description**: opcjonalny tekst wyświetlany nad osadzonym wątkiem.
   - **Permit Users to View this Content**: przełącznik dostępności Tak/Nie.
   - **Track Number of Views**: włącz, jeśli chcesz statystyki wyświetleń dla pozycji w Blackboard. FastComments prowadzi własną analitykę niezależnie.
   - **Date and Time Restrictions**: opcjonalne okna **Display After** / **Display Until**.
6. Wyślij. Narzędzie pojawi się jako element klikalny w obszarze zawartości.

#### Osadzanie wewnątrz elementu lub dokumentu

W obu widokach kursu instruktorzy osadzają FastComments inline w treści elementu, dokumentu lub dowolnego pola bogatego tekstu za pomocą przycisku LTI Advantage w Edytorze treści.

Widok kursu Ultra:

1. Utwórz lub edytuj **Dokument**.
2. Kliknij **Add content** wewnątrz treści dokumentu w miejscu, gdzie chcesz, aby pojawił się wątek.
3. W pasku narzędzi edytora otwórz menu **Wstaw zawartość** i kliknij **Content Market** (punkt wejścia LTI Advantage / Deep Linking).
4. Wybierz **FastComments**. FastComments zwróci ładunek deep-link, a Blackboard wstawi osadzony blok w treści dokumentu w pozycji kursora.
5. Zapisz dokument. Studenci zobaczą wątek wyrenderowany inline, gdy przewiną do niego.

Widok kursu Original:

1. Edytuj dowolny element z treścią typu rich-text.
2. W pasku narzędzi Content Editor kliknij ikonę plus **Add Content** i wybierz **Content Market** (oznaczone jako **Add Content from External Tool** w starszych CU z Q4 2024).
3. Wybierz **FastComments**. Edytor wstawi blok zastępczy odnoszący się do zasobu deep-link.
4. Wyślij element.

Każde osadzenie deep-link tworzy własny wątek FastComments, więc element z dwoma osadzonymi blokami FastComments ma dwa niezależne strumienie komentarzy.

#### Widoczność, warunki udostępniania i ograniczenia grupowe

Elementy zawartości FastComments zachowują się jak każdy inny element zawartości w Blackboard pod względem zasad kontroli dostępu na nich nakładanych.

- Ultra: kliknij selektor widoczności na wierszu (**Widoczne dla studentów**, **Ukryte przed studentami**, **Dostęp warunkowy**). Dostęp warunkowy obsługuje okna dat/godzin, reguły wydajności względem elementów w księdze ocen oraz reguły członkostwa względem grup kursu.
- Original: otwórz menu kontekstowe elementu i wybierz **Adaptive Release** lub **Adaptive Release: Advanced**, aby ograniczyć narzędzie według daty, członkostwa, oceny lub statusu przeglądu. Użyj **Set Group Availability** na elemencie, aby ograniczyć dostęp do konkretnych grup kursu.

FastComments respektuje decyzję bramki Blackboard. Jeśli Blackboard ukryje element przed studentem, uruchomienie LTI nigdy nie nastąpi dla tego studenta i nie pojawi się on w widoku moderatora.

#### Zachowanie w Księdze ocen

FastComments nie przesyła ocen z powrotem przez LTI Advantage Assignment and Grade Services. Dla elementów zawartości FastComments nie tworzy się automatycznie kolumna ocen.

Jeśli Twoja instancja Blackboard jest skonfigurowana tak, że automatycznie tworzy kolumnę w księdze ocen dla każdego nowego elementu zawartości niezależnie od metadanych oceny, pojawi się pusta kolumna. Aby ją ukryć:

- Ultra: otwórz **Gradebook**, kliknij nagłówek kolumny, wybierz **Edit** i wyłącz **Show to students** oraz **Include in calculations**. Albo użyj **Delete**, jeśli Twoja instytucja zezwala na usuwanie kolumn dla pozycji nieocenianych.
- Original: otwórz **Grade Center**, kliknij trójkąt przy kolumnie, wybierz **Hide from Users (on/off)**, a opcjonalnie **Hide from Instructor View** w **Column Organization**.

#### Co widzą studenci

Gdy student otworzy element FastComments lub przewinie do osadzonego bloku:

1. Blackboard uruchamia komunikat LTI 1.3 do FastComments. Student zostaje zalogowany przez SSO przy użyciu swojej tożsamości Blackboard (imię, e-mail, avatar, rola) bez wyświetlania formularza logowania.
2. Wątek komentarzy renderuje się w iframe. Wątkowanie, odpowiedzi, wzmianki i reakcje są dostępne w zależności od ustawień widżetu komentarzy skonfigurowanych w FastComments.
3. Ich komentarze są przypisywane do konta Blackboard. Jeśli student później zmieni imię lub zdjęcie w Blackboard, przy następnym uruchomieniu profil FastComments zostanie zaktualizowany.

Mapowanie ról z Blackboard do FastComments:

- **System Administrator** i **Course Builder** mapują się na FastComments **admin**.
- **Instructor** i **Teaching Assistant** mapują się na FastComments **moderator**.
- **Student**, **Guest** i **Observer** mapują się na FastComments **commenter**.

Moderatorzy widzą kontrolki moderacji (przypnij, ukryj, zablokuj, usuń) bezpośrednio na każdym komentarzu w wątku.

#### Zakres wątku

FastComments określa zakres każdego wątku przez **(host Blackboard, ID kursu, ID linku zasobu)**. Dwa elementy FastComments w tym samym kursie tworzą dwa wątki. Ten sam element skopiowany do dwóch szablonów kursu (na przykład przez kopiowanie kursu) tworzy dwa wątki, ponieważ Blackboard wydaje nowy resource link ID podczas kopiowania. Aby zachować współdzielony wątek po kopiowaniu kursu, użyj Deep Linking z jawnie określonym URN wątku skonfigurowanym w FastComments przed uruchomieniem kopiowania.

#### Specyficzne uwagi dotyczące Blackboard

**Brak kafelka FastComments w menu Build Content (Original) lub w Content Market (Ultra).** Administrator zatwierdził narzędzie, ale pozostawił politykę instytucji blokującą odpowiednie umiejscowienie. Przejdź do **Administrator Panel** > **Integrations** > **LTI Tool Providers**, edytuj wpis FastComments i potwierdź, że zarówno umiejscowienie **Course Content Tool** (Original), jak i **Course Content Tool - allow students** / **Deep Linking content tool** (Ultra) są włączone. Zapisz i odśwież stronę kursu.

**Błąd „Tool not configured for this context” lub „Tool is not deployed” podczas uruchamiania.** Zakres wdrożenia zarejestrowany podczas rejestracji dynamicznej nie odpowiada kontekstowi instytucji, do którego należy kurs. W wpisie dostawcy narzędzia w Blackboard sprawdź, czy **Deployment ID** odpowiada temu, który FastComments pokazuje na swojej stronie Konfiguracji LTI 1.3 dla tego tenant. Jeśli się różnią, usuń umiejscowienie i ponownie uruchom rejestrację dynamiczną z nowego adresu rejestracji (<a href="https://fastcomments.com/auth/my-account/lti-config" target="_blank">uzyskaj go tutaj</a>).

**Wysokość iframe wydaje się stała lub zawartość jest obcinana.** Niektóre instancje Blackboard mają rygorystyczną Content Security Policy, która blokuje domyślny postMessage do zmiany rozmiaru iframe LTI. FastComments emituje zarówno komunikat w stylu Canvas `lti.frameResize`, jak i komunikat zgodny ze specyfikacją IMS `org.imsglobal.lti.frameResize`, aby zmaksymalizować kompatybilność, ale nakładka CSP na poziomie tenant może blokować listener w rodzicu. Poproś administratora, aby potwierdził, że `*.fastcomments.com` jest na liście dozwolonych narzędzi LTI i że żaden niestandardowy nagłówek CSP nie usuwa zdarzeń postMessage. Po tym zmiana rozmiaru powinna działać bez dalszej konfiguracji.

**Kopiowanie kursu powoduje duplikowanie wątków.** Przy kopiowaniu kursu Blackboard wydaje nowe resource link ID dla umiejscowień LTI, więc skopiowane kursy zaczynają z pustymi wątkami. To zachowanie jest oczekiwane. Jeśli chcesz, aby skopiowany kurs odziedziczył oryginalny wątek, skonfiguruj Deep Linking z jawnie określonym URN wątku przed kopiowaniem lub skontaktuj się z pomocą techniczną FastComments, aby masowo przypisać ID wątków.

**Student widzi ogólny błąd Blackboard przy uruchomieniu.** Przyczyną jest brakujący lub przestarzały claim `email`. Potwierdź, że polityka instytucji dla FastComments ma włączone **Role**, **Name** oraz **Email Address** w sekcji **User Fields to Send**. Zapisz ustawienia, a następnie uruchom ponownie w świeżej sesji przeglądarki.
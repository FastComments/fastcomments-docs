Po tym, jak administrator zarejestruje FastComments jako narzędzie LTI 1.3 Advantage i zatwierdzi polityki instytucji, instruktorki/instuktorzy dodają je do kursów przez standardowe punkty umiejscowienia w Blackboard. Dokładne kroki różnią się między Ultra Course View a Original Course View, dlatego oba przypadki opisano poniżej.

#### Ultra Course View

Ultra Course View jest domyślny w Blackboard Learn SaaS od 2026 roku.

1. Otwórz kurs i przejdź na stronę **Zawartość kursu**.
2. Najedź kursorem lub stuknij miejsce, w którym chcesz umieścić wątek komentarzy na planie kursu i kliknij fioletowy przycisk **+** (Dodaj zawartość).
3. Wybierz **Content Market**. Panel Content Market wyświetla wszystkie zatwierdzone narzędzia LTI i umiejscowienia Building Block dla Twojej instytucji.
4. Znajdź kafelek **FastComments** i kliknij go. Blackboard tworzy element zawartości w miejscu, w którym otworzyłeś menu **+**.
5. Element domyślnie trafia do planu kursu jako wpis „Widoczne dla studentów” dla instruktorów, którzy mają ustawienie domyślne **Ukryj przed studentami** wyłączone. Jeśli Twoje ustawienie domyślne to **Ukryte**, element jest tworzony jako ukryty i włączasz przełącznik widoczności na wierszu elementu, gdy będziesz gotowy.
6. Aby zmienić nazwę elementu, kliknij tytuł w planie kursu i wpisz nową etykietę. Tytuł widoczny dla studentów w planie kursu jest niezależny od identyfikatora wątku FastComments, więc zmiana nazwy jest bezpieczna w dowolnym momencie.

Jeśli nie widzisz opcji **Content Market**, Twoja instytucja ukryła to umiejscowienie. Ten sam wybór osiągniesz także przez **Więcej narzędzi** w tym samym menu **+** w grupie **Narzędzia LTI**.

#### Original Course View

Original Course View jest nadal obsługiwany w Learn SaaS i pozostaje głównym doświadczeniem dla samodzielnie hostowanych witryn Learn 9.1 na linii wydań Q4 2024 CU.

1. Otwórz kurs i wejdź do **Obszaru z zawartością** (na przykład domyślny obszar **Informacje** lub **Zawartość** w menu kursu).
2. Włącz **Tryb edycji** przełącznikiem w prawym górnym rogu strony.
3. Kliknij **Utwórz zawartość** w pasku akcji.
4. W podmenu **Narzędzia do nauki** kliknij **FastComments**. Podmenu Narzędzia do nauki jest zasilane z umiejscowień narzędzi LTI 1.3 po tym, jak administrator zarejestruje narzędzie. Jeśli go nie widzisz, zobacz sekcję z problemami poniżej.
5. Na formularzu **Utwórz FastComments** ustaw:
   - **Nazwa**: etykieta widoczna dla studentów w obszarze z zawartością.
   - **Opis**: opcjonalny tekst wyświetlany nad osadzonym wątkiem.
   - **Zezwalaj użytkownikom na wyświetlanie tej zawartości**: przełącznik dostępności Tak/Nie.
   - **Śledź liczbę wyświetleń**: włącz, jeśli chcesz statystyki wyświetleń elementu z Blackboard. FastComments prowadzi własną analitykę niezależnie.
   - **Ograniczenia daty i czasu**: opcjonalne okna **Pokaż po** / **Pokaż do**.
6. Prześlij formularz. Narzędzie pojawia się jako klikalny element w obszarze z zawartością.

#### Osadzanie wewnątrz elementu lub dokumentu

W obu widokach kursu instruktorzy osadzają FastComments inline wewnątrz treści elementu, dokumentu lub dowolnego pola rich-text za pomocą przycisku LTI Advantage w edytorze zawartości.

Ultra Course View:

1. Utwórz lub edytuj **Dokument**.
2. Kliknij **Dodaj zawartość** wewnątrz ciała dokumentu w miejscu, w którym chcesz, by pojawił się wątek.
3. W pasku narzędzi edytora otwórz menu **Wstaw zawartość** i kliknij **Content Market** (punkt wejścia LTI Advantage / Deep Linking).
4. Wybierz **FastComments**. FastComments zwraca ładunek deep-link, a Blackboard wstawia osadzony blok w ciele dokumentu w pozycji kursora.
5. Zapisz dokument. Studenci widzą wątek wyrenderowany inline, gdy przewijają stronę.

Original Course View:

1. Edytuj dowolny element z polem rich-text.
2. W pasku narzędzi edytora zawartości kliknij ikonę plus **Dodaj zawartość** i wybierz **Content Market** (oznaczone jako **Dodaj zawartość z narzędzia zewnętrznego** w starszych Q4 2024 CU).
3. Wybierz **FastComments**. Edytor wstawia blok zastępczy odnoszący się do zasobu deep-link.
4. Prześlij element.

Każde osadzenie deep-link tworzy własny wątek FastComments, więc element z dwoma osadzonymi blokami FastComments ma dwa niezależne strumienie komentarzy.

#### Widoczność, warunki udostępniania i ograniczenia grupowe

Elementy zawartości FastComments zachowują się jak każdy inny element zawartości Blackboard pod względem reguł kontroli dostępu na nich nałożonych.

- Ultra: kliknij selektor widoczności na wierszu (**Widoczne dla studentów**, **Ukryte przed studentami**, **Dostęp warunkowy**). Dostęp warunkowy obsługuje okna dat/czasu, reguły wydajności względem elementów dziennika ocen oraz reguły członkostwa względem grup kursu.
- Original: otwórz menu kontekstowe elementu i wybierz **Adaptive Release** lub **Adaptive Release: Advanced**, aby ograniczyć narzędzie według daty, członkostwa, oceny lub statusu przeglądu. Użyj **Ustaw dostępność dla grup** na elemencie, aby ograniczyć go do konkretnych grup kursu.

FastComments respektuje dowolną regułę bramy Blackboard. Jeśli Blackboard ukryje element przed studentem, uruchomienie LTI nigdy nie nastąpi dla tego studenta i nie pojawi się on w widoku moderatora.

#### Zachowanie względem dziennika ocen

FastComments nie przesyła ocen z powrotem przez LTI Advantage Assignment and Grade Services. Żadna kolumna ocen nie jest automatycznie tworzona dla elementów zawartości FastComments.

Jeśli Twój tenant Blackboard jest skonfigurowany tak, by automatycznie tworzyć kolumnę dziennika ocen dla każdego nowego elementu zawartości niezależnie od metadanych oceniania, pojawi się pusta kolumna. Aby ją ukryć:

- Ultra: otwórz **Dziennik ocen**, kliknij nagłówek kolumny, wybierz **Edytuj** i wyłącz **Pokaż studentom** oraz **Uwzględnij w obliczeniach**. Alternatywnie użyj **Usuń**, jeśli Twoja instytucja zezwala na usuwanie kolumn dla elementów nieocenianych.
- Original: otwórz **Centrum ocen**, kliknij strzałkę przy kolumnie, wybierz **Ukryj przed użytkownikami (włącz/wyłącz)** i opcjonalnie **Ukryj w widoku instruktora** w sekcji **Organizacja kolumn**.

#### Co widzą studenci

Gdy student otworzy element FastComments lub przewinie do osadzonego bloku:

1. Blackboard uruchamia komunikat LTI 1.3 do FastComments. Student zostaje zalogowany przez SSO przy użyciu jego tożsamości Blackboard (imię i nazwisko, e-mail, awatar, rola) bez konieczności wypełniania formularza logowania.
2. Wątek komentarzy renderuje się w iframe. Wątkowanie, odpowiedzi, wzmianki i reakcje są dostępne zgodnie z ustawieniami widgetu komentarzy skonfigurowanymi w FastComments.
3. Ich komentarze są przypisane do konta Blackboard. Jeśli student później edytuje swoje imię lub zdjęcie w Blackboard, przy następnym uruchomieniu profil FastComments zostanie zaktualizowany.

Mapowanie ról z Blackboard do FastComments:

- **System Administrator** i **Course Builder** mapują się na FastComments **admin**.
- **Instructor** i **Teaching Assistant** mapują się na FastComments **moderator**.
- **Student**, **Guest** i **Observer** mapują się na FastComments **commenter**.

Moderatorzy widzą narzędzia moderacyjne (przypięcie, ukrycie, zbanowanie, usunięcie) inline przy każdym komentarzu w wątku.

#### Zakresowanie wątków

FastComments zakresuje każdy wątek według **(host Blackboard, ID kursu, resource link ID)**. Dwa elementy FastComments w tym samym kursie tworzą dwa wątki. Ten sam element skopiowany do dwóch powłok kursu (na przykład przez kopiowanie kursu) tworzy dwa wątki, ponieważ Blackboard nadaje nowy resource link ID podczas kopiowania. Aby zachować współdzielony wątek między kopiami kursu, użyj Deep Linking z eksplicytnym URN wątku skonfigurowanym w FastComments przed uruchomieniem kopiowania.

#### Specyficzne problemy związane z Blackboard

**Brak kafelka FastComments w menu Utwórz zawartość (Original) lub Content Market (Ultra).** Administrator zatwierdził narzędzie, ale pozostawił politykę instytucji blokującą odpowiednie umiejscowienie. Przejdź do **Panel administratora** > **Integracje** > **Dostawcy narzędzi LTI**, edytuj wpis FastComments i potwierdź, że zarówno **Course Content Tool** (Original), jak i **Course Content Tool - allow students** / **Deep Linking content tool** (Ultra) umiejscowienia są włączone. Zapisz i odśwież stronę kursu.

**Błąd „Tool not configured for this context” lub „Tool is not deployed” przy uruchomieniu.** Zakres wdrożenia zarejestrowany podczas rejestracji dynamicznej nie zgadza się z kontekstem instytucji, do którego należy kurs. W wpisie dostawcy narzędzia w Blackboard sprawdź, czy **Deployment ID** zgadza się z tym, co FastComments pokazuje na stronie konfiguracji LTI 1.3 dla tego tenanta. Jeśli się różnią, usuń umiejscowienie i ponownie uruchom rejestrację dynamiczną z nowego URL rejestracji.

**Wysokość iframe wygląda na stałą lub zawartość jest obcinana.** Niektóre tenany Blackboard mają restrykcyjną politykę Content Security Policy, która blokuje domyślną zmianę rozmiaru iframe LTI za pomocą postMessage. FastComments emituje zarówno wiadomość w stylu Canvas `lti.frameResize`, jak i zgodną z IMS spec `org.imsglobal.lti.frameResize`, aby zmaksymalizować kompatybilność, ale nadpisanie CSP na poziomie tenanta blokuje nasłuch rodzica. Poproś administratora, aby potwierdził, że `*.fastcomments.com` znajduje się na liście dozwolonych narzędzi LTI i że żaden niestandardowy nagłówek CSP nie usuwa zdarzeń postMessage. Po tym zmiana rozmiaru działa bez dalszej konfiguracji.

**Kopiowanie kursu duplikuje wątki.** Kopiowanie kursu w Blackboard nadaje nowe resource link ID dla umiejscowień LTI, więc skopiowane kursy zaczynają od pustych wątków. To zachowanie jest oczekiwane. Jeśli potrzebujesz, aby skopiowany kurs odziedziczył oryginalny wątek, skonfiguruj Deep Linking z eksplicytnym URN wątku przed kopiowaniem lub skontaktuj się z pomocą FastComments w celu masowego remapowania ID wątków.

**Student widzi ogólny błąd Blackboard przy uruchomieniu.** Przyczyną jest brakujący lub przestarzały claim `email`. Potwierdź, że polityka instytucji dla FastComments ma włączone **Rola**, **Imię i nazwisko** oraz **Adres e-mail** w sekcji **Pola użytkownika do wysłania**. Zapisz, a następnie uruchom ponownie w nowej sesji przeglądarki.
Po zarejestrowaniu FastComments na platformie, instruktorzy dodają go do zawartości kursu, używając standardowych przepływów dodawania narzędzi zewnętrznych platformy. Ta strona opisuje Sakai 23.x i Schoology Enterprise.

#### Zablokuj dostęp publiczny (zalecane)

Domyślnie dane komentarzy FastComments są na obu platformach publicznie czytelne. Każdy, kto potrafi odgadnąć URL wątku lub punkt końcowy API, może zobaczyć komentarze, nawet poza Sakai lub Schoology. W przypadku dyskusji kursowych niemal na pewno chcesz ograniczyć podgląd tylko do zapisanych studentów.

Otwórz swoją <a href="https://fastcomments.com/auth/my-account/customize-widget" target="_blank">stronę dostosowywania widgetu</a> i utwórz regułę z włączoną opcją **Require SSO To View Comments**, następnie ustaw poziom bezpieczeństwa na **Secure SSO**, aby wątki mogły być ładowane tylko przez podpisane uruchomienie LTI.

Zobacz [Protecting Comment Threads With Single-Sign-On](/guide-customizations-and-configuration.html#sso-require-to-view-comments) po pełny przewodnik krok po kroku, w tym jak ograniczyć zakres reguły do konkretnej domeny lub strony.

#### Sakai

**1. Dodaj FastComments do witryny**

Opiekun witryny włącza narzędzie dla poszczególnych witryn:

1. Otwórz witrynę i kliknij **Site Info** w lewym panelu nawigacyjnym.
2. Kliknij **Manage Tools**.
3. Przewiń do listy **External Tools** i włącz **FastComments**.
4. Kliknij **Continue**, przejrzyj listę narzędzi, a następnie kliknij **Finish**.

FastComments pojawi się teraz jako pozycja w lewym panelu nawigacyjnym witryny.

**2. Zmień kolejność pozycji w lewym panelu**

Przejdź do **Site Info** > **Tool Order**. Przeciągnij **FastComments** na żądaną pozycję i kliknij **Save**. Z tego ekranu możesz też zmienić etykietę nawigacji i ukryć ją przed studentami.

**3. Osadź inline na stronie Lessons**

Aby umieścić FastComments bezpośrednio wewnątrz strony Lessons zamiast jako samodzielne narzędzie w lewym panelu:

1. Otwórz narzędzie **Lessons** w witrynie.
2. Kliknij **Add Content** > **Add External Tool**.
3. Wybierz **FastComments** z listy.
4. Jeśli FastComments zadeklarował Deep Linking podczas rejestracji, Sakai otworzy selektor zawartości narzędzia, abyś mógł wybrać lub oznaczyć wątek. Jeśli Deep Linking nie zostało zadeklarowane, Sakai wstawi domyślne łącze uruchamiające.
5. Zapisz element Lessons.

Każda osadzona instancja otrzymuje własny wątek, powiązany z tym linkiem zasobu.

**4. Dostosowanie uprawnień dla dostępu studentów**

Sakai kontroluje uruchamianie narzędzi zewnętrznych przez Realms. Aby potwierdzić, że studenci mogą uruchamiać FastComments:

1. Zaloguj się jako administrator Sakai i otwórz **Administration Workspace** > **Realms**.
2. Otwórz odpowiedni realm (na przykład `!site.template.course` lub konkretny realm witryny).
3. Potwierdź, że rola `access` ma włączone `lti.launch` i że uprawnienia roli w grupie **external.tools** zostały przyznane.
4. Zapisz realm.

Dla nadpisania na poziomie witryny, opiekun może dostosować widoczność narzędzia dla poszczególnych ról z poziomu **Site Info** > **Tool Order**, ukrywając lub pokazując FastComments dla danej roli.

**5. Co widzą studenci**

Studenci klikają pozycję FastComments w lewym panelu nawigacyjnym (lub przewijają do osadzonego bloku Lessons) i trafiają bezpośrednio do widoku wątków komentarzy. SSO działa automatycznie: Sakai wysyła tożsamość użytkownika w uruchomieniu LTI, a FastComments loguje go pod kontem Sakai.

Mapowanie ról:

- Sakai `Instructor` -> FastComments moderator
- Sakai `Admin` (admin w Administration Workspace) -> FastComments admin
- Sakai `Student` / `access` -> FastComments commenter

**6. Uwagi dotyczące Sakai**

- **Narzędzie niewidoczne w Manage Tools.** Jeśli FastComments nie pojawia się na liście External Tools, administrator Sakai musi otworzyć rejestr narzędzi (**Administration Workspace** > **External Tools** > **FastComments**) i ustawić **Stealthed** na `false`. Narzędzia stealthed są ukryte przed selektorem Manage Tools dla witryn.
- **Błędy uruchamiania w przeglądarkach z współdzieloną sesją.** Token CSRF portalu Sakai jest powiązany z sesją przeglądarki. Jeśli student jest zalogowany do dwóch witryn Sakai w różnych kartach lub ma przestarzałą sesję, uruchomienie zwraca 403. Naprawa: zamknij inne karty Sakai, wyloguj się, zaloguj ponownie i uruchom ponownie. Administratorzy mogą też podwyższyć `sakai.csrf.token.cache.ttl`, jeśli problem pojawia się w całym klastrze.
- **Osadzanie w ramce.** Potwierdź, że `lti.frameheight` w `sakai.properties` jest wystarczająco duże (600 lub więcej), aby wątek komentarzy nie był przycinany wewnątrz strony Lessons.

#### Schoology

Schoology Enterprise ma dwa scenariusze instalacji. Potwierdź, który z nich ma zastosowanie przed dodaniem narzędzia do kursu.

**1. Dwa scenariusze instalacji**

- **(a) Instalacja na poziomie przedsiębiorstwa.** Administrator systemu Schoology zainstalował FastComments na poziomie organizacji i przypisał go do wszystkich kursów lub do konkretnych szablonów kursów. Instruktorzy pomijają instalację i przechodzą bezpośrednio do "Add Materials".
- **(b) Samodzielna instalacja przez instruktora.** Instruktor instaluje narzędzie w pojedynczym kursie z **Course Options** > **External Tools** > **Install LTI Apps**. Samodzielna instalacja wymaga uprzedniej akceptacji aplikacji FastComments na poziomie organizacji przez Administratora systemu.

**2. Dodaj FastComments jako materiał kursowy**

W obrębie kursu:

1. Otwórz kurs i przejdź do **Materials**.
2. Kliknij **Add Materials** > **Add File/Link/External Tool**.
3. Wybierz **External Tool**.
4. Wybierz **FastComments** z listy zarejestrowanych narzędzi.
5. Ustaw **Name** (to widzą studenci na liście materiałów) i opcjonalnie **Description**.
6. Pozostaw **Enable Grading** (grade passback) **OFF**. FastComments nie przesyła ocen z powrotem do Schoology, więc włączenie przekazywania ocen tworzy pustą kolumnę w dzienniku ocen.
7. Kliknij **Submit**.

Materiał pojawi się teraz na liście materiałów kursu i otworzy wątek FastComments po kliknięciu.

**3. Osadzanie inline przez edytor Rich Text**

Jeśli Administrator systemu włączył Deep Linking placement dla FastComments podczas rejestracji, instruktorzy mogą osadzić wątek komentarzy w dowolnym polu Rich Text (instrukcje zadania, treści stron, polecenia dyskusji):

1. Otwórz edytor Rich Text na docelowej stronie.
2. Kliknij ikonę **External Tool** (symbol puzzla) na pasku narzędzi.
3. Wybierz **FastComments**.
4. Skonfiguruj osadzenie w dialogu deep-linking i kliknij **Insert**.
5. Zapisz stronę.

Jeśli przycisk External Tool nie pojawia się w edytorze Rich Text, Deep Linking jest wyłączone dla tego narzędzia w tej instancji. Zobacz uwagi poniżej.

**4. Widoczność i przypisywanie do sekcji**

Schoology ogranicza dostępność narzędzia per sekcja poprzez Course Options:

1. Z kursu kliknij **Course Options** > **External Tools**.
2. Dla każdej zainstalowanej aplikacji LTI kontrolujesz, czy jest dostępna dla wszystkich sekcji w kursie, czy tylko dla wybranych sekcji.
3. Aby ograniczyć FastComments do określonych sekcji, odznacz sekcje, które nie powinny widzieć narzędzia.
4. Dostęp na poziomie sekcji także decyduje, które sekcje zobaczą wpis **Add Materials** > **External Tool** dla FastComments.

**5. Co widzą studenci**

Studenci klikają materiał FastComments (lub przewijają do osadzenia inline) i trafiają do dyskusji w wątkach. SSO działa automatycznie przez uruchomienie LTI Schoology pod ich kontem Schoology.

Mapowanie ról:

- Schoology `Administrator` -> FastComments admin
- Schoology `Instructor` -> FastComments moderator
- Schoology `Student` -> FastComments commenter

**6. Uwagi dotyczące Schoology**

- **Tylko Enterprise.** Konta osobiste i darmowe w Schoology nie mogą instalować narzędzi LTI 1.3. Jeśli Twój tenant korzysta z darmowego planu, opcja **External Tools** nie pojawia się w Course Options. Uaktualnij do Schoology Enterprise, aby używać FastComments.
- **Deep Linking wyłączone domyślnie przez tenant.** Niektóre tenany Schoology ograniczają placement Deep Linking na poziomie organizacji. W takim przypadku instruktorzy widzą tylko przepływ **Add Materials** > **External Tool**, a nie przycisk External Tool w edytorze Rich Text. Aby włączyć osadzanie inline, Administrator systemu przechodzi do **System Settings** > **Integration** > **LTI 1.3** > **FastComments** i włącza placement **Content Item / Deep Linking**, a następnie zapisuje.
- **Nadpisanie przypisania na poziomie sekcji.** Jeśli FastComments jest przypisany na poziomie przedsiębiorstwa, ale instruktor nie widzi go w **Add Materials**, sekcja kursu została wykluczona w przypisaniu na poziomie organizacji. Poproś Administratora systemu o dodanie sekcji do przypisania aplikacji FastComments.
- **Nazwa materiału vs. tożsamość wątku.** Zmiana nazwy materiału w Schoology nie przenosi wątku komentarzy. Wątki są identyfikowane na podstawie LTI resource link ID, więc zmiana nazwy zachowuje ten sam wątek; usunięcie i ponowne utworzenie materiału tworzy nowy, pusty wątek.
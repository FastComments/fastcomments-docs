Po zarejestrowaniu FastComments w platformie, instruktorzy dodają je do treści kursu za pomocą standardowych przepływów narzędzia zewnętrznego platformy. Ta strona obejmuje Sakai 23.x i Schoology Enterprise.

#### Sakai

**1. Dodaj FastComments do witryny**

Opiekun witryny włącza narzędzie dla każdej witryny osobno:

1. Otwórz witrynę i kliknij **Site Info** w lewym menu.
2. Kliknij **Manage Tools**.
3. Przewiń do listy **External Tools** i włącz **FastComments**.
4. Kliknij **Continue**, przejrzyj listę narzędzi, a następnie kliknij **Finish**.

FastComments pojawi się teraz jako element w lewym menu witryny.

**2. Zmień kolejność wpisu w lewym menu**

Przejdź do **Site Info** > **Tool Order**. Przeciągnij **FastComments** na wybrane miejsce i kliknij **Save**. Z tego ekranu możesz także zmienić etykietę nawigacji i ukryć ją przed studentami.

**3. Osadź inline na stronie Lekcji**

Aby umieścić FastComments bezpośrednio w stronie Lessons zamiast jako samodzielne narzędzie w lewej nawigacji:

1. Otwórz narzędzie **Lessons** w witrynie.
2. Kliknij **Add Content** > **Add External Tool**.
3. Wybierz **FastComments** z listy.
4. Jeśli FastComments zadeklarowało Deep Linking podczas rejestracji, Sakai otworzy selektor treści narzędzia, abyś mógł wybrać lub oznaczyć wątek. Jeśli Deep Linking nie zostało zadeklarowane, Sakai wstawi domyślny link uruchamiający.
5. Zapisz element Lessons.

Każda osadzona instancja otrzymuje własny wątek, przypisany do tej konkretnej linki zasobu.

**4. Dostosowanie uprawnień dla dostępu studentów**

Sakai kontroluje uruchamianie narzędzi zewnętrznych przez Realms. Aby potwierdzić, że studenci mogą uruchamiać FastComments:

1. Zaloguj się jako administrator Sakai i otwórz **Administration Workspace** > **Realms**.
2. Otwórz odpowiedni realm (na przykład `!site.template.course` lub konkretny realm witryny).
3. Potwierdź, że rola `access` ma włączone `lti.launch` oraz że uprawnienia ról w grupie **external.tools** zostały przyznane.
4. Zapisz realm.

Dla nadpisania na poziomie witryny, opiekun może dostosować widoczność narzędzia dla poszczególnych ról z **Site Info** > **Tool Order**, ukrywając lub pokazując FastComments dla danej roli.

**5. Co widzą studenci**

Studenci klikają element FastComments w lewym menu (lub przewijają do osadzonego bloku w Lessons) i trafiają bezpośrednio do widoku wątków z komentarzami. SSO jest automatyczne: Sakai wysyła tożsamość użytkownika w uruchomieniu LTI, a FastComments loguje go na konto Sakai.

Mapowanie ról:

- Sakai `Instructor` -> FastComments moderator
- Sakai `Admin` (admin in Administration Workspace) -> FastComments administrator
- Sakai `Student` / `access` -> FastComments komentujący

**6. Pułapki Sakai**

- **Narzędzie niewidoczne w Manage Tools.** Jeśli FastComments nie pojawia się na liście External Tools, administrator Sakai musi otworzyć rejestr narzędzi (**Administration Workspace** > **External Tools** > **FastComments**) i ustawić **Stealthed** na `false`. Narzędzia oznaczone jako stealthed są ukryte w selektorze Manage Tools dla poszczególnych witryn.
- **Błędy uruchamiania w przeglądarkach z dzieloną sesją.** Token CSRF portalu Sakai jest powiązany z sesją przeglądarki. Jeśli student jest zalogowany do dwóch witryn Sakai w różnych kartach lub ma przeterminowaną sesję, uruchomienie zwraca 403. Rozwiązanie: zamknij inne karty Sakai, wyloguj się, zaloguj ponownie i uruchom ponownie. Administratorzy mogą także zwiększyć `sakai.csrf.token.cache.ttl`, jeśli problem występuje w całym klastrze.
- **Osadzanie w ramce.** Potwierdź, że `lti.frameheight` w `sakai.properties` jest wystarczająco duże (600 lub więcej), aby wątek komentarzy nie był obcięty wewnątrz strony Lessons.

#### Schoology

Schoology Enterprise ma dwa scenariusze instalacji. Potwierdź, który dotyczy Twojej organizacji przed dodaniem narzędzia do kursu.

**1. Dwa scenariusze instalacji**

- **(a) Instalacja na poziomie Enterprise.** Administrator systemu Schoology zainstalował FastComments na poziomie organizacji i przypisał je do wszystkich kursów lub do określonych szablonów kursów. Instruktorzy pomijają instalację i przechodzą bezpośrednio do „Add Materials”.
- **(b) Samoinstalacja przez instruktora.** Instruktor instaluje narzędzie do pojedynczego kursu z **Course Options** > **External Tools** > **Install LTI Apps**. Samoinstalacja wymaga uprzedniej akceptacji aplikacji FastComments na poziomie organizacji przez Administratora Systemu.

**2. Dodaj FastComments jako materiał kursu**

W obrębie kursu:

1. Otwórz kurs i przejdź do **Materials**.
2. Kliknij **Add Materials** > **Add File/Link/External Tool**.
3. Wybierz **External Tool**.
4. Wybierz **FastComments** z zarejestrowanej listy narzędzi.
5. Ustaw **Name** (to jest to, co widzą studenci na liście materiałów) i opcjonalnie **Description**.
6. Pozostaw **Enable Grading** (przekazywanie ocen) **WYŁĄCZONE**. FastComments nie raportuje ocen z powrotem do Schoology, więc włączenie przekazywania ocen tworzy pustą kolumnę w dzienniku ocen.
7. Kliknij **Submit**.

Materiał pojawi się teraz na liście materiałów kursu i otworzy wątek FastComments po kliknięciu.

**3. Osadzanie inline przez edytor Rich Text**

Jeśli Administrator Systemu włączył Deep Linking placement dla FastComments podczas rejestracji, instruktorzy mogą osadzić wątek komentarzy w dowolnym polu Rich Text (instrukcje zadania, treści strony, zaproszenia do dyskusji):

1. Otwórz edytor Rich Text na docelowej stronie.
2. Kliknij ikonę **External Tool** (ikona puzzla) na pasku narzędzi.
3. Wybierz **FastComments**.
4. Skonfiguruj osadzenie w oknie deep-linking i kliknij **Insert**.
5. Zapisz stronę.

Jeśli przycisk External Tool nie pojawia się w edytorze Rich Text, Deep Linking jest wyłączone dla tego narzędzia na tym tenantcie. Zobacz pułapki poniżej.

**4. Widoczność i przypisania do sekcji**

Schoology określa dostępność narzędzia per sekcja za pomocą Course Options:

1. W kursie kliknij **Course Options** > **External Tools**.
2. Dla każdej zainstalowanej aplikacji LTI możesz kontrolować, czy jest dostępna dla wszystkich sekcji w kursie, czy tylko dla wybranych sekcji.
3. Aby ograniczyć FastComments do niektórych sekcji, odznacz sekcje, które nie powinny widzieć narzędzia.
4. Dostęp na poziomie sekcji również decyduje, które sekcje widzą opcję **Add Materials** > **External Tool** dla FastComments.

**5. Co widzą studenci**

Studenci klikają materiał FastComments (lub przewijają do osadzenia inline) i trafiają do dyskusji w wątkach. SSO jest automatyczne za pośrednictwem uruchomienia LTI pod ich kontem Schoology.

Mapowanie ról:

- Schoology `Administrator` -> FastComments administrator
- Schoology `Instructor` -> FastComments moderator
- Schoology `Student` -> FastComments komentujący

**6. Pułapki Schoology**

- **Tylko Enterprise.** Konta osobiste i darmowe konta Schoology nie mogą instalować narzędzi LTI 1.3. Jeśli Twój tenant jest na darmowym planie, opcja **External Tools** nie pojawia się w Course Options. Uaktualnij do Schoology Enterprise, aby używać FastComments.
- **Deep Linking wyłączone domyślnie przez tenant.** Niektóre tenancy Schoology ograniczają umieszczanie przez Deep Linking na poziomie organizacji. W takim przypadku instruktorzy widzą tylko przepływ **Add Materials** > **External Tool**, a nie przycisk External Tool w edytorze Rich Text. Aby włączyć osadzanie inline, Administrator Systemu przechodzi do **System Settings** > **Integration** > **LTI 1.3** > **FastComments** i włącza umiejscowienie **Content Item / Deep Linking**, a następnie zapisuje.
- **Nadpisanie przypisania na poziomie sekcji.** Jeśli FastComments zostało przypisane na poziomie enterprise, ale instruktor nie widzi go w **Add Materials**, sekcja kursu została wykluczona w przypisaniu na poziomie organizacji. Poproś Administratora Systemu o dodanie sekcji do przypisania aplikacji FastComments.
- **Nazwa materiału a tożsamość wątku.** Zmiana nazwy materiału w Schoology nie przenosi wątku komentarzy. Wątki są identyfikowane przez LTI resource link ID, więc zmiana nazwy zachowuje ten sam wątek; usunięcie i ponowne utworzenie materiału tworzy nowy, pusty wątek.
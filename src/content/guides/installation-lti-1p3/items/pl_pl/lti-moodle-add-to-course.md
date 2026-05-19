Ten przewodnik opisuje dodawanie FastComments do kursu Moodle 4.x po tym, jak administrator witryny zarejestrował narzędzie i ustawił je tak, aby było widoczne w wyborze aktywności. Jeśli FastComments nie jest jeszcze zarejestrowany, najpierw zobacz przewodnik rejestracji Moodle.

#### Otwórz kurs w trybie edycji

1. Zaloguj się do Moodle jako Editing Teacher (lub wyżej) dla danego kursu.
2. Otwórz kurs.
3. Włącz przełącznik **Edit mode** w prawym górnym rogu nagłówka kursu.

Moodle 4.x zastąpił stare rozwijane menu "Add an activity or resource" z wersji 3.x dialogiem wyboru aktywności na pełnym ekranie. Moodle 4.5 zachowuje ten sam wybór, ale dodaje wiersz z ulubionymi/oznaczonymi gwiazdką na górze, więc przypięcie FastComments raz ułatwia jego znalezienie w kolejnych sekcjach.

#### Dodaj aktywność FastComments

1. Przewiń do sekcji kursu (temat lub tydzień), w której powinna się znajdować dyskusja.
2. Kliknij **Add an activity or resource** na dole tej sekcji.
3. W oknie wyboru wybierz **FastComments**. Jeśli go nie widzisz, przejdź do sekcji o problemach poniżej.

Otworzy się formularz ustawień aktywności. Pola, które mają znaczenie:

- **Activity name** (wymagane). Wyświetlane na stronie kursu i w dzienniku ocen. Przykład: `Week 3 Discussion`.
- **Activity description**. Opcjonalny tekst wprowadzający renderowany nad wątkiem komentarzy.
- **Show description on course page**. Zaznacz to, jeśli chcesz, aby opis był widoczny bez konieczności wchodzenia do aktywności.
- **Preconfigured tool**. Ustawione na `FastComments` (automatycznie wybrane po uruchomieniu z wybieraka). Nie zmieniaj.
- **Launch container**. Ustaw na **New window**. Zobacz sekcję o problemach, dlaczego opcja "Same window" może powodować błędy w niektórych wdrożeniach Moodle.
- **Tool URL**, **Public key**, **Shared secret**, **Custom parameters**. Pozostaw puste. Dynamic Registration obsłużyło to na poziomie witryny.

Przewiń na dół i kliknij **Save and return to course** (lub **Save and display**, aby od razu otworzyć aktywność).

Aktywność pojawi się jako wiersz w sekcji z ikoną FastComments. Studenci klikają wiersz, aby otworzyć wątek komentarzy.

#### Osadź FastComments inline w edytorze

Dla wątku wewnątrz Page, rozdziału Book, Lesson lub dowolnego innego zasobu używającego edytora Atto lub TinyMCE:

1. Otwórz zasób w trybie edycji.
2. Umieść kursor w miejscu, gdzie powinien pojawić się wątek.
3. W pasku narzędzi edytora kliknij przycisk **LTI** / **External tool**. W Atto jest oznaczony jako "Insert LTI Advantage content". W TinyMCE (domyślnie w Moodle 4.3+) znajduje się w menu **More** jako **External tools**.
4. Wybierz **FastComments** z listy narzędzi.
5. FastComments otworzy wybierak deep-linking. Potwierdź tytuł wątku i kliknij **Embed**.
6. Edytor wstawia blok zastępczy LTI. Zapisz zasób.

Każda osadzona instancja to odrębny wątek identyfikowany na podstawie ID elementu deep-link, więc strona Page z trzema osadzeniami FastComments będzie miała trzy niezależne wątki.

#### Ograniczenia dostępu i ustawienia grup

Standardowe ustawienia aktywności Moodle mają zastosowanie do aktywności FastComments:

- **Common module settings** > **Group mode**. Ustawienie na **Separate groups** lub **Visible groups** nie dzieli automatycznie FastComments na wątki per grupa. Tryb grup w Moodle jedynie filtruje dziennik ocen i listę członków. Aby uruchomić oddzielny wątek dla każdej grupy, dodaj po jednej aktywności FastComments dla każdej grupy i użyj **Restrict access**, aby ograniczyć zasięg każdej z nich.
- **Restrict access** > **Add restriction**. Obsługuje standardowe warunki Moodle: **Date**, **Grade**, **Group**, **Grouping**, **User profile** oraz zagnieżdżone zestawy ograniczeń. Użyj **Group**, aby przypisać aktywność FastComments do jednej grupy.
- **Activity completion**. Ustaw na **Students must view this activity to complete it**, jeśli chcesz śledzić ukończenie. FastComments obecnie nie zgłasza do Moodle zdarzenia ukończenia poza uruchomieniem.

#### Mapowanie ról

FastComments odczytuje roszczenie `roles` LTI, które Moodle wysyła przy każdym uruchomieniu i mapuje je w następujący sposób:

- Moodle **Manager** lub **Site administrator** -> FastComments **admin**
- Moodle **Editing teacher** lub **Non-editing teacher** -> FastComments **moderator**
- Moodle **Student** -> FastComments **commenter**
- Moodle **Guest** -> tylko do odczytu

Administratorzy mogą usuwać dowolne komentarze, blokować użytkowników i edytować ustawienia wątku. Moderatorzy mogą usuwać i zatwierdzać komentarze wewnątrz wątku, do którego zostali uruchomieni. Niestandardowe role Moodle dziedziczą mapowanie archetypu, od którego zostały sklonowane.

#### Co widzą studenci

Studenci klikają aktywność FastComments (lub przewijają do osadzonego bloku wewnątrz Page lub Book). Moodle wysyła ich tożsamość do FastComments za pomocą uruchomienia LTI:

- Brak ekranu logowania. FastComments loguje ich przy użyciu konta Moodle.
- Ich nazwa wyświetlana, e-mail i avatar pochodzą z Moodle.
- Wątek jest ograniczony do (Moodle site, course, resource link ID), więc ta sama aktywność zduplikowana w innym kursie otrzymuje nowy wątek.
- Odpowiedzi w wątkach, głosowanie i powiadomienia działają tak samo, jak w samodzielnym wątku FastComments.

#### Zablokuj publiczny dostęp (zalecane)

Domyślnie dane komentarzy FastComments są publicznie czytelne. Każdy, kto potrafi odgadnąć URL wątku lub punkt końcowy API, może przeglądać jego komentarze, nawet poza Moodle. Dla dyskusji kursowych prawie na pewno chcesz ograniczyć podgląd tylko do zapisanych studentów.

Otwórz swoją <a href="https://fastcomments.com/auth/my-account/customize-widget" target="_blank">stronę dostosowywania widgetu</a> i utwórz regułę z włączoną opcją **Require SSO To View Comments**, a następnie ustaw poziom bezpieczeństwa na **Secure SSO**, aby wątki mogły być ładowane tylko przez podpisane uruchomienie LTI.

Zobacz [Protecting Comment Threads With Single-Sign-On](/guide-customizations-and-configuration.html#sso-require-to-view-comments) po pełny przewodnik krok po kroku, w tym jak ograniczyć regułę do pojedynczej domeny lub strony.

#### Problemy Moodle

**Brak FastComments w wyborze aktywności.** Administrator witryny zarejestrował narzędzie, ale nie ustawił **Tool configuration usage** na **Show in activity chooser and as a preconfigured tool**. Napraw to w **Site administration** > **Plugins** > **Activity modules** > **External tool** > **Manage tools** > ikona koła zębatego na kafelku FastComments.

**Uruchomienie kończy się niepowodzeniem lub pokazuje puste okno, gdy ustawione na "Same window".** Ciasteczka sesyjne Moodle używają domyślnie `SameSite=Lax`, a niektóre przeglądarki usuwają je podczas cross-site POST, którego używa LTI 1.3, aby wrócić z FastComments. Ustaw **Launch container** na **New window** przy aktywności. To twardy wymóg dla osadzonego FastComments wewnątrz Page lub Book, ponieważ ścieżka uruchamiania osadzona przez edytor zawsze otwiera nowe okno.

**Roszczenie `iss` to URL strony Moodle, a nie identyfikator tenanta.** FastComments używa URL strony Moodle (wartość konfiguracyjna `wwwroot`) jako wydawcy LTI. Jeśli Twoja instancja Moodle zostanie przeniesiona na nową domenę lub zmienisz `wwwroot`, istniejące wątki FastComments pozostaną powiązane ze starym wydawcą i nie będą pasować do nowych uruchomień. Zarejestruj narzędzie ponownie dla nowego URL i w razie potrzeby przenieś wątki przez panel administracyjny FastComments.

**Kopia zapasowa i przywracanie aktywności.** Wykonanie kopii zapasowej kursu i przywrócenie go do nowego kursu tworzy nowe ID resource link, więc przywrócone aktywności FastComments zaczynają z pustymi wątkami. Oryginalny kurs zachowuje oryginalne wątki. To zamierzone zachowanie, a nie błąd.

**TinyMCE jako domyślny w Moodle 4.5.** Moodle 4.5 jest dostarczany z TinyMCE jako domyślnym edytorem dla nowych instalacji. Przycisk External tool znajduje się w menu **More** (`...`) zamiast na głównym pasku narzędzi. Starsze witryny zaktualizowane z 4.1 zachowują Atto, chyba że administrator zmienił domyślny edytor.
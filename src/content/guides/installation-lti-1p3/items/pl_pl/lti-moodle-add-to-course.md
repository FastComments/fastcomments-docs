Ten przewodnik opisuje dodawanie FastComments do kursu Moodle 4.x po tym, jak administrator witryny zarejestrował narzędzie i ustawił je tak, aby było widoczne w wyborze aktywności. Jeśli FastComments nie jest jeszcze zarejestrowany, najpierw zobacz przewodnik rejestracji Moodle.

#### Otwórz kurs w trybie edycji

1. Zaloguj się do Moodle jako Nauczyciel z uprawnieniami do edycji (lub wyżej) dla danego kursu.
2. Otwórz kurs.
3. Włącz **Tryb edycji**, używając przełącznika w prawym górnym rogu nagłówka kursu.

Moodle 4.x zastąpił starszy rozwijany przycisk „Add an activity or resource”, używany w wersji 3.x, pełnoekranowym oknem wyboru aktywności. Moodle 4.5 zachowuje ten sam wybór, ale dodaje wiersz z ulubionymi na górze, więc przypięcie FastComments sprawia, że później łatwiej go znaleźć w kolejnych sekcjach.

#### Dodaj aktywność FastComments

1. Przewiń do sekcji kursu (tematu lub tygodnia), do której należy dyskusja.
2. Kliknij **Add an activity or resource** na dole tej sekcji.
3. W oknie wyboru wybierz **FastComments**. Jeśli go nie widzisz, przejdź do sekcji z uwagami poniżej.

Otwiera się formularz ustawień aktywności. Pola, które mają znaczenie:

- **Activity name** (wymagane). Wyświetlane na stronie kursu i w dzienniku ocen. Przykład: `Week 3 Discussion`.
- **Activity description**. Opcjonalny tekst wprowadzający renderowany ponad wątkiem komentarzy.
- **Show description on course page**. Zaznacz, jeśli chcesz, aby opis był widoczny bez konieczności wchodzenia w aktywność.
- **Preconfigured tool**. Ustawione na `FastComments` (wybierane automatycznie po uruchomieniu z wyboru). Nie zmieniaj.
- **Launch container**. Ustaw na **New window**. Zobacz sekcję z uwagami, dlaczego „Same window” może powodować błędy w niektórych wdrożeniach Moodle.
- **Tool URL**, **Public key**, **Shared secret**, **Custom parameters**. Pozostaw puste. Rejestracja dynamiczna obsłużyła to na poziomie witryny.

Przewiń na dół i kliknij **Save and return to course** (lub **Save and display**, aby od razu otworzyć aktywność).

Aktywność pojawia się jako wiersz w sekcji z ikoną FastComments. Studenci klikają w wiersz, aby otworzyć wątek komentarzy.

#### Osadź FastComments bezpośrednio w edytorze

Dla wątku wewnątrz strony Page, rozdziału Book, Lekcji lub innego zasobu korzystającego z edytora Atto lub TinyMCE:

1. Otwórz zasób w trybie edycji.
2. Umieść kursor w miejscu, w którym ma pojawić się wątek.
3. W pasku narzędzi edytora kliknij przycisk **LTI** / **External tool**. W Atto jest on oznaczony jako „Insert LTI Advantage content”. W TinyMCE (domyślnie w Moodle 4.3+) znajduje się pod menu **More** jako **External tools**.
4. Wybierz z listy narzędzi **FastComments**.
5. FastComments otworzy picker do głębokiego linkowania. Potwierdź tytuł wątku i kliknij **Embed**.
6. Edytor wstawi blok zastępczy LTI. Zapisz zasób.

Każda osadzona instancja to odrębny wątek identyfikowany przez ID elementu zawartości deep-link, więc strona z trzema osadzonymi FastComments zawiera trzy niezależne wątki.

#### Ograniczenia dostępu i ustawienia grup

Standardowe ustawienia aktywności Moodle mają zastosowanie do aktywności FastComments:

- **Common module settings** > **Group mode**. Ustawienie tego na **Separate groups** lub **Visible groups** samo w sobie nie dzieli FastComments na wątki per grupa. Tryb grup w Moodle jedynie filtruje dziennik ocen i listę członków. Aby prowadzić oddzielne wątki dla każdej grupy, dodaj jedną aktywność FastComments na grupę i użyj **Restrict access**, aby ograniczyć zasięg każdej z nich.
- **Restrict access** > **Add restriction**. Obsługuje standardowe warunki Moodle: **Date**, **Grade**, **Group**, **Grouping**, **User profile** oraz zagnieżdżone zestawy ograniczeń. Użyj **Group**, aby przypisać aktywność FastComments do jednej grupy.
- **Activity completion**. Ustaw na **Students must view this activity to complete it**, jeśli chcesz śledzić ukończenie aktywności. FastComments obecnie nie zgłasza do Moodle zdarzenia zakończenia poza samym uruchomieniem.

#### Mapowanie ról

FastComments odczytuje roszczenie `roles`, które Moodle wysyła przy każdym uruchomieniu i mapuje je w następujący sposób:

- Moodle **Manager** lub **Site administrator** -> FastComments **admin**
- Moodle **Editing teacher** lub **Non-editing teacher** -> FastComments **moderator**
- Moodle **Student** -> FastComments **commenter**
- Moodle **Guest** -> tylko do odczytu

Administratorzy mogą usuwać dowolne komentarze, blokować użytkowników i edytować ustawienia wątku. Moderatorzy mogą usuwać i zatwierdzać komentarze wewnątrz wątku, do którego zostali wprowadzeni. Niestandardowe role Moodle dziedziczą mapowanie archetypu, z którego zostały sklonowane.

#### Co widzą studenci

Studenci klikają aktywność FastComments (lub przewijają do osadzonego bloku wewnątrz Page lub Book). Moodle przesyła ich tożsamość do FastComments za pomocą uruchomienia LTI:

- Brak ekranu logowania. FastComments loguje ich przy użyciu konta Moodle.
- Ich nazwa wyświetlana, e-mail i awatar pochodzą z Moodle.
- Wątek jest przypisany do zakresu (Moodle site, course, resource link ID), więc ta sama aktywność skopiowana do innego kursu otrzymuje nowy wątek.
- Odpowiedzi w wątku, głosowanie i powiadomienia działają tak samo jak w samodzielnym wątku FastComments.

#### Moodle — uwagi i problemy

**FastComments nie pojawia się w wyborze aktywności.** Administrator witryny zarejestrował narzędzie, ale nie ustawił **Tool configuration usage** na **Show in activity chooser and as a preconfigured tool**. Napraw to w **Site administration** > **Plugins** > **Activity modules** > **External tool** > **Manage tools** > ikona zębatki na kafelku FastComments.

**Uruchomienie nie powiodło się lub pokazuje puste ramki, gdy ustawiono „Same window”.** Ciasteczka sesyjne Moodle używają domyślnie `SameSite=Lax`, a niektóre przeglądarki usuwają je przy cross-site POST, którego używa LTI 1.3, aby powrócić z FastComments. Ustaw **Launch container** na **New window** w ustawieniach aktywności. Jest to twardy wymóg dla osadzonych FastComments wewnątrz Page lub Book, ponieważ ścieżka uruchomienia osadzonego przez edytor zawsze otwiera nowe okno.

**Roszczenie `iss` to URL witryny Moodle, a nie identyfikator tenant’a.** FastComments używa URL witryny Moodle (wartość konfiguracji `wwwroot`) jako wydawcy LTI. Jeśli Twoja instancja Moodle zmieni domenę lub zmienisz `wwwroot`, istniejące wątki FastComments pozostaną powiązane ze starym wydawcą i nie będą pasować do nowych uruchomień. Zarejestruj ponownie narzędzie dla nowego URL i w razie potrzeby przenieś wątki przez panel administracyjny FastComments.

**Kopia zapasowa i przywracanie aktywności.** Utworzenie kopii zapasowej kursu i przywrócenie go w nowym kursie tworzy nowe resource link ID, więc przywrócone aktywności FastComments zaczynają od pustych wątków. Oryginalny kurs zachowuje pierwotne wątki. To zamierzone zachowanie, a nie błąd.

**Moodle 4.5 — TinyMCE jako domyślny.** Moodle 4.5 dostarczany jest z TinyMCE jako domyślnym edytorem dla nowych instalacji. Przycisk External tool znajduje się w menu **More** (`...`), a nie na głównym pasku narzędzi. Starsze witryny, które zostały zaktualizowane z wersji 4.1, pozostają przy Atto, chyba że administrator zmienił domyślny edytor.
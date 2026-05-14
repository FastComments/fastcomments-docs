Ta strona opisuje dodawanie FastComments do kursu Brightspace po tym, jak administrator zarejestrował narzędzie i utworzył wdrożenie. Jeśli narzędzie nie jest jeszcze zarejestrowane, najpierw zobacz przewodnik rejestracji D2L.

<div class="screenshot white-bg">
    <div class="title">FastComments osadzone jako temat jednostki w Brightspace</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-d2l-comments-in-unit.png" alt="FastComments uruchomione wewnątrz jednostki Brightspace, pokazujące wątkowe komentarze i selektor @-wzmianki" />
</div>

Brightspace oferuje dwa doświadczenia tworzenia treści: **Classic Content** i **New Content Experience** (zwane także **Lessons**). Oba udostępniają FastComments, ale ścieżki w menu różnią się. Każda sekcja poniżej obejmuje oba przypadki tam, gdzie się rozchodzą.

#### Znajdź narzędzie FastComments

Narzędzie FastComments pojawia się w dwóch miejscach w edytorze treści kursu:

1. W selektorze aktywności, dostępnym z przycisku modułu/jednostki **Add Existing** (w starszych wersjach Brightspace opisane jako **Add Existing Activities**). FastComments pojawia się bezpośrednio w selektorze w obecnych wersjach Brightspace; w starszych wersjach jest zagnieżdżone w podmenu **External Learning Tools**. Każda z tych ścieżek dodaje FastComments jako osobny temat.
2. W dialogu **Insert Stuff** wewnątrz edytora HTML, pod **LTI Advantage**. To osadza FastComments inline w temacie HTML za pomocą przepływu deep linking LTI.

Jeśli FastComments nie pojawia się w żadnym z selektorów, wdrożenie nie jest włączone dla jednostki organizacyjnej przechowującej kurs. Poproś administratora Brightspace, aby otworzył **Admin Tools** > **Manage Extensibility** > **LTI Advantage** > FastComments tool > **View Deployments**, otworzył wdrożenie i dodał jednostkę organizacyjną kursu (lub jednostkę nadrzędną) w sekcji **Org Units**.

#### Dodaj FastComments jako temat w module

Classic Content:

1. Otwórz kurs i kliknij **Content** w pasku nawigacyjnym.
2. Wybierz moduł, który ma zawierać dyskusję (lub utwórz nowy za pomocą **Add a module**).
3. Kliknij **Add Existing** (starszy Brightspace: **Add Existing Activities** > **External Learning Tools**).
4. W selektorze kliknij **FastComments**. Brightspace utworzy temat w module i wróci do widoku zawartości.
5. Kliknij nowy temat. Zmień jego nazwę na coś opisowego, np. `FastComments Discussion`, używając edytora tytułu inline.

New Content Experience (Lessons):

1. Otwórz kurs i kliknij **Content**.
2. Otwórz jednostkę i lekcję, które mają zawierać dyskusję.
3. Kliknij **Add** > **Existing Activity** i wybierz **FastComments** (starszy Brightspace: zagnieżdżone pod **External Learning Tools**).
4. Aktywność zostanie dodana do lekcji.
5. Kliknij tytuł aktywności, aby go zmienić.

Za pierwszym razem, gdy dowolny użytkownik (instruktor lub student) otworzy temat, FastComments inicjalizuje wątek dla tego resource link. Wątek jest powiązany z identyfikatorem resource link, więc zmiana nazwy lub przeniesienie tematu nie zmienia, który wątek jest ładowany.

#### Osadź FastComments inline w temacie HTML

Użyj tego przepływu, gdy chcesz, aby komentarze pojawiały się pod materiałem do czytania, wideo lub inną treścią wewnątrz tej samej strony tematu, zamiast jako osobny temat.

1. Otwórz lub utwórz temat HTML w module/lekcji.
2. Kliknij **Edit HTML**, aby otworzyć edytor HTML Brightspace.
3. Umieść kursor w miejscu, gdzie ma się pojawić wątek komentarzy.
4. Kliknij przycisk **Insert Stuff** (ikona puzzla na pasku narzędzi edytora).
5. W dialogu Insert Stuff przewiń do **LTI Advantage** i kliknij **FastComments**.
6. FastComments otworzy selektor deep linking. Potwierdź umiejscowienie (domyślne opcje działają dla dyskusji zawartości); kliknij **Insert** lub **Continue**.
7. Brightspace wróci do edytora HTML z blokiem zastępczym reprezentującym uruchomienie LTI. Kliknij **Save and Close** w temacie.

Gdy temat się załaduje, Brightspace zastąpi blok zastępczy iframe’em, który automatycznie uruchamia FastComments przez LTI. Studenci zobaczą wątek dyskusji inline.

Pojedynczy temat HTML może zawierać wiele deep-linked osadzeń FastComments. Każde osadzenie otrzymuje własny wątek, ponieważ każde deep link generuje odrębny identyfikator resource link.

#### Temat modułu vs szybkie osadzenie inline

Wybierz podejście **module topic**, gdy:

- Dyskusja jest główną aktywnością na tym kroku w module.
- Chcesz, aby temat pojawiał się w spisie treści Brightspace, śledzeniu ukończeń i Class Progress.

Wybierz podejście **inline embed**, gdy:

- Komentarze powinny znajdować się pod inną treścią na tej samej stronie.
- Nie chcesz oddzielnego elementu śledzającego ukończenie w spisie treści.

#### Widoczność, szkic i warunki udostępnienia

Nowy temat FastComments jest domyślnie widoczny dla studentów. Aby go ukryć podczas konfiguracji:

1. W edytorze zawartości kliknij tytuł tematu (Classic) lub menu trzech kropek przy aktywności (New Content Experience).
2. Ustaw status na **Draft** (Classic) lub wyłącz przełącznik **Visibility** (New Content Experience).

Tematy w stanie szkicu są niewidoczne dla studentów. Instruktorzy i asystenci nadal widzą je z odznaką "Draft".

Aby ograniczyć temat do konkretnej grupy lub sekcji:

1. Otwórz temat.
2. Kliknij menu tytułu tematu > **Edit Properties In-place** (Classic) lub **Edit** > **Restrictions** (New Content Experience).
3. W sekcji **Release Conditions** kliknij **Create**.
4. Wybierz **Group enrollment** lub **Section enrollment**, wybierz grupę/sekcję i zapisz.

Warunki udostępnienia sumują się z własnym mapowaniem ról FastComments. Studenci, którzy nie mogą zobaczyć tematu, nie otrzymują uruchomienia LTI.

#### Co studenci widzą przy pierwszym uruchomieniu

Gdy student kliknie temat (lub załaduje temat HTML z osadzeniem):

1. Brightspace wykonuje w tle uruchomienie LTI 1.3.
2. FastComments otrzymuje imię i nazwisko studenta, email, URL avatara oraz rolę w LMS i loguje go automatycznie. Nie pojawia się żadne okno logowania do FastComments.
3. Wątek komentarzy dla tego resource link renderuje się wewnątrz iframe Brightspace.

Mapowanie ról przy uruchomieniu:

- Brightspace `Administrator` staje się w FastComments administratorem (pełne moderowanie, usuwanie, ban i dostęp do konfiguracji).
- Brightspace `Instructor` staje się w FastComments moderatorem (przypinanie, ukrywanie, usuwanie, ban).
- Wszystkie inne role (`Learner`, `TeachingAssistant`, itd.) stają się zwykłymi komentującymi.

Komentarze są przypisywane do konta studenta w Brightspace. Jeśli student zmieni swoje imię lub avatar w Brightspace, następne uruchomienie LTI zsynchronizuje zmianę.

#### Wysokość iframe i zmiana rozmiaru

FastComments wysyła wiadomość postMessage `org.imsglobal.lti.frameResize` przy każdym renderze wątku i przy zmianach treści (nowy komentarz, rozwinięcie odpowiedzi). Brightspace nasłuchuje tej wiadomości i dostosowuje wysokość iframe, aby wątek nie był obcięty i nie pojawiał się wewnętrzny pasek przewijania.

Jeśli iframe pozostaje na stałej małej wysokości:

- Potwierdź, że kurs jest ładowany przez HTTPS. Nasłuchiwacz postMessage Brightspace odrzuca ramki mieszanej zawartości.
- Potwierdź, że żadna rozszerzenie przeglądarki nie blokuje kanału postMessage.
- Dla osadzeń inline w temacie HTML, otaczający HTML nie może opakowywać iframe’a w kontener o stałej wysokości. Usuń wszelkie inline `style="height: ..."` z elementu nadrzędnego.

#### Specyficzne pułapki Brightspace

**Narzędzie nie pokazuje się w selektorze Add Existing.** Wdrożenie nie jest włączone dla jednostki organizacyjnej tego kursu. Administrator musi dodać jednostkę organizacyjną (lub jednostkę nadrzędną) do listy **Org Units** wdrożenia. Sama rejestracja narzędzia nie wystarczy; to wdrożenie określa, które kursy widzą narzędzie.

**Niezgodność `deployment_id` przy uruchomieniu.** FastComments przypina metodą TOFU pierwszy `deployment_id`, który zobaczył dla rejestracji. Jeśli administrator usunie oryginalne wdrożenie i utworzy nowe, uruchomienia z nowego wdrożenia będą odrzucane z błędem niezgodności deploymentu. Naprawą jest ponowna rejestracja FastComments (wygenerowanie nowego URL rejestracji i ponowne uruchomienie rejestracji dynamicznej); stary rekord konfiguracji zostanie zastąpiony.

**Narzędzie uruchamia się, ale pokazuje "Invalid LTI launch".** Kurs znajduje się w innej strukturze tenanta/organizacji niż zakres wdrożenia, lub wdrożenie zostało wyłączone po rejestracji. Sprawdź ponownie **Admin Tools** > **Manage Extensibility** > **LTI Advantage** > FastComments > przełącznik **Enabled** oraz listę jednostek organizacyjnych wdrożenia.

**Brakuje imion i ról wewnątrz FastComments.** Brightspace wysyła uruchomienia LTI z Names and Role Provisioning Services (NRPS) claims. Jeśli kurs został zaktualizowany ze starszego linku LTI 1.1, uruchomienie może nie zawierać claimów `name` i `email`. Ponownie dodaj temat FastComments przez **Add Existing** (nie migruj starego linku), aby uruchomienie używało LTI 1.3.

**Osadzenie pokazuje ekran logowania zamiast automatycznego SSO.** Temat HTML został wstawiony jako zwykły `<iframe>` wskazujący bezpośrednio na FastComments zamiast przez **Insert Stuff** > **LTI Advantage**. Zwykłe iframe’y pomijają uruchomienie LTI i kierują użytkowników na publiczną stronę FastComments. Usuń iframe i wstaw ponownie przez przepływ Insert Stuff.
Ta strona opisuje dodawanie FastComments do kursu w Brightspace po tym, jak administrator zarejestrował narzędzie i utworzył wdrożenie. Jeśli narzędzie nie zostało jeszcze zarejestrowane, najpierw zobacz przewodnik rejestracji D2L.

Brightspace udostępnia dwa doświadczenia tworzenia treści: **Classic Content** oraz **New Content Experience** (zwane również **Lessons**). Oba udostępniają FastComments, ale ścieżki w menu różnią się. Każda sekcja poniżej omawia oba przypadki tam, gdzie się rozchodzą.

#### Zlokalizuj narzędzie FastComments

Narzędzie FastComments pojawia się w dwóch miejscach w edytorze zawartości kursu:

1. Picker aktywności, dostępny z przycisku **Add Existing** modułu/jednostki (w starszych wersjach Brightspace oznaczony jako **Add Existing Activities**). W nowszych wersjach Brightspace FastComments pojawia się bezpośrednio w pickerze; w starszych wersjach jest zagnieżdżone pod podmenu **External Learning Tools**. Każda z tych ścieżek dodaje FastComments jako samodzielny temat.
2. Okno dialogowe **Insert Stuff** wewnątrz edytora HTML, pod **LTI Advantage**. To osadza FastComments inline w temacie HTML za pomocą przepływu głębokiego linkowania LTI.

Jeśli FastComments nie pojawia się w żadnym z pickerów, wdrożenie nie jest włączone dla jednostki organizacyjnej kursu. Poproś administratora Brightspace o wejście do **Admin Tools** > **Manage Extensibility** > **LTI Advantage** > narzędzie FastComments > **View Deployments**, otwarcie wdrożenia i dodanie jednostki organizacyjnej kursu (lub jednostki nadrzędnej) pod **Org Units**.

#### Dodaj FastComments jako temat w module

Classic Content:

1. Otwórz kurs i kliknij **Content** w pasku nawigacyjnym.
2. Wybierz moduł, który ma zawierać dyskusję (lub utwórz nowy przez **Add a module**).
3. Kliknij **Add Existing** (starszy Brightspace: **Add Existing Activities** > **External Learning Tools**).
4. W pickerze kliknij **FastComments**. Brightspace tworzy temat w module i wraca do widoku zawartości.
5. Kliknij nowy temat. Zmień jego nazwę na coś opisowego, np. `FastComments Discussion`, używając edytora tytułu inline.

New Content Experience (Lessons):

1. Otwórz kurs i kliknij **Content**.
2. Otwórz jednostkę i lesson, które mają zawierać dyskusję.
3. Kliknij **Add** > **Existing Activity** i wybierz **FastComments** (w starszym Brightspace: zagnieżdżone pod **External Learning Tools**).
4. Aktywność zostaje dodana do lesson.
5. Kliknij tytuł aktywności, aby zmienić jej nazwę.

Za pierwszym razem, gdy dowolny użytkownik (prowadzący lub student) otworzy temat, FastComments inicjalizuje wątek dla tego resource link. Wątek jest powiązany z identyfikatorem resource link, więc zmiana nazwy lub przeniesienie tematu nie zmienia wątku, który jest ładowany.

#### Osadź FastComments inline w temacie HTML

Użyj tego przepływu, gdy chcesz, aby komentarze pojawiały się pod lekturą, wideo lub inną zawartością wewnątrz tej samej strony tematu zamiast jako osobny temat.

1. Otwórz lub utwórz temat HTML w module/lesson.
2. Kliknij **Edit HTML**, aby otworzyć edytor HTML Brightspace.
3. Umieść kursor w miejscu, gdzie ma się pojawić wątek komentarzy.
4. Kliknij przycisk **Insert Stuff** (ikona puzzla na pasku narzędzi edytora).
5. W oknie Insert Stuff przewiń do **LTI Advantage** i kliknij **FastComments**.
6. FastComments otworzy picker głębokiego linkowania. Potwierdź umiejscowienie (domyślne opcje działają dla dyskusji w treści); kliknij **Insert** lub **Continue**.
7. Brightspace wróci do edytora HTML z blokiem zastępczym reprezentującym uruchomienie LTI. Kliknij **Save and Close** w temacie.

Gdy temat się załaduje, Brightspace zastąpi blok zastępczy iframe’em, który automatycznie uruchamia FastComments za pomocą LTI. Studenci zobaczą wątek dyskusji osadzony inline.

Pojedynczy temat HTML może zawierać wiele głębokich linków FastComments. Każde osadzenie otrzymuje własny wątek, ponieważ każdy głęboki link generuje odrębny identyfikator resource link.

#### Temat modułu vs szybkie osadzenie inline

Wybierz podejście „temat modułu”, gdy:

- Dyskusja jest główną aktywnością na danym etapie modułu.
- Chcesz, aby temat pojawiał się w spisie treści Brightspace, w monitoringu ukończeń i w Class Progress.

Wybierz podejście „osadzenie inline”, gdy:

- Komentarze mają znajdować się pod inną zawartością na tej samej stronie.
- Nie chcesz mieć osobnego elementu śledzonego pod kątem ukończeń w spisie treści.

#### Widoczność, szkic i warunki udostępniania

Nowy temat FastComments jest domyślnie widoczny dla studentów. Aby ukryć go podczas konfiguracji:

1. W edytorze zawartości kliknij tytuł tematu (Classic) lub menu trzech kropek przy aktywności (New Content Experience).
2. Ustaw status na **Draft** (Classic) lub wyłącz przełącznik **Visibility** (New Content Experience).

Tematy w stanie szkicu są niewidoczne dla studentów. Prowadzący i asystenci nadal widzą je z odznaką „Draft”.

Aby ograniczyć widoczność tematu do konkretnej grupy lub sekcji:

1. Otwórz temat.
2. Kliknij menu tytułu tematu > **Edit Properties In-place** (Classic) lub **Edit** > **Restrictions** (New Content Experience).
3. W sekcji **Release Conditions** kliknij **Create**.
4. Wybierz **Group enrollment** lub **Section enrollment**, wybierz grupę/sekcję i zapisz.

Warunki udostępniania nakładają się na mapowanie ról FastComments. Studenci, którzy nie widzą tematu, nie otrzymują uruchomienia LTI.

#### Co studenci widzą przy pierwszym uruchomieniu

Gdy student kliknie temat (lub załaduje temat HTML z osadzeniem):

1. Brightspace wykonuje w tle uruchomienie LTI 1.3.
2. FastComments otrzymuje imię i nazwisko studenta, adres e-mail, URL awatara oraz rolę w LMS i loguje go automatycznie. Nie pojawia się monit logowania do FastComments.
3. Wątek komentarzy dla tego resource link renderuje się wewnątrz iframe’a Brightspace.

Mapowanie ról podczas uruchomienia:

- Brightspace `Administrator` staje się w FastComments administratorem (pełne możliwości moderacji, usuwania, blokowania i dostęp do konfiguracji).
- Brightspace `Instructor` staje się w FastComments moderatorem (pinowanie, ukrywanie, usuwanie, blokowanie).
- Wszystkie pozostałe role (`Learner`, `TeachingAssistant`, itp.) stają się standardowymi komentującymi.

Komentarze przypisywane są do konta studenta w Brightspace. Jeżeli student zmieni swoje imię lub awatar w Brightspace, następne uruchomienie LTI zsynchronizuje tę zmianę.

#### Wysokość iframe i zmiana rozmiaru

FastComments wysyła komunikat postMessage `org.imsglobal.lti.frameResize` przy każdym renderowaniu wątku i przy zmianach treści (nowy komentarz, rozwinięcie odpowiedzi). Brightspace nasłuchuje tego komunikatu i dopasowuje wysokość iframe’a, aby wątek nie był przycięty i nie pojawiał się wewnętrzny pasek przewijania.

Jeśli iframe pozostaje o stałej małej wysokości:

- Potwierdź, że kurs jest ładowany przez HTTPS. Nasłuchiwacz postMessage Brightspace odrzuca ramki mieszanej zawartości.
- Potwierdź, że żadna wtyczka przeglądarki nie blokuje kanału postMessage.
- Dla osadzeń inline w temacie HTML, otaczający HTML nie może opakowywać iframe’a w kontener o stałej wysokości. Usuń wszelkie inline’owe atrybuty `style="height: ..."` z elementu nadrzędnego.

#### Specyficzne pułapki Brightspace

**Narzędzie nie pokazuje się w pickerze Add Existing.** Wdrożenie nie jest włączone dla jednostki organizacyjnej tego kursu. Administrator musi dodać jednostkę organizacyjną (lub jednostkę nadrzędną) do listy **Org Units** wdrożenia. Sama rejestracja narzędzia nie wystarczy; wdrożenie określa, które kursy widzą narzędzie.

**`deployment_id` mismatch on launch.** FastComments przypina metodą TOFU pierwsze `deployment_id`, które zobaczy dla rejestracji. Jeśli administrator usunie oryginalne wdrożenie i utworzy nowe, uruchomienia z nowego wdrożenia zostaną odrzucone z błędem niezgodności wdrożenia. Naprawą jest ponowna rejestracja FastComments (wygenerowanie nowego URL rejestracji i ponowne przeprowadzenie Dynamic Registration); stary rekord konfiguracji zostanie zastąpiony.

**Narzędzie uruchamia się, ale pokazuje „Invalid LTI launch”.** Kurs znajduje się w innej strukturze tenant/org niż ta, którą obejmuje wdrożenie, lub wdrożenie zostało wyłączone po rejestracji. Sprawdź ponownie **Admin Tools** > **Manage Extensibility** > **LTI Advantage** > FastComments > przełącznik **Enabled** oraz listę jednostek organizacyjnych wdrożenia.

**Imiona i role brakują w FastComments.** Brightspace wysyła uruchomienia LTI z roszczeniami Names and Role Provisioning Services (NRPS). Jeśli kurs został zaktualizowany ze starszego linku LTI 1.1, uruchomienie może nie zawierać roszczeń `name` i `email`. Ponownie dodaj temat FastComments przez **Add Existing** (nie migruj starego linku), aby uruchomienie korzystało z LTI 1.3.

**Osadzenie pokazuje ekran logowania zamiast automatycznego SSO.** Temat HTML został wstawiony jako zwykły element `<iframe>` wskazujący bezpośrednio na FastComments zamiast przez **Insert Stuff** > **LTI Advantage**. Zwykłe iframe’y pomijają uruchomienie LTI i kierują użytkowników na publiczną stronę FastComments. Usuń iframe i wstaw ponownie przez przepływ Insert Stuff.
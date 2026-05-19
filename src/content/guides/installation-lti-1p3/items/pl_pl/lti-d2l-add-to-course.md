Ta strona opisuje dodawanie FastComments do kursu Brightspace po tym, jak administrator zarejestrował narzędzie i utworzył wdrożenie. Jeśli narzędzie nie zostało jeszcze zarejestrowane, najpierw zobacz przewodnik rejestracji D2L.

<div class="screenshot white-bg">
    <div class="title">FastComments osadzony jako temat jednostki w Brightspace</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-d2l-comments-in-unit.png" alt="FastComments running inside a Brightspace unit, showing threaded comments and an @-mention picker" />
</div>

Brightspace oferuje dwa środowiska tworzenia treści: **Classic Content** oraz **New Content Experience** (również nazywane **Lessons**). Oba umożliwiają korzystanie z FastComments, lecz ścieżki w menu się różnią. Każda z poniższych sekcji opisuje oba przypadki tam, gdzie się rozchodzą.

#### Zlokalizuj narzędzie FastComments

Narzędzie FastComments pojawia się w dwóch miejscach w edytorze zawartości kursu:

1. W selektorze aktywności, dostępnym z przycisku modułu/jednostki **Add Existing** (w starszych wersjach Brightspace oznaczonym jako **Add Existing Activities**). W aktualnych wersjach Brightspace FastComments pojawia się bezpośrednio w selektorze; w starszych wersjach znajduje się w podmenu **External Learning Tools**. Każda z dróg dodaje FastComments jako samodzielny temat.
2. W oknie dialogowym **Insert Stuff** w edytorze HTML, pod **LTI Advantage**. To osadza FastComments inline w temacie HTML za pomocą przepływu deep linking LTI.

Jeśli FastComments nie pojawia się w żadnym z selektorów, wdrożenie nie jest włączone dla jednostki organizacyjnej (org unit) zawierającej kurs. Poproś administratora Brightspace o przejście do **Admin Tools** > **Manage Extensibility** > **LTI Advantage** > narzędzie FastComments > **View Deployments**, otwarcie wdrożenia i dodanie jednostki organizacyjnej kursu (lub jednostki nadrzędnej) w sekcji **Org Units**.

#### Dodaj FastComments jako temat w module

Classic Content:

1. Otwórz kurs i kliknij **Content** w pasku nawigacyjnym.
2. Wybierz moduł, który ma zawierać dyskusję (lub utwórz nowy przez **Add a module**).
3. Kliknij **Add Existing** (starszy Brightspace: **Add Existing Activities** > **External Learning Tools**).
4. W selektorze kliknij **FastComments**. Brightspace tworzy temat w module i wraca do widoku zawartości.
5. Kliknij nowy temat. Zmień nazwę na coś opisowego, np. `FastComments Discussion`, używając edytora tytułu inline.

New Content Experience (Lessons):

1. Otwórz kurs i kliknij **Content**.
2. Otwórz jednostkę i lekcję, która ma zawierać dyskusję.
3. Kliknij **Add** > **Existing Activity** i wybierz **FastComments** (starszy Brightspace: zagnieżdżone pod **External Learning Tools**).
4. Aktywność zostanie dodana do lekcji.
5. Kliknij tytuł aktywności, aby go zmienić.

Za pierwszym razem, gdy którykolwiek użytkownik (instruktor lub student) otworzy temat, FastComments inicjalizuje wątek dla tego resource link. Wątek jest powiązany z identyfikatorem resource link, więc zmiana nazwy lub przeniesienie tematu nie zmienia wczytywanego wątku.

#### Osadź FastComments inline w temacie HTML

Użyj tego przepływu, gdy chcesz, aby komentarze pojawiały się poniżej materiału do przeczytania, wideo lub innej zawartości w tej samej stronie tematu, zamiast jako oddzielny temat.

1. Otwórz lub utwórz temat HTML w module/lekcji.
2. Kliknij **Edit HTML**, aby otworzyć edytor HTML Brightspace.
3. Umieść kursor w miejscu, gdzie ma się pojawić wątek komentarzy.
4. Kliknij przycisk **Insert Stuff** (ikonka puzzla na pasku narzędzi edytora).
5. W oknie Insert Stuff przewiń do **LTI Advantage** i kliknij **FastComments**.
6. FastComments otworzy selektor deep linking. Potwierdź miejsce umieszczenia (domyślne opcje działają dla dyskusji z treści); kliknij **Insert** lub **Continue**.
7. Brightspace wróci do edytora HTML z blokiem zastępczym reprezentującym uruchomienie LTI. Kliknij **Save and Close** w temacie.

Gdy temat się załaduje, Brightspace zastąpi blok zastępczy iframe’em, który automatycznie uruchomi FastComments przez LTI. Studenci zobaczą wątek dyskusji inline.

Pojedynczy temat HTML może zawierać wiele deep-linked osadzeń FastComments. Każde osadzenie otrzymuje własny wątek, ponieważ każdy deep link generuje odrębny identyfikator resource link.

#### Temat modułu vs osadzenie inline (Quicklink)

Wybierz podejście **temat modułu**, gdy:

- Dyskusja jest główną aktywnością dla tego kroku w module.
- Chcesz, aby temat pojawiał się w spisie treści Brightspace, w śledzeniu ukończeń i w Class Progress.

Wybierz podejście **osadzenia inline**, gdy:

- Komentarze powinny znajdować się pod inną treścią na tej samej stronie.
- Nie chcesz oddzielnego elementu śledzonego w spisie treści.

#### Widoczność, wersja robocza i warunki publikacji

Nowy temat FastComments jest domyślnie widoczny dla studentów. Aby go ukryć podczas konfiguracji:

1. W edytorze zawartości kliknij tytuł tematu (Classic) lub menu trzech kropek przy aktywności (New Content Experience).
2. Ustaw status na **Draft** (Classic) lub wyłącz przełącznik **Visibility** (New Content Experience).

Tematy w wersji roboczej są niewidoczne dla studentów. Instruktorzy i asystenci dydaktyczni nadal widzą je z odznaką "Draft".

Aby ograniczyć temat do konkretnej grupy lub sekcji:

1. Otwórz temat.
2. Kliknij menu tytułu tematu > **Edit Properties In-place** (Classic) lub **Edit** > **Restrictions** (New Content Experience).
3. W sekcji **Release Conditions** kliknij **Create**.
4. Wybierz **Group enrollment** lub **Section enrollment**, wybierz grupę/sekcję i zapisz.

Warunki publikacji sumują się z mapowaniem ról FastComments. Studenci, którzy nie widzą tematu, nie otrzymują uruchomienia LTI.

#### Co widzą studenci przy pierwszym uruchomieniu

Gdy student kliknie temat (lub załaduje temat HTML z osadzeniem):

1. Brightspace wykonuje uruchomienie LTI 1.3 w tle.
2. FastComments otrzymuje imię i nazwisko studenta, e-mail, URL avatara oraz rolę w LMS i automatycznie loguje go. Nie pojawia się monit logowania do FastComments.
3. Wątek komentarzy dla tego resource link renderuje się wewnątrz iframe Brightspace.

Mapowanie ról przy uruchomieniu:

- Brightspace `Administrator` staje się w FastComments administratorem wątku (pełne uprawnienia do moderacji, usuwania, banowania i dostępu do konfiguracji).
- Brightspace `Instructor` staje się w FastComments moderatorem (pin, ukrywanie, usuwanie, banowanie).
- Wszystkie inne role (`Learner`, `TeachingAssistant` itp.) stają się zwykłymi komentującymi.

Komentarze są przypisywane do konta Brightspace studenta. Jeśli student zmieni swoje imię lub avatar w Brightspace, następne uruchomienie LTI zsynchronizuje zmianę.

#### Zabezpiecz publiczny dostęp (zalecane)

Domyślnie dane komentarzy FastComments są publicznie odczytywalne. Każdy, kto potrafi odgadnąć URL wątku lub punkt końcowy API, może zobaczyć komentarze, także poza Brightspace. W przypadku dyskusji kursowych niemal na pewno chcesz ograniczyć przeglądanie tylko do zapisanych uczestników.

Otwórz swoją <a href="https://fastcomments.com/auth/my-account/customize-widget" target="_blank">stronę dostosowywania widgetu</a> i utwórz regułę z włączoną opcją **Require SSO To View Comments**, następnie ustaw poziom zabezpieczeń na **Secure SSO**, aby wątki mogły być ładowane tylko przez podpisane uruchomienie LTI.

Zobacz [Protecting Comment Threads With Single-Sign-On](/guide-customizations-and-configuration.html#sso-require-to-view-comments) po pełny przewodnik, w tym jak ograniczyć regułę do pojedynczej domeny lub strony.

#### Wysokość iframe i zmiana rozmiaru

FastComments wysyła postMessage `org.imsglobal.lti.frameResize` przy każdym renderowaniu wątku i przy zmianach zawartości (nowy komentarz, rozwinięcie odpowiedzi). Brightspace nasłuchuje tego komunikatu i dostosowuje wysokość iframe, aby wątek nie był przycinany i nie pokazywał wewnętrznego paska przewijania.

Jeśli iframe pozostaje na stałej małej wysokości:

- Upewnij się, że kurs ładuje się przez HTTPS. Nasłuchiwacz postMessage Brightspace odrzuca ramki mieszanej zawartości.
- Upewnij się, że żadna wtyczka przeglądarki nie blokuje kanału postMessage.
- Dla osadzeń inline w temacie HTML, otaczający HTML nie może owijać iframe’a w kontener o stałej wysokości. Usuń wszelkie inline `style="height: ..."` z elementu nadrzędnego.

#### Specyficzne problemy w Brightspace

**Narzędzie nie pokazuje się w selektorze Add Existing.** Wdrożenie nie jest włączone dla jednostki organizacyjnej tego kursu. Administrator musi dodać jednostkę organizacyjną (lub jednostkę nadrzędną) do listy **Org Units** wdrożenia. Sama rejestracja narzędzia nie wystarczy; wdrożenie określa, które kursy widzą narzędzie.

**Niepasujący `deployment_id` przy uruchomieniu.** FastComments zapamiętuje (TOFU) pierwszy `deployment_id`, który zobaczy przy rejestracji. Jeśli administrator usunie oryginalne wdrożenie i stworzy nowe, uruchomienia z nowego wdrożenia będą odrzucane z błędem niezgodności deployment. Rozwiązaniem jest ponowna rejestracja FastComments (wygenerowanie nowego URL rejestracji (<a href="https://fastcomments.com/auth/my-account/lti-config" target="_blank">pobierz go tutaj</a>) i ponowne uruchomienie Dynamic Registration); stary rekord konfiguracji zostanie zastąpiony.

**Narzędzie uruchamia się, ale pokazuje "Invalid LTI launch".** Kurs znajduje się w innej strukturze tenant/organizacji niż ta, którą obejmuje wdrożenie, lub wdrożenie zostało wyłączone po rejestracji. Sprawdź ponownie **Admin Tools** > **Manage Extensibility** > **LTI Advantage** > FastComments > przełącznik **Enabled** oraz listę jednostek organizacyjnych wdrożenia.

**Brakuje imion i ról w FastComments.** Brightspace wysyła uruchomienia LTI z roszczeniami Names and Role Provisioning Services (NRPS). Jeśli kurs został zaktualizowany ze starszego linku LTI 1.1, uruchomienie może nie zawierać roszczeń `name` i `email`. Dodaj ponownie temat FastComments przez **Add Existing** (nie migruj starego linku), aby uruchomienie używało LTI 1.3.

**Osadzenie pokazuje ekran logowania zamiast auto-SSO.** Temat HTML został wstawiony jako zwykły `<iframe>` wskazujący na FastComments zamiast przez **Insert Stuff** > **LTI Advantage**. Zwykłe iframe’y pomijają uruchomienie LTI i kierują użytkowników na publiczną stronę FastComments. Usuń iframe i ponownie wstaw go przez przepływ Insert Stuff.
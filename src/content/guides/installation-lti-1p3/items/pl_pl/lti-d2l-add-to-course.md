Ta strona opisuje dodanie FastComments do kursu Brightspace po tym, jak administrator zarejestrował narzędzie i utworzył deployment. Jeśli narzędzie nie jest jeszcze zarejestrowane, najpierw zobacz przewodnik rejestracji D2L.

<div class="screenshot white-bg">
    <div class="title">FastComments osadzony jako temat jednostki w Brightspace</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-d2l-comments-in-unit.png" alt="FastComments działający wewnątrz jednostki Brightspace, pokazujący wątkowe komentarze i narzędzie do wspomnienia @-mention" />
</div>

Brightspace dostarcza dwa doświadczenia tworzenia zawartości: **Classic Content** oraz **New Content Experience** (zwane też **Lessons**). Oba udostępniają FastComments, ale ścieżki w menu różnią się. Każda z poniższych sekcji opisuje oba warianty tam, gdzie się rozchodzą.

#### Locate the FastComments Tool

Narzędzie FastComments pojawia się w dwóch miejscach w edytorze treści kursu:

1. W wyborze aktywności, dostępnym z przycisku modułu/jednostki **Add Existing** (w starszych wersjach Brightspace oznaczone jako **Add Existing Activities**). FastComments pojawia się bezpośrednio w pickerze w aktualnych buildach Brightspace; w starszych wersjach jest zagnieżdżone w podmenu **External Learning Tools**. Każda ścieżka dodaje FastComments jako samodzielny temat.
2. W dialogu **Insert Stuff** wewnątrz edytora HTML, w sekcji **LTI Advantage**. To osadza FastComments inline w temacie HTML za pomocą przepływu głębokiego linkowania LTI.

Jeśli FastComments nie pojawia się w żadnym z pickerów, deployment nie jest włączony dla jednostki organizacyjnej (org unit), w której znajduje się kurs. Poproś administratora Brightspace, aby otworzył **Admin Tools** > **Manage Extensibility** > **LTI Advantage** > narzędzie FastComments > **View Deployments**, otworzył deployment i dodał jednostkę organizacyjną kursu (lub jednostkę nadrzędną) w sekcji **Org Units**.

#### Add FastComments as a Topic in a Module

Classic Content:

1. Otwórz kurs i kliknij **Content** na pasku nawigacyjnym.
2. Wybierz moduł, który ma zawierać dyskusję (lub utwórz go przez **Add a module**).
3. Kliknij **Add Existing** (starszy Brightspace: **Add Existing Activities** > **External Learning Tools**).
4. W pickerze kliknij **FastComments**. Brightspace utworzy temat w module i powróci do widoku zawartości.
5. Kliknij nowy temat. Zmień jego nazwę na coś opisowego, np. `FastComments Discussion`, używając edytora tytułu inline.

New Content Experience (Lessons):

1. Otwórz kurs i kliknij **Content**.
2. Otwórz jednostkę i lekcję, która ma zawierać dyskusję.
3. Kliknij **Add** > **Existing Activity** i wybierz **FastComments** (starszy Brightspace: zagnieżdżone pod **External Learning Tools**).
4. Aktywność zostanie dodana do lekcji.
5. Kliknij tytuł aktywności, aby go zmienić.

Za pierwszym razem, gdy którykolwiek użytkownik (instruktor lub student) otworzy temat, FastComments inicjalizuje wątek dla tego resource link. Wątek jest powiązany z resource link ID, więc zmiana nazwy lub przeniesienie tematu nie zmienia wątku, który jest ładowany.

#### Embed FastComments Inline in an HTML Topic

Użyj tego przepływu, gdy chcesz, aby komentarze pojawiały się poniżej lektury, wideo lub innej zawartości wewnątrz tej samej strony tematu, zamiast jako osobny temat.

1. Otwórz lub utwórz temat HTML w module/lekcji.
2. Kliknij **Edit HTML**, aby otworzyć edytor HTML Brightspace.
3. Umieść kursor w miejscu, gdzie powinien się pojawić wątek komentarzy.
4. Kliknij przycisk **Insert Stuff** (ikona puzzla na pasku narzędzi edytora).
5. W dialogu Insert Stuff przewiń do **LTI Advantage** i kliknij **FastComments**.
6. FastComments otworzy picker deep linking. Potwierdź umiejscowienie (domyślne opcje działają dla dyskusji związanych z zawartością); kliknij **Insert** lub **Continue**.
7. Brightspace powróci do edytora HTML z blokiem zastępczym reprezentującym LTI launch. Kliknij **Save and Close** w temacie.

Gdy temat się załaduje, Brightspace zastąpi blok zastępczy iframe, który automatycznie uruchomi FastComments przez LTI. Studenci zobaczą wątek dyskusji inline.

Pojedynczy temat HTML może zawierać wiele głębokich linków FastComments. Każde osadzenie otrzymuje własny wątek, ponieważ każdy deep link generuje odrębny resource link ID.

#### Module Topic vs Inline Quicklink

Wybierz podejście **module topic**, gdy:

- Dyskusja jest główną aktywnością na tym etapie modułu.
- Chcesz, aby temat pojawiał się w spisie treści Brightspace, śledzeniu ukończeń i Class Progress.

Wybierz podejście **inline embed**, gdy:

- Komentarze powinny znajdować się poniżej innej zawartości na tej samej stronie.
- Nie chcesz oddzielnego elementu śledzonego pod kątem ukończeń w spisie treści.

#### Visibility, Draft, and Release Conditions

Nowy temat FastComments jest domyślnie widoczny dla studentów. Aby go ukryć podczas konfiguracji:

1. W edytorze zawartości kliknij tytuł tematu (Classic) lub menu trzech kropek przy aktywności (New Content Experience).
2. Ustaw status na **Draft** (Classic) lub wyłącz przełącznik **Visibility** (New Content Experience).

Tematy w stanie Draft są niewidoczne dla studentów. Instruktorzy i asystenci nauczyciela (TAs) nadal widzą je z odznaką "Draft".

Aby ograniczyć dostęp do tematu do konkretnej grupy lub sekcji:

1. Otwórz temat.
2. Kliknij menu tytułu tematu > **Edit Properties In-place** (Classic) lub **Edit** > **Restrictions** (New Content Experience).
3. W sekcji **Release Conditions** kliknij **Create**.
4. Wybierz **Group enrollment** lub **Section enrollment**, wybierz grupę/sekcję i zapisz.

Warunki wydania łączą się z własnym mapowaniem ról FastComments. Studenci, którzy nie widzą tematu, nie otrzymują LTI launch.

#### What Students See on First Launch

Gdy student kliknie temat (lub załaduje temat HTML z osadzeniem):

1. Brightspace wykonuje w tle LTI 1.3 launch.
2. FastComments otrzymuje imię i nazwisko studenta, e-mail, URL avatara oraz rolę w LMS i automatycznie go loguje. Nie ma monitów logowania FastComments.
3. Wątek komentarzy dla tego resource link renderuje się wewnątrz iframe Brightspace.

Mapowanie ról przy launchu:

- Brightspace `Administrator` becomes a FastComments **admin** for the thread (full moderation, delete, ban, and configuration access).
- Brightspace `Instructor` becomes a FastComments **moderator** (pin, hide, delete, ban).
- All other roles (`Learner`, `TeachingAssistant`, etc.) become standard commenters.

Komentarze są przypisywane do konta studenta w Brightspace. Jeśli student edytuje swoje imię lub avatar w Brightspace, następny LTI launch zsynchronizuje zmianę.

#### Iframe Height and Resize

FastComments emituje postMessage `org.imsglobal.lti.frameResize` przy każdym renderze wątku oraz przy zmianach zawartości (nowy komentarz, rozwinięcie odpowiedzi). Brightspace nasłuchuje tej wiadomości i dostosowuje wysokość iframe, aby wątek nie był przycięty i nie pokazywał wewnętrznego paska przewijania.

Jeśli iframe pozostaje o stałej, zbyt małej wysokości:

- Potwierdź, że kurs jest ładowany przez HTTPS. Nasłuchiwacz postMessage Brightspace odrzuca ramki mieszanej zawartości.
- Potwierdź, że żadna rozszerzenie przeglądarki nie blokuje kanału postMessage.
- Dla osadzeń inline w temacie HTML, otaczający HTML nie może owijać iframe w kontener o stałej wysokości. Usuń wszelkie inline `style="height: ..."` z elementu nadrzędnego.

#### Brightspace-Specific Gotchas

**Tool not showing in the Add Existing picker.** Deployment nie jest włączony dla jednostki org unit tego kursu. Administrator musi dodać jednostkę org unit (lub jednostkę nadrzędną) do listy Org Units deploymentu. Sama rejestracja narzędzia nie wystarczy; deployment określa, które kursy widzą narzędzie.

**`deployment_id` mismatch on launch.** FastComments przypina pierwszy `deployment_id`, który zobaczył, do rekordu TOFU. Jeśli administrator usunie pierwotny deployment i utworzy nowy, uruchomienia z nowego deploymentu będą odrzucane z błędem niedopasowania deploymentu. Naprawą jest ponowna rejestracja FastComments (wygenerowanie nowego URL rejestracji (<a href="https://fastcomments.com/auth/my-account/lti-config" target="_blank">get it here</a>) i ponowne uruchomienie Dynamic Registration); stary rekord konfiguracji zostanie zastąpiony.

**Tool launches but shows "Invalid LTI launch".** Kurs znajduje się w innej strukturze tenant/org niż ta, którą obejmuje deployment, lub deployment został wyłączony po rejestracji. Ponownie sprawdź **Admin Tools** > **Manage Extensibility** > **LTI Advantage** > FastComments > przełącznik **Enabled** oraz listę org unit deploymentu.

**Names and roles missing inside FastComments.** Brightspace wysyła launch z roszczeniami Names and Role Provisioning Services (NRPS). Jeśli kurs został zaktualizowany ze starszego linku LTI 1.1, launch może nie zawierać roszczeń `name` i `email`. Dodaj ponownie temat FastComments przez **Add Existing** (nie migruj starego linku), aby launch używał LTI 1.3.

**Embed shows a login screen instead of auto-SSO.** Temat HTML został wstawiony jako zwykły `<iframe>` wskazujący na FastComments zamiast przez **Insert Stuff** > **LTI Advantage**. Zwykłe iframe pomijają LTI launch i kierują użytkowników na publiczną stronę FastComments. Usuń iframe i wstaw ponownie przez przepływ Insert Stuff.
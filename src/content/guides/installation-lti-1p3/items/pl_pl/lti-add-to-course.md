Gdy FastComments zostanie zarejestrowany w Twoim LMS, prowadzący dodają go do kursów w taki sam sposób, w jaki dodają każde inne zewnętrzne narzędzie LTI.

#### D2L Brightspace

W obszarze treści kursu:

1. Kliknij **Add Existing Activities** > **External Learning Tools**.
2. Wybierz **FastComments** z listy.
3. Narzędzie pojawi się jako temat w obszarze treści. Otwórz je raz, aby zainicjować wątek komentarzy dla tego zasobu.

#### Moodle

W kursie:

1. Włącz **Edit mode**.
2. W sekcji, w której chcesz umieścić komentarze, kliknij **Add an activity or resource**.
3. Wybierz **FastComments** z wyboru aktywności.
4. Zapisz. Studenci zobaczą wątek komentarzy osadzony w tej sekcji.

#### Blackboard Learn

W kursie:

1. Przejdź do obszaru treści.
2. Kliknij **Build Content** > **FastComments** (w sekcji "Learning Tools").
3. Skonfiguruj nazwę i zatwierdź.

#### Sakai

Administratorzy witryny dodają narzędzie poprzez **Site Info** > **Manage Tools** > przewiń do **External Tools** > wybierz **FastComments** > **Continue**.

#### How Threads Are Scoped

FastComments tworzy oddzielny wątek komentarzy dla **(instancja LMS, kurs, łącze zasobu)**. To oznacza:

- Dwa różne kursy w tym samym LMS otrzymują osobne wątki, nawet jeśli używają tej samej nazwy narzędzia.
- To samo narzędzie FastComments użyte w dwóch miejscach w obrębie jednego kursu tworzy dwa wątki.
- Dwie różne instalacje Brightspace pod tym samym kontem FastComments otrzymują odrębne wątki — nazwa hosta LMS jest częścią identyfikatora wątku.

#### SSO

Studenci nie widzą ekranu logowania. LMS przesyła ich tożsamość (imię i nazwisko, e-mail, awatar, rola) do FastComments poprzez uruchomienie LTI, a FastComments automatycznie ich loguje. Ich komentarze są przypisywane do konta w LMS.

Użytkownicy z rolami w LMS **Instruktor** lub **Administrator** są automatycznie mapowani na role moderatora/administratora FastComments dla danego wątku.
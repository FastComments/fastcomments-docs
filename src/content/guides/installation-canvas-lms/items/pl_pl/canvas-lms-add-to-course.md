#### Jak komentarze pojawiają się w Twoich kursach

Po włączeniu integracji LTI i zainstalowaniu Aplikacji Zewnętrznej, FastComments działa automatycznie na podstawie umiejscowień, które skonfigurowałeś:

#### Widok zadania

Jeśli umiejscowienie **Widok zadania** jest włączone, komentarze pojawiają się automatycznie poniżej każdego zadania w kursie. Studenci i instruktorzy widzą sekcję z wątkami komentarzy podczas przeglądania zadania — nie jest potrzebna dodatkowa konfiguracja dla każdego zadania.

Każde zadanie otrzymuje swój własny, oddzielny wątek komentarzy.

#### Przycisk Edytora treści sformatowanej

Jeśli umiejscowienie **Przycisk Edytora** jest włączone, instruktorzy mogą osadzać FastComments w dowolnej treści korzystającej z Edytora treści sformatowanej:

1. Edytuj **Stronę**, **Quiz** lub **Ogłoszenie**.
2. W pasku narzędzi Edytora treści sformatowanej kliknij przycisk **FastComments**.
3. FastComments jest automatycznie osadzany w treści.
4. Zapisz stronę.

Gdy studenci przeglądają stronę, osadzony widget FastComments ładuje się z wątkiem komentarzy unikatowym dla tej strony.

#### Automatyczne logowanie SSO

W obu umiejscowieniach studenci są automatycznie logowani za pomocą swojego konta Canvas. Imiona i nazwiska, adresy e-mail oraz awatary są synchronizowane przez uruchomienie LTI, nie jest potrzebne osobne logowanie.

#### Zablokuj publiczny dostęp (zalecane)

Domyślnie dane komentarzy FastComments są publicznie czytelne. Każdy, kto zgadnie URL wątku lub punkt końcowy API, może zobaczyć jego komentarze, nawet poza Canvas. Dla dyskusji kursowych prawie na pewno chcesz ograniczyć ich widoczność tylko do zapisanych studentów.

Otwórz swoją <a href="https://fastcomments.com/auth/my-account/customize-widget" target="_blank">stronę dostosowywania widgetu</a> i utwórz regułę z włączoną opcją **Wymagaj SSO do wyświetlania komentarzy**, a następnie ustaw poziom zabezpieczeń na **Bezpieczne SSO**, aby wątki mogły być ładowane tylko przez podpisane uruchomienie LTI.

Zobacz [Ochrona wątków komentarzy za pomocą logowania jednokrotnego (SSO)](/guide-customizations-and-configuration.html#sso-require-to-view-comments) po pełny przewodnik, w tym jak ograniczyć regułę do pojedynczej domeny lub strony.
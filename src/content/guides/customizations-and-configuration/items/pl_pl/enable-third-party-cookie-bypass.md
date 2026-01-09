[related-parameter-start name = 'enableThirdPartyCookieBypass'; type = 'boolean'; related-parameter-end]

W celu uwierzytelniania FastComments wymaga włączenia cookies stron trzecich w Twojej przeglądarce. Bez nich użytkownicy zawsze będą musieli
zostawić swój adres e‑mail, aby skomentować (chyba że pole adresu e‑mail jest ukryte), a ich komentarze będą zawsze wyświetlane jako niezweryfikowane (domyślnie).

Aby to obejść, możesz włączyć mechanizm obejścia cookies stron trzecich. 

Po włączeniu tego ustawienia pojawi się małe okienko popup informujące, że użytkownik jest logowany. To okienko
pojawia się zawsze, gdy użytkownik wchodzi w interakcję z widżetem komentarzy; na przykład gdy zostawia komentarz.

Możemy to zrobić w kodzie, ustawiając flagę **enableThirdPartyCookieBypass** na true:

[code-example-start config = {enableThirdPartyCookieBypass: true}; linesToHighlight = [6]; title = 'Enabling Third-Party Cookie Bypass'; code-example-end]

Możemy to także skonfigurować przez interfejs dostosowywania widżetu, w sekcji `Enable Third-Party Cookie Popup`:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.enable-third-party-cookie-bypass'; clickSelectors = ['.enable-third-party-cookie-bypass']; title='Enabling Third-Party Cookie Bypass' app-screenshot-end]
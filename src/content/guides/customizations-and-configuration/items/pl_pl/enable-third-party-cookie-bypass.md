[related-parameter-start name = 'enableThirdPartyCookieBypass'; type = 'boolean'; related-parameter-end]

Do uwierzytelniania FastComments zależy od włączonych w przeglądarce ciasteczek stron trzecich. Bez nich użytkownicy zawsze będą musieli
podać swój e‑mail, aby skomentować (chyba że pole wprowadzania e‑maila jest ukryte), a ich komentarze będą zawsze wyświetlane jako niezweryfikowane (domyślnie).

Aby obejść ten problem, możesz włączyć obejście ciasteczek stron trzecich.

Gdy to ustawienie jest włączone, spowoduje wyświetlenie małego popupu, który pokazuje komunikat informujący, że użytkownik jest logowany. Ten popup
pokazuje się za każdym razem, gdy użytkownik wchodzi w interakcję z widżetem komentarzy; na przykład, gdy zostawia komentarz.

Możemy to zrobić w kodzie, ustawiając flagę **enableThirdPartyCookieBypass** na true:

[code-example-start config = {enableThirdPartyCookieBypass: true}; linesToHighlight = [6]; title = 'Włączanie obejścia ciasteczek stron trzecich'; code-example-end]

Możemy również ustawić to za pomocą UI Dostosowywania Widżetu, w sekcji `Enable Third-Party Cookie Popup`:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.enable-third-party-cookie-bypass'; clickSelectors = ['.enable-third-party-cookie-bypass']; alt='Strona dostosowywania widżetu z zaznaczonym polem wyboru Enable Third-Party Cookie Popup'; title='Włączanie obejścia ciasteczek stron trzecich' app-screenshot-end]

---
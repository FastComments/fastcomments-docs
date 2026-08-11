---
[related-parameter-start name = 'enableCommenterLinks'; type = 'boolean'; related-parameter-end]

Domyślnie FastComments poprosi użytkownika jedynie o komentarz, nazwę użytkownika i adres e‑mail.

Jednak w niektórych sytuacjach możesz chcieć, aby użytkownik podał link do swojego bloga lub strony internetowej.

Możemy włączyć wyświetlanie dodatkowego pola wejściowego, aby podać adres URL strony użytkownika, ustawiając flagę **enableCommenterLinks** na true:

[code-example-start config = {enableCommenterLinks: true}; linesToHighlight = [6]; title = 'Enabling Commenter Links'; code-example-end]

Gdy podany zostanie ten adres URL, konto użytkownika zostanie zaktualizowane, a wszystkie jego nazwy użytkownika we wszystkich przeszłych i przyszłych komentarzach będą odnośnikami do tego URL.

Można to dostosować bez kodu, na stronie dostosowywania widgetu:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.click-to-show-comments', '.commenter-links']; selector = '.commenter-links'; alt='Strona dostosowywania widgetu z zaznaczonym polem wyboru linków komentującego, aby dodać pole adresu URL strony internetowej do formularza komentarza'; title='Włączanie linków komentującego' app-screenshot-end]

---
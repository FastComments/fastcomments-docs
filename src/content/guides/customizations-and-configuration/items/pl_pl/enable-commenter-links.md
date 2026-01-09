[related-parameter-start name = 'enableCommenterLinks'; type = 'boolean'; related-parameter-end]

Domyślnie FastComments poprosi użytkownika tylko o komentarz, nazwę użytkownika i adres e-mail.

Jednak w niektórych sytuacjach możesz chcieć, aby użytkownik zostawił link do własnego bloga lub strony internetowej.

Możemy włączyć wyświetlanie dodatkowego pola do wprowadzenia adresu URL strony użytkownika, ustawiając flagę **enableCommenterLinks** na true:

[code-example-start config = {enableCommenterLinks: true}; linesToHighlight = [6]; title = 'Enabling Commenter Links'; code-example-end]

Po podaniu tego adresu URL konto użytkownika zostanie zaktualizowane, a nazwa użytkownika we wszystkich wcześniejszych i przyszłych komentarzach będzie prowadzić do tego adresu URL.

To można dostosować bez użycia kodu, na stronie dostosowywania widżetu:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.click-to-show-comments', '.commenter-links']; selector = '.commenter-links'; title='Enabling Commenter Links' app-screenshot-end]

---
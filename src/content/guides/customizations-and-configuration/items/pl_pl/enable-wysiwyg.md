[related-parameter-start name = 'enableWYSIWYG'; type = 'boolean'; related-parameter-end]

Domyślnie funkcje formatowania w FastComments są realizowane przez dodawanie widocznych tagów kotwic, takich jak `<b></b>` wokół Twojego tekstu. Klikanie paska narzędzi
lub używanie skrótów klawiaturowych robi to za Ciebie. Jednak niektóre społeczności mogą chcieć korzystać z formatowania bez tagów kotwic. Nazywa się to włączeniem
edytora WYSIWYG (what you see is what you get). Ten edytor wygląda dokładnie tak samo jak domyślny, z wyjątkiem tego, że ładuje dodatkowy
kod, który pozwala użytkownikom pogrubiać, podkreślać itp. ich tekst bez widocznych tagów kotwic.

[code-example-start config = {enableWYSIWYG: true}; linesToHighlight = [6]; title = 'Enabling WYSIWYG Editing'; code-example-end]

Można to również zrobić bez kodu. Na stronie dostosowywania widżetu, zobacz opcję „Enable Advanced Formatting”.

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.enable-wysiwyg']; selector = '.enable-search-label'; alt='Strona dostosowywania widżetu z zaznaczonym polem wyboru „Enable Advanced Formatting”, aby włączyć edytor WYSIWYG'; title='Włącz WYSIWYG' app-screenshot-end]
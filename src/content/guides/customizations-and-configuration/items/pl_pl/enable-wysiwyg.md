[related-parameter-start name = 'enableWYSIWYG'; type = 'boolean'; related-parameter-end]

Domyślnie funkcje formatowania w FastComments są realizowane przez dodawanie widocznych znaczników kotwic, takich jak `<b></b>`, wokół twojego tekstu. Kliknięcie paska narzędzi
lub użycie skrótów wykonuje to za Ciebie. Jednak niektóre społeczności mogą chcieć włączyć korzystanie z formatowania bez widocznych znaczników kotwic. Nazywa się to włączeniem
edytora WYSIWYG (co widzisz, to dostajesz). Ten edytor wygląda dokładnie tak samo jak domyślny, z wyjątkiem tego, że ładuje pewien
dodatkowy kod, który pozwala użytkownikom pogrubiać, podkreślać itp. tekst bez widocznych znaczników kotwic.

[code-example-start config = {enableWYSIWYG: true}; linesToHighlight = [6]; title = 'Enabling WYSIWYG Editing'; code-example-end]

Można to również zrobić bez kodu. Na stronie dostosowywania widżetu zobacz opcję "Włącz zaawansowane formatowanie".

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.enable-wysiwyg']; selector = '.enable-search-label'; title='Enable WYSIWYG' app-screenshot-end]
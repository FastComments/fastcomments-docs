[related-parameter-start name = 'enableWYSIWYG'; type = 'boolean'; related-parameter-end]

Per impostazione predefinita, le funzionalità di formattazione in FastComments vengono eseguite aggiungendo tag di ancoraggio visibili come `<b></b>` attorno al testo. Cliccando la barra degli strumenti
o usando le scorciatoie fa questo per te. Tuttavia, alcune community potrebbero voler optare per l'uso della formattazione senza tag di ancoraggio. Questo si chiama abilitare il
editor WYSIWYG (ciò che vedi è ciò che ottieni). Questo editor appare esattamente uguale a quello predefinito, eccetto che carica del
codice extra che permette agli utenti di mettere in grassetto, sottolineare, ecc. il loro testo senza tag di ancoraggio visibili.

[code-example-start config = {enableWYSIWYG: true}; linesToHighlight = [6]; title = 'Enabling WYSIWYG Editing'; code-example-end]

This can also be done without code. In the widget customization page, see the "Abilita formattazione avanzata" option.

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.enable-wysiwyg']; selector = '.enable-search-label'; title='Enable WYSIWYG' app-screenshot-end]
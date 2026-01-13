[related-parameter-start name = 'enableSpoilers'; type = 'boolean'; related-parameter-end]

Podporo za spojlerje lahko omogočite tako, da nastavite zastavico **enableSpoilers** na true:

[code-example-start config = {enableSpoilers: true}; linesToHighlight = [6]; title = 'Enabling Spoilers'; code-example-end]

To je mogoče narediti tudi brez kode. Na strani za prilagajanje pripomočka si oglejte možnost "Omogoči spojlerje".

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.enable-spoilers']; selector = '.enable-spoilers'; title='Enable Spoilers' app-screenshot-end]

Ko je besedilo označeno, in se klikne zdaj vidni gumb `SPOILER`, bo besedilo zatemnjeno, dokler uporabnik nanj ne premakne miške. Za temni način naredimo enako, z drugačnimi
barvami, ki se bolje ujemajo s temnim načinom.

To je tudi združljivo z urejevalnikom WYSIWYG.
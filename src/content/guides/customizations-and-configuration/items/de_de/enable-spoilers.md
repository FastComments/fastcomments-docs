[related-parameter-start name = 'enableSpoilers'; type = 'boolean'; related-parameter-end]

Wir können die Spoiler-Unterstützung aktivieren, indem wir das Flag **enableSpoilers** auf true setzen:

[code-example-start config = {enableSpoilers: true}; linesToHighlight = [6]; title = 'Enabling Spoilers'; code-example-end]

Dies kann auch ohne Code gemacht werden. Auf der Seite zur Anpassung des Widgets finden Sie die Option "Spoiler aktivieren".

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.enable-spoilers']; selector = '.enable-spoilers'; title='Enable Spoilers' app-screenshot-end]

Wenn Text markiert ist und der nun sichtbare `SPOILER`-Button angeklickt wird, wird der Text verborgen, bis der Benutzer mit der Maus darüber fährt. Im Dunkelmodus machen wir dasselbe, mit anderen
Farben, die besser zum Dunkelmodus passen.

Dies ist auch mit dem WYSIWYG-Editor kompatibel.

---
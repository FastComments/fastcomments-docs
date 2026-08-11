---
[related-parameter-start name = 'enableSpoilers'; type = 'boolean'; related-parameter-end]

Wir können die Spoiler‑Unterstützung aktivieren, indem wir das **enableSpoilers**‑Flag auf true setzen:

[code-example-start config = {enableSpoilers: true}; linesToHighlight = [6]; title = 'Spoiler aktivieren'; code-example-end]

Dies kann auch ohne Code durchgeführt werden. Auf der Widget‑Anpassungsseite finden Sie die Option "Enable Spoilers".

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.enable-spoilers']; selector = '.enable-spoilers'; alt='Widget‑Anpassungsseite mit dem aktivierten Enable Spoilers‑Kontrollkästchen, um die SPOILER‑Schaltfläche zum Editor hinzuzufügen'; title='Spoiler aktivieren' app-screenshot-end]

Wenn Text markiert wird und die nun sichtbare `SPOILER`‑Schaltfläche geklickt wird, wird der Text maskiert, bis der Benutzer mit der Maus darüber fährt. Im Dark‑Mode machen wir dasselbe, jedoch mit anderen Farben, die besser zum Dark‑Mode passen.

Dies ist auch mit dem WYSIWYG‑Editor kompatibel.
---
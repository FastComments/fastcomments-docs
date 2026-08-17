[related-parameter-start name = 'defaultAvatarSrc'; type = 'string'; related-parameter-end]

Wenn ein Benutzer zum ersten Mal mit FastComments kommentiert, versuchen wir, sein Avatar von <a href="https://gravatar.com/" target="_blank">http://gravatar.com/</a> abzurufen.

Falls wir jedoch kein Avatar finden oder der Benutzer nie eines in seinem Konto festlegt, zeigen wir ein statisches Standard-Avatar-Bild an.

Um ein eigenes statisches Avatar-Bild anzugeben, können wir die Einstellung *defaultAvatarSrc* verwenden.

[code-example-start config = {defaultAvatarSrc: "https://example.com/some-image.png"}; linesToHighlight = [6]; title = 'Standard-Avatar überschreiben'; code-example-end]

Dies kann auch ohne Code durchgeführt werden. Auf der Widget-Anpassungsseite finden Sie den Abschnitt „Standard-Avatar“.

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.default-avatar'; alt='Standard-Avatar-Abschnitt der Widget-Anpassungsseite, wo Sie die URL des Ersatz-Avatar-Bildes festlegen'; title='Anpassen des Standard-Avatars' app-screenshot-end]

Beachten Sie, dass das Festlegen des Avatars für einen bestimmten Benutzer, beispielsweise mit SSO, in einem eigenen Abschnitt behandelt wird.
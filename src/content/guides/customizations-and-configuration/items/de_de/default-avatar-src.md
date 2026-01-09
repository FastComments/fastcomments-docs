[related-parameter-start name = 'defaultAvatarSrc'; type = 'string'; related-parameter-end]

Wenn ein Benutzer zum ersten Mal mit FastComments kommentiert, versuchen wir, sein Avatar von <a href="http://gravatar.com/" target="_blank">http://gravatar.com/</a> abzurufen.

Wenn wir jedoch kein Avatarbild finden oder der Benutzer nie eines in seinem Konto einstellt, zeigen wir ein statisches Standard-Avatarbild an.

Um ein eigenes statisches Avatarbild anzugeben, können wir die Einstellung *defaultAvatarSrc* verwenden.

[code-example-start config = {defaultAvatarSrc: "https://example.com/some-image.png"}; linesToHighlight = [6]; title = 'Override The Default Avatar'; code-example-end]

Dies kann auch ohne Code erfolgen. Auf der Seite zur Anpassung des Widgets siehe den Abschnitt "Default Avatar".

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.default-avatar'; title='Customizing The Default Avatar' app-screenshot-end]

Beachten Sie, dass die Festlegung des Avatars für einen bestimmten Benutzer, z. B. bei SSO, in einem eigenen Abschnitt behandelt wird.

---
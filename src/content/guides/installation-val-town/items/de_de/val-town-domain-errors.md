Sobald Sie den `demo`-Mandanten ausschalten, kann das Widget mit einem Autorisierungsfehler die Ladeverweigerung zeigen. Das liegt daran, dass FastComments nicht weiß, dass Ihr Konto auf dieser Domain verwendet werden darf.

[Gehen Sie hier, um Ihre Site zu Ihrem Konto hinzuzufügen.](https://fastcomments.com/auth/my-account/configure-domains)

Val Town lohnt hier einen zweiten Blick, weil ein Val über mehr als einen Hostnamen erreichbar sein kann:

- Jeder HTTP-Val hat einen langen Standard-Endpunkt, `<org>--<id>.web.val.run`.
- Das Beanspruchen einer benutzerdefinierten Subdomain fügt `<name>.val.run` hinzu.
- Eine [benutzerdefinierte Domain](https://docs.val.town/vals/http/custom-domains/) fügt eine dritte hinzu.
- Branches erhalten eigene URLs.

Fügen Sie die Hostnamen hinzu, von denen Sie das Widget tatsächlich bereitstellen. Wenn Sie nach der Einrichtung eine Subdomain beanspruchen, fügen Sie diese ebenfalls hinzu, sonst funktioniert das Widget nur mit der alten URL und schlägt bei der neuen fehl.
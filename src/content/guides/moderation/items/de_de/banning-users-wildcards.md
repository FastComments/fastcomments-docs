Es ist möglich, Benutzer bestimmter E-Mail-Anbieter mithilfe von Wildcards zu sperren.

Wenn Sie beispielsweise feststellen, dass alle Kommentare von **@bademail.com** Spam sind, können Sie einfach den gesamten E-Mail-Anbieter sperren, indem Sie "*@bademail.com" in das E-Mail-Eingabefeld beim Hinzufügen eines gesperrten Benutzers eingeben.

Beachten Sie das "*" vor dem @ in der E-Mail-Adresse.

### Subdomains

Eine Domain-Sperre gilt auch für jede Subdomain dieser Domain. Das Sperren von `*@bademail.com` sperrt auch
`someone@mail.bademail.com` und `someone@eu.mail.bademail.com`, sodass es nicht erforderlich ist, für jede Subdomain eine separate Sperre hinzuzufügen.

Wenn Sie nur eine bestimmte Subdomain sperren möchten, geben Sie stattdessen diese Subdomain ein, zum Beispiel `*@mail.bademail.com`. Diese Sperre
wirkt sich nicht auf `someone@bademail.com` aus.

### Sperren einer Domain über einen Kommentar

Sie müssen das Muster nicht selbst eingeben. Wenn Sie einen Benutzer über einen Kommentar auf der Seite „Kommentare moderieren" sperren, enthält der Sperrdialog
ein Kontrollkästchen „Alle @domain-Benutzer sperren", das dieselbe `*@domain`-Sperre für die E-Mail-Domain des Kommentators erstellt.

### Unterstützte Muster

Die einzige unterstützte Wildcard-Form ist ein einzelnes `*` anstelle des gesamten Namensanteils, gefolgt von `@` und einer Domain. Andere Formen
werden beim Speichern abgelehnt:

- `*@*.bademail.com` ist nicht erforderlich, da `*@bademail.com` bereits Subdomains abdeckt.
- `name*@bademail.com` und `*bademail.com` werden nicht unterstützt.
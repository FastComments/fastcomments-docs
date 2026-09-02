---
Es ist möglich, Benutzer, die bestimmte E‑Mail‑Provider verwenden, mit Platzhaltern zu sperren.

Zum Beispiel, wenn Sie feststellen, dass alle Kommentare von **@bademail.com** Spam sind, können Sie den gesamten E‑Mail‑Provider einfach sperren, indem Sie "*@bademail.com" in das E‑Mail‑Eingabefeld eingeben, wenn Sie einen gesperrten Benutzer hinzufügen.

Beachten Sie das "*" vor dem @ in der E‑Mail.

### Subdomains

Ein Domain‑Verbot deckt auch jede Subdomain dieser Domain ab. Das Sperren von `*@bademail.com` sperrt ebenfalls `someone@mail.bademail.com` und `someone@eu.mail.bademail.com`, sodass es nicht nötig ist, für jede Subdomain ein separates Verbot hinzuzufügen.

Wenn Sie nur eine bestimmte Subdomain sperren möchten, geben Sie stattdessen diese Subdomain ein, zum Beispiel `*@mail.bademail.com`. Dieses Verbot wirkt sich nicht auf `someone@bademail.com` aus.

### Banning a Domain From a Comment

Sie müssen das Muster nicht selbst eingeben. Wenn Sie einen Benutzer von einem Kommentar auf der Seite "Moderate Comments" sperren, enthält das Sperr‑Dialogfeld ein Kontrollkästchen "Ban All @domain Users", das das gleiche `*@domain` Verbot für die E‑Mail‑Domain des Kommentators erstellt.

### Supported Patterns

Die einzige unterstützte Platzhalter‑Form ist ein einzelnes `*` anstelle des gesamten Namensanteils, gefolgt von `@` und einer Domain. Andere Formen werden abgelehnt, wenn Sie versuchen, sie zu speichern:

- `*@*.bademail.com` ist nicht nötig, weil `*@bademail.com` bereits Subdomains abdeckt.
- `name*@bademail.com` und `*bademail.com` werden nicht unterstützt.
---
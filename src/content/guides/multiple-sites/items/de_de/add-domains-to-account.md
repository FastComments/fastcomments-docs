FastComments authentifiziert Anfragen an Ihr Konto, um zu prüfen, dass sie von Ihrer Website stammen. Deshalb müssen wir wissen, welche Website(s) Sie FastComments installieren möchten.

FastComments unterstützt die Authentifizierung über Domains sowie Subdomains.

Nehmen wir die Seite `https://example.com`. In diesem Fall ist "`example.com`" die Domain. `example.com` unterstützt sowohl `example.com` als auch `www.example.com`. Wir nennen das „www“ die „Subdomain“.

Beispiel:

- Um nur `blog.example.com` zu erlauben:
  - Fügen Sie `blog.example.com` zu Ihren Domains hinzu.
- Um `www.example.com`, `somesite.example.com` und `example.com` zu erlauben:
  - Fügen Sie `example.com` zu Ihren Domains hinzu.
  - Dies wird als **eine Domain** abgerechnet, die mit Ihrem Konto verknüpft ist.
- Sie können jetzt Wildcard-Subdomains hinzufügen, zum Beispiel *myname.vercel.app.
  - Dies wird als **eine Domain** abgerechnet, die mit Ihrem Konto verknüpft ist.

Wenn Sie eine Blogging-Plattform verwenden und Ihnen eine Subdomain zugewiesen wurde, sollten Sie die **vollständige Domain einschließlich der Subdomain** zu Ihrem Konto hinzufügen, zum Beispiel: `cats.blogger.com`.

Wir können Domains zu unserem Konto hinzufügen, indem wir die Seite `My Domains` aufrufen und unten auf `Add a Domain` klicken:

[app-screenshot-start url='/auth/my-account/configure-domains'; selector = '.content'; alt='My Domains Seite, die die Domains im Konto auflistet, mit dem Button „Add a Domain“ am unteren Rand'; title='Die My Domains Seite' app-screenshot-end]

Während der Testphase werden **Domains automatisch zu Ihrem Konto hinzugefügt**, wenn Anfragen von diesen Domains kommen. Nach Ablauf dieser Zeit müssen sie aus Sicherheitsgründen explizit hinzugefügt werden. Sie erhalten eine E‑Mail, wenn dieses automatisierte Verhalten eintritt.

Sie müssen **nicht** `localhost` für die lokale Entwicklung hinzufügen – es ist standardmäßig erlaubt.

#### Über die API

Domains können auch hinzugefügt und konfiguriert werden [über die DomainConfigs API](/guide-api.html#domain-config-structure).
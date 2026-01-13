---
Für Entwickler, die Sie möglicherweise nicht zu `Administrators` machen möchten, sollten Sie einen `Administrator`-Benutzer mit den folgenden Berechtigungen anlegen:

1. Analytics Admin
2. Customizations Admin
3. Data Management Admin
4. Comment Moderation Admin
5. API/SSO Admin

Dieses Berechtigungsset gibt einem Entwickler alles, was er benötigt, um FastComments einzurichten, sowie die Einsicht in das System, die erforderlich ist, um sicherzustellen, dass es funktioniert.

Die Gründe für diese Berechtigungen sind wie folgt:

1. **Analytics Admin**: Dies kann verwendet werden, um die Nutzung von FastComments zu debuggen.
2. **Customizations Admin**: Dies wird benötigt, um benutzerdefinierte Stile auf das Kommentar-Widget anzuwenden.
3. **Data Management Admin**: Dies wird benötigt, um Importe und Exporte zu verwalten und Webhooks einzurichten.
4. **Comment Moderation Admin**: Dies wird benötigt, um Kommentardaten einzusehen, zumindest während der Einrichtung.
5. **API/SSO Admin**: Dies ermöglicht ihnen, die API keys direkt von unserer Plattform abzurufen. Wir halten dies für sicherer, als wenn ein `Administrator` sie für sie kopiert und den API Secret per E-Mail versendet, was möglicherweise nicht sehr sicher ist.

---
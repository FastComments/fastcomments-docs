Eine Flex-Rechnung ist eine Liste von Positionen, eine pro Nutzungsart, wobei jede die Menge und den Preis zeigt, zum Beispiel `213,000 Page Loads. ($1.00 per 100,000 Page Loads)`. Die Preise finden Sie auf der [Preisseite](https://fastcomments.com/traffic-pricing) und auf Ihrer [Billing Info](https://fastcomments.com/auth/my-account/billing-info)-Seite. Dieser Abschnitt erklärt, was jede Menge bedeutet.

### Über den Monat gezählt

Diese Werte werden jeden Monat zurückgesetzt und vom ersten bis zum letzten Tag des Kalendermonats (UTC) gezählt:

- **Page Loads**: die Anzahl der Male, wie ein Kommentar-Widget oder Feed auf Ihren Seiten geladen wurde. Jedes Laden des Kommentar-Widgets zählt, unabhängig davon, wie viele Kommentare es anzeigt. Es gibt keine Gebühr pro Kommentar-Thread oder pro Seite.
- **API Credits**: Aufrufe der REST-API mit Ihrem API-Schlüssel.
- **Small Widgets Credits**: Aufrufe der kleinen Widgets, wie Kommentarzahlen und Badges.
- **LLM Tokens**: Tokens, die von [AI Agents](/guide-ai-agents.html) und anderen LLM-Funktionen verwendet werden.

### Am Ende des Monats gezählt

Dies sind Momentaufnahmen, die bei der Erstellung der Rechnung aufgenommen werden:

- **Comments Stored**: die Gesamtzahl der Kommentare in Ihrem Konto, nicht nur die in diesem Monat hinzugefügten. Das Löschen von Kommentaren reduziert diesen Wert, wobei ein gelöschter Kommentar mit Antworten als Platzhalter erhalten bleibt, damit die Antworten weiterhin sinnvoll sind.
- **SSO Users**: die Gesamtzahl der SSO-Benutzer, die Ihre Seite erstellt hat, einschließlich Personen, die vor langer Zeit einmal kommentiert haben. Wenn dies der größte Teil Ihrer Rechnung ist, fragen Sie den Support nach Abrechnung nach monatlich aktiven Benutzern, wobei nur Benutzer zählen, die sich im Monat angemeldet haben.
- **Moderators**, **Admins**, **SSO Moderators** und **SSO Admins**: Sitze, behandelt in [Admin and Moderator Seats](#admin-and-moderator-seats). Der erste Admin ist kostenlos.
- **Domains**: die Seiten in Ihrem Konto. Die erste Seite ist kostenlos und jede weitere wird monatlich abgerechnet, solange sie im Konto ist. Eine Subdomain zählt nur, wenn Sie sie als eigene Seite hinzufügen; eine einzelne Top-Level-Domain deckt ihre Subdomains ab. Siehe [Multiple Sites](/guide-multiple-sites.html#domain-limits).
- **Managed Tenants**: Unterkonten, die von diesem Konto verwaltet werden, falls vorhanden, plus deren eigene Nutzung als separate Positionen.

### Minimum

Wenn die Positionen zusammen weniger als das Flex-Minimum ergeben, wird die Rechnung auf das Minimum aufgestockt. Eine kleine Seite mit einem Admin und einer Domain zahlt nur das Minimum.

### Vorschau der Rechnung

Die [Billing Analytics](https://fastcomments.com/auth/my-account/analytics/billing)-Seite zeigt jede Position der nächsten Rechnung mit den heutigen Zahlen, sodass es keine Überraschungen gibt. Die Page-Load-, API- und Small-Widget-Zahlen können bis zu einer Minute hinterher sein; der Rest ist live.

### Andere Währungen

Die Flex-Nutzung wird in US-Dollar abgerechnet. Wenn Ihre Rechnungswährung eine andere ist, wird jede Position zum Wechselkurs des Tages umgerechnet, an dem die Rechnung erstellt wird, sodass dieselbe Nutzung von Monat zu Monat leicht unterschiedliche Beträge kosten kann. Siehe [Currencies](#currencies).
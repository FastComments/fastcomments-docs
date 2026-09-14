[Val Town](https://val.town) führt TypeScript auf Deno aus, sodass ein Val ein echter Server ist. Das macht es zu einer guten Wahl für FastComments: Das Widget ist ein Script‑Tag auf der Seite, und alles, was ein Geheimnis benötigt, wie Secure SSO oder die Verifizierung eines Webhooks, kann serverseitig im selben Val ausgeführt werden.

Dieser Leitfaden behandelt das Hinzufügen des Kommentar‑Widgets zu einem HTTP‑Val, das Anzeigen von Kommentarzahlen auf einer Indexseite, das Anmelden von Benutzern mit dem bereits vorhandenen Val‑Town‑Konto und das Empfangen von Kommentar‑Webhooks.

Sie benötigen kein Konto, um es auszuprobieren. Die Beispiele verwenden `tenantId: "demo"`, eine gemeinsam genutzte Sandbox, und Schritt 2 behandelt das Wechseln zu Ihrem eigenen.
Canvas-Rollen werden während des LTI-Starts automatisch auf FastComments-Rollen abgebildet. Keine manuelle Konfiguration erforderlich.

#### Rollenabgleich

| Canvas Rolle | FastComments Rolle | Berechtigungen |
|---|---|---|
| **Administrator** | Admin | Voller Kontozugriff, alle Kommentare und Einstellungen verwalten |
| **Instructor** | Moderator | Kommentare bearbeiten und löschen, Threads anheften, Diskussionen verwalten |
| **Learner** | Commenter | Kommentare posten, antworten, abstimmen und Erwähnungen verwenden |

#### Wie es funktioniert

Wenn ein Benutzer FastComments aus Canvas startet, enthält das LTI 1.3-Protokoll seine Canvas-Rolle. FastComments liest diese Rolle und weist automatisch die entsprechenden Berechtigungen zu.

Wenn ein Benutzer mehrere Rollen hat (z. B. ein Instructor, der gleichzeitig Administrator ist), wird die Rolle mit den höchsten Rechten verwendet.

---
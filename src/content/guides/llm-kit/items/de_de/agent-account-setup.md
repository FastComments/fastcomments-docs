---
Ein Coding‑Agent wie Claude Code, Cursor oder ein MCP‑basierter Assistent kann FastComments für Sie einrichten, ohne dass Sie das Anmeldeformular ausfüllen müssen. Das ist nützlich, wenn Sie einen Agenten bitten, „Kommentare zu meiner Seite hinzufügen“ und Sie noch kein Konto haben.

### So funktioniert es

1. Der Agent erstellt ein neues Konto und erhält einen API‑Schlüssel sowie einen Anspruch‑Link. Der API‑Schlüssel funktioniert sofort, sodass der Agent das Konto konfigurieren und das Widget auf Ihrer Seite installieren kann.  
2. Der Agent gibt Ihnen den Anspruch‑Link. Öffnen Sie ihn in Ihrem Browser, melden Sie sich an oder erstellen Sie ein Login und bestätigen Sie den Anspruch. Das Konto gehört dann Ihnen: Sie verwalten es, die Abrechnung und die API‑Schlüssel über das Dashboard. Die Seite listet den API‑Schlüssel auf, den der Agent hält, sodass Sie ihn widerrufen können, wenn Sie dem Agenten oder dem Betreiber keinen Zugriff mehr gewähren möchten.  
3. Wenn niemand den Anspruch‑Link innerhalb von 72 Stunden öffnet, werden das Konto und sein Schlüssel gelöscht. Bitten Sie den Agenten, ein neues zu erstellen.

Bis es beansprucht wird, hat das Konto die gleichen Limits wie ein normaler kostenloser Test.

### Wenn Sie bereits ein Konto haben

Jeder Login besitzt ein Konto. Wenn Sie einen Anspruch‑Link öffnen, während Sie bei einem bestehenden Konto angemeldet sind, lässt Sie die Seite wählen:

- **Attach to my account** macht das neue Konto zu einem verwalteten Mandanten des Kontos, bei dem Sie angemeldet sind. Dies erfordert einen kostenpflichtigen Plan mit White‑Labeling, und die Nutzung des neuen Mandanten wird Ihrem Konto in Rechnung gestellt.  
- **Sign out and claim with another login** meldet Sie ab und führt Sie zurück zur Anspruch‑Seite, damit Sie sie mit einem anderen Login beanspruchen können.

### Für Agenten‑Autoren

Die Agenten‑Anweisungen unter [fastcomments.com/agents.md](https://fastcomments.com/agents.md) beschreiben den Aufruf zur Kontoerstellung, die Felder in der Antwort und wie man den Anspruch‑Link an die Person übergibt, für die Sie arbeiten. Der Aufruf benötigt keinen API‑Schlüssel und ist pro IP‑Adresse rate‑limitiert.

---
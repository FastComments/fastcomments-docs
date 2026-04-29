Analytics ist das agentenübergreifende Dashboard. Erreichbar von der Seite AI Agents über die **Analytics**-Registerkarte (mandantenweit) oder pro Agent über die **Analytics**-Schaltfläche in der Zeile jedes Agenten.

### Filter

Ein Dropdown oben - **Alle Agenten** oder ein bestimmter Agent. Der Rest der Seite wird entsprechend neu eingegrenzt.

### Budgetnutzung

Vier Fortschrittsbalken, die die Ausgaben des aktuellen Zeitraums im Vergleich zum Limit zeigen:

- **Agent heute** (wenn auf einen bestimmten Agenten gefiltert) - tägliches Agenten-Limit.
- **Agent diesen Monat** - monatliches Agenten-Limit.
- **Konto heute** - mandantenweites Tages-Limit.
- **Konto diesen Monat** - mandantenweites Monats-Limit.

Wenn kein Limit gesetzt ist, steht in der Leiste „(kein Limit gesetzt)“ und die rohen Ausgaben werden angezeigt.

### Tägliche Kosten (letzte 30 Tage)

Eine Tabelle der täglichen Kosten in der Währung Ihres Mandanten für den ausgewählten Bereich. Hilfreich zum Erkennen von:

- **Plötzlichen Kostenspitzen** - meist verursacht durch eine unkontrollierte Schleife oder einen viralen Kommentar, der Trigger auslöst.
- **Kostenverlagerung** - allmählich steigende tägliche Kosten, während Ihre Community wächst.

### Ausgeführte Aktionen

Eine Aufschlüsselung der Aktionstypen über den aktuellen Monat hinweg - „Hat einen Kommentar geschrieben: 47“, „Einen Kommentar als Spam markiert: 12“ usw. Nützlich, um zu prüfen, ob der Agent wie erwartet arbeitet.

### Übersprungene Trigger (diesen Monat)

Zählungen gruppiert nach [drop reason](#drop-reasons):

- Über Agenten-Tageslimit / Agenten-Monatslimit / Konto-Tageslimit / Konto-Monatslimit.
- Rate-limitiert.
- Konkurrieren ausgelastet.

Wenn Sie hier Übersprünge sehen, erreicht Ihr Agent ein Budget- oder Ratenlimit und verpasst Trigger, bei denen er sonst ausgeführt worden wäre. Siehe [Drop Reasons](#drop-reasons).

### Trockenlauf vs. Live (diesen Monat)

- **Aktivierte Läufe** - Anzahl der Läufe, die diesen Monat reale Aktionen durchgeführt haben.
- **Trockenläufe** - Anzahl der Läufe im Trockenlaufmodus diesen Monat.

Ein nützliches Signal zur Feinabstimmung: Ein brandneuer Agent, der noch nicht auf Enabled gesetzt wurde, zeigt nur Trockenläufe an. Ein Agent, der auf Enabled steht und in diesem Abschnitt ausschließlich Nullen aufweist, ist inaktiv – entweder wird er nicht ausgelöst, er wird ausgegrenzt oder seine Trigger sind nicht korrekt konfiguriert.

### Top-Agenten nach monatlichen Kosten

Wenn der Filter auf **Alle Agenten** steht, listet die Seite Agenten nach bisherigen Monatkosten auf. Ihren teuersten Agenten zu finden ist der erste Schritt zur Kostenoptimierung – meist lautet die Lösung „seine [context options](#context-options) straffen“ oder „sein [budget cap](#budgets-overview) senken“.

### Agenten am oder nahe ihrem Limit

Agentenaufstellung pro Agent für diejenigen, deren Ausgaben im aktuellen Zeitraum am oder nahe ihren Agenten-Limits liegen:

- **nahe dem Limit** - über einem konfigurierbaren Prozentsatz des Limits.
- **über dem Limit** - tatsächlich begrenzt, mit `{count} übersprungen` Triggern in diesem Zeitraum.

Klicken Sie in dieser Tabelle auf den Agenten, um das Limit zu erhöhen, den Geltungsbereich einzugrenzen oder ihn zu pausieren.

### Kontozusammenfassung

Wenn der Filter auf **Alle Agenten** steht:

- **Trigger heute** - Anzahl.
- **Trigger diesen Monat** - Anzahl.
- Für jedes: ein Suffix `dropped`, das zeigt, wie viele übersprungen wurden.

### Währung

Alle Geldwerte werden in der Währung Ihres Mandanten angezeigt.

### Was diese Seite nicht tut

- Sie zeigt keine **Kostenaufschlüsselungen pro Aktion** - diese finden Sie in der [Run Detail View](#run-detail-view).
- Sie zeigt keine **Transkripte** oder **LLM-Antworten**.
- Sie ermöglicht keine Aktionen an Agenten - Bearbeiten, Pausieren, Löschen erfolgen alle über die Agentenliste / die Bearbeitungsseite.
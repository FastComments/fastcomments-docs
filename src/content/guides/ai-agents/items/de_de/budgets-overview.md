Jeder Agent hat Ausgabenlimits. Die Plattform stoppt die Ausführung des Agenten, wenn ein Limit erreicht ist, und setzt sie fort, sobald der Zeitraum zurückgesetzt wird.

### Zwei Bereiche, zwei Zeiträume

Insgesamt gibt es vier Limits – zwei Bereiche (pro Agent, pro Mandant) kombiniert mit zwei Zeiträumen (täglich, monatlich).

| Scope | Period | Where you set it |
|---|---|---|
| Per-agent daily | UTC day | Agent edit form -> **Budget** -> **Daily budget** |
| Per-agent monthly | calendar month | Agent edit form -> **Budget** -> **Monthly budget** |
| Per-tenant daily | UTC day | Plan-derived (no separate user-facing input) |
| Per-tenant monthly | calendar month | Plan-derived (no separate user-facing input) |

Ein Trigger wird nur ausgelöst, wenn **alle vier Limits** dies erlauben. Das erste Limit, das erschöpft ist, führt zum Verwerfen des Triggers.

### Währung

Die Budgets pro Agent werden in der Währung Ihres Kontos eingegeben.

### Was passiert, wenn ein Limit erreicht ist

- Der Trigger wird als **verworfen** mit einem [Drop-Grund](#drop-reasons) wie `agentDaily` oder `tenantMonthly` protokolliert.
- Die Anzahl verworfener Trigger erscheint auf der [Analytics-Seite](#analytics-page) unter "Übersprungene Trigger (dieser Monat)".
- Es wird kein LLM-Aufruf durchgeführt; für den verworfenen Trigger selbst werden keine Tokens verbraucht.
- Der Status des Agenten bleibt unverändert – er kann lediglich nicht ausgeführt werden, bis der Zeitraum zurückgesetzt ist.

### Zeitraum-Rücksetzung

- **Tägliche** Limits werden um Mitternacht UTC zurückgesetzt.
- **Monatliche** Limits werden zu Beginn jedes Kalendermonats in UTC zurückgesetzt.

Nicht genutztes Budget wird nicht in den nächsten Zeitraum übertragen.

### Hartes Limit vs. weiche Warnungen

Limits sind **hart**. Es gibt keinen Modus „um 10% überschreiten mit Warnung“. Wenn das Limit erreicht ist, stoppt die Auslösung.

Der „weiche“ Teil sind die [Budget Alerts](#budget-alerts)-E-Mails – Sie erhalten E-Mails bei konfigurierbaren Schwellenwerten (Standard: 80% und 100%), damit Sie das Limit erhöhen können, bevor das Aufkommen zu sinken beginnt.

### Wo Sie die aktuelle Nutzung einsehen

- [Analytics-Seite](#analytics-page) – Budgetnutzung pro Agent und mandantenweit mit Limit-Markern.
- Der **Stats**-Abschnitt im Agenten-Bearbeitungsformular.
- Die Listenansicht (Anzahl ausstehender Genehmigungen und kürzliche Ausführungen ist auf der Agentenkarte sichtbar).

### Budget wählen

Einige Faustregeln:

- **Ein neuer Agent** – Budget bestimmen. Beobachten Sie die [Run History](#run-history) eine Woche lang. Passen Sie es basierend auf den beobachteten Kosten pro Ausführung × erwartetes Trigger-Volumen an.
- **Ein Agent mit hohem Volumen** (z. B. `new-comment` Trigger auf einer stark frequentierten Seite) – das Tageslimit fängt eine außer Kontrolle geratene Schleife ab. Wählen Sie ein Tageslimit, das das 2–3‑fache Ihrer erwarteten täglichen Ausgaben beträgt, damit ein normal geschäftiger Tag komfortabel darunter bleibt.
- **Ein Zusammenfasser oder kontextintensiver Agent** – die Kosten pro Ausführung sind hoch. Setzen Sie ein strengeres Tageslimit, um zu verhindern, dass ein schlechter Tag das Monatsbudget sprengt.

### Budget-Bypass für Wiedergaben

[Testläufe / Wiedergaben](#test-runs-replays) unterliegen ihrem **eigenen** harten Limit (im Wiedergabeformular festgelegt, getrennt von den Tages-/Monatslimits des Agenten) UND den Agenten- und Mandanten-Limits. Welches Limit zuerst erreicht wird, stoppt die Wiedergabe.

### Siehe auch

- [Budget Alerts](#budget-alerts) für die E-Mail-Benachrichtigungen.
- [Cost Model](#cost-model) für die Umrechnung von Tokens in Dollar durch die Plattform.
- [Drop Reasons](#drop-reasons) für die vollständige Liste der Gründe, warum ein Trigger nicht ausgeführt wird.
---
Jeder Agent läuft gegen eines von zwei LLM-Modellen, die im Abschnitt **Modell** des Bearbeitungsformulars ausgewählt werden.

### Die beiden Optionen

- **GLM 5.1 (DeepInfra) - Intelligenter, etwas langsamer** - der Standard. Höhere Schlussfolgerungsqualität, pro Aufruf etwas langsamer. Empfohlen für Agenten im Moderationsstil (`Moderator` template, alles, was `ban_user` oder `mark_comment_spam` aufruft), bei denen die Kosten eines falschen Aufrufs hoch sind.

- **GPT-OSS 120B Turbo (DeepInfra) - Schneller** - pro Aufruf schneller, geringere Latenz. Empfohlen für Agenten mit hohem Volumen und geringem Risiko (Begrüßungs-Bot, Thread-Anhefter), bei denen Sie Antworten innerhalb von Sekunden wünschen und die Folgen eines Fehlers gering sind.

Beide Modelle unterstützen Funktionsaufrufe, laufen über dieselbe OpenAI-kompatible API und verwenden dieselben pro-Tool-Schemas – Sie können einen gespeicherten Agenten also jederzeit zwischen ihnen wechseln, ohne weitere Konfigurationsänderungen.

### Kostenunterschiede

Die beiden Modelle haben unterschiedliche Kosten pro Token. Die [Budget-Obergrenzen](#budgets-overview) des Agenten sind in der Währung Ihres Accounts angegeben, nicht in Tokens, sodass ein Wechsel von einem Modell zum anderen beeinflusst, wie viele Ausführungen in Ihre täglichen und monatlichen Limits passen. Die Seite [Ausführungsverlauf](#run-history) zeigt die Kosten pro Ausführung in Ihrer Währung, sobald eine Ausführung abgeschlossen ist – einige Ausführungen nach einem Wechsel anzusehen ist der einfachste Weg, die neue Verbrauchsrate abzuschätzen.

### Tokens pro Ausführung

Die vom Modell verwendeten Antwort-Token werden bei jedem Trigger als **tokensUsed** protokolliert. Sie sind in den Webhook-Payloads `trigger.succeeded` und `trigger.failed` enthalten (siehe [Webhook Payloads](#webhook-payloads)) und werden in der [Detailansicht der Ausführung](#run-detail-view) angezeigt. Die Menge hängt ab von:

- Wie viel [Kontext](#context-options) Sie einbeziehen – Thread-Kontext, Benutzerverlauf und Seitenmetadaten fügen alle Token hinzu.
- Wie lang Ihr [Initialer Prompt](#personality-prompt) und Ihre [Community-Richtlinien](#community-guidelines) sind.
- Wie viele Tools der Agent in einem einzelnen Lauf aufruft (jeder Tool-Aufruf und dessen Ergebnis wird durch das Modell hin- und hergesendet).

**Max Tokens Per Trigger** (default 20,000) ist die Obergrenze pro Lauf und wird pro Agent gesetzt.

### Modelle wechseln

Sie können Modelle im Bearbeitungsformular jederzeit wechseln. Bestehende Ausführungsverläufe und Analysen behalten ihre ursprünglichen Token- und Kostenwerte – sie werden zur Laufzeit aufgezeichnet. Das neue Modell gilt nur für Ausführungen, die nach dem Speichern beginnen.

There is no "use whichever model is cheaper" option. The choice is explicit per agent.
---
AI Agents sind in den Plänen **Flex** und **Pro** verfügbar. Der Creator-Plan enthält sie nicht.

### Beschränkungen auf Plan-Ebene

Jede Planstufe legt fest:

- **Standardmäßige tägliche und monatliche Budgetobergrenzen.** Sie können diese pro Agent senken; das Anheben des Kontenweiten Limits erfordert einen Plan mit einer höheren Obergrenze. Siehe [Budget-Übersicht](#budgets-overview).

Die genauen Zahlen werden auf der [Preisseite](https://fastcomments.com/traffic-pricing) und auf der Abrechnungsseite Ihres Kontos angezeigt. Sie werden auch direkt im Agenten-Bearbeitungsformular angezeigt, sodass Sie das Formular nie verlassen müssen, um Ihre Obergrenze zu finden.

FastComments Pro beinhaltet $200/Monat an KI-Nutzung. Bei Flex erfolgt die Abrechnung mit $20 pro Million Tokens für alle Modelle (derzeit entweder GLM 5.1 oder gpt-oss-120B-turbo).

### Gültige Abrechnung erforderlich

AI Agents laufen nur, wenn der Mandant **gültige Abrechnungsinformationen** hinterlegt hat. Wenn die Zahlungsmethode ungültig wird, werden alle Agents angehalten und die Seite für AI Agents zeigt ein Banner an, das die Person mit der **Billing Admin**-Rolle auffordert, die Abrechnung zu aktualisieren. Agents nehmen ihren Betrieb von selbst wieder auf, sobald die Abrechnung wiederhergestellt ist – es erfolgt keine Wiedergabe oder Nachbearbeitung von Triggern, die während des Ausfalls ausgelöst wurden.

Dies ist eine harte Voraussetzung: Token-Ausgaben werden Ihrem Konto in Rechnung gestellt, daher wird die Plattform keinen LLM-Aufruf starten, ohne dass eine funktionierende Zahlungsmethode vorhanden ist.

### Wer Agents verwalten kann

Die Admin-Seiten für Agents sind durch die Dashboard-Rolle **Customization Admin** geschützt. **Comment Moderator Admins** können Überprüfungen durchführen und Entscheidungen über Genehmigungen treffen (siehe [Genehmigungs-Workflow](#approval-workflow)), dürfen jedoch keine Agents erstellen oder bearbeiten. **Billing Admins** erhalten [Budgetwarnungen per E-Mail](#budget-alerts), unabhängig davon, ob sie Zugriff auf Agents haben.
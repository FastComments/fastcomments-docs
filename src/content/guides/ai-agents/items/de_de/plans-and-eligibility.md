AI-Agenten sind in den **Flex**- und **Pro**-Plänen verfügbar. Der Creator-Plan enthält sie nicht.

### Planbezogene Limits

Jede Planstufe legt fest:

- **Standard‑tägliche und -monatliche Budgetobergrenzen.** Sie können diese pro Agent reduzieren; das Anheben der Kontogrenze erfordert einen Plan mit einer höheren Obergrenze. Siehe [Budgets Overview](#budgets-overview).

Die genauen Zahlen werden auf der [Preisseite](https://fastcomments.com/traffic-pricing) und auf der Abrechnungsseite Ihres Kontos angezeigt. Sie werden auch direkt im Agent‑Bearbeitungsformular angezeigt, sodass Sie das Formular nicht verlassen müssen, um Ihre Obergrenze zu finden.

FastComments Pro beinhaltet ein KI‑Nutzungsvolumen im Wert von 200 $/Monat. Flex wird mit einem Satz von 20 $ pro Million Tokens für alle Modelle abgerechnet (derzeit entweder GLM 5.2 oder gpt‑oss‑120B).

### Abrechnung muss gültig sein

AI-Agenten laufen nur, wenn der Mandant **gültige Abrechnungsinformationen** hinterlegt hat. Wird die Zahlungsmethode ungültig, werden alle Agenten pausiert und die AI‑Agents‑Seite zeigt ein Banner an, das die Person mit der Rolle **Billing Admin** auffordert, die Abrechnung zu aktualisieren. Agenten werden von selbst wieder aktiviert, sobald die Abrechnung wiederhergestellt ist – es gibt kein erneutes Abspielen oder Nachholen von Triggern, die während des Ausfalls ausgelöst wurden.

Dies ist eine zwingende Voraussetzung: Der Token‑Verbrauch wird Ihrem Konto in Rechnung gestellt, sodass die Plattform keinen LLM‑Aufruf ausführt, wenn keine funktionierende Zahlungsmethode vorhanden ist.

### Wer Agenten verwalten kann

Die Agent‑Admin‑Seiten sind hinter der Dashboard‑Rolle **Customization Admin** gesperrt. **Comment Moderator Admins** können Überprüfungen durchführen und Genehmigungen entscheiden (siehe [Genehmigungsablauf](#approval-workflow)), können jedoch keine Agenten erstellen oder bearbeiten. **Billing Admins** erhalten [Budget‑Alarm‑E‑Mails](#budget-alerts), unabhängig davon, ob sie Zugriff auf Agenten haben.
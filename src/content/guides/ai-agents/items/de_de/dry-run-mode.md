**Dry Run** ist der Sicherheitsmodus, in dem jeder neue Agent startet. Der Agent läuft Ende‑zu‑Ende, mit Ausnahme des Teils, in dem er mit Ihrer Community interagiert.

### Was im Dry Run ausgeführt wird

- Trigger feuern normal.
- Das Prompt des Agenten, die [Community-Richtlinien](#community-guidelines) und der [Kontext](#context-options) werden zusammengefügt.
- Das LLM wird aufgerufen.
- Das Modell wählt Tool‑Aufrufe aus und liefert Begründungen sowie Vertrauenswerte.
- Der Lauf wird mit einem **Dry Run**‑Badge protokolliert, sodass er klar von Live‑Läufen unterschieden werden kann.

### Was im Dry Run nicht ausgeführt wird

- Es wird kein Kommentar gepostet, keine Stimme abgegeben, kein Kommentar angeheftet/abgeheftet/gesperrt/entsperrt.
- Kein Kommentar wird als Spam markiert, genehmigt oder überprüft.
- Kein Nutzer wird gesperrt, verwarnt oder mit einem Abzeichen ausgezeichnet.
- Es wird keine E‑Mail gesendet.
- Es wird kein memory geschrieben. (Ja – einschließlich memory. Dry‑Run‑Agenten können den shared memory pool nicht mit hypothetischen Entscheidungen vergiften.)
- Für Tool‑Aktionen werden keine Webhooks ausgelöst. (Die Trigger‑Ebene `trigger.succeeded` / `trigger.failed` Webhooks werden weiterhin ausgelöst und die Nutzlast enthält `wasDryRun: true`. Siehe [Webhook Payloads](#webhook-payloads).)

### Was es kostet

Dry Run führt **denselben LLM‑Aufruf** aus, den auch ein Enabled‑Lauf ausführen würde. Tokens werden berechnet, [budget caps](#budgets-overview) gelten, und die Läufe zählen gegen die täglichen/monatlichen Limits pro Agent und pro Tenant.

Diese Kosten sind der Preis für eine verlässliche Vorschau. Ein „skip the LLM call“-Modus würde Ihnen kein Signal darüber geben, wie sich der Agent verhalten würde.

### Dry‑Run‑Ergebnisse lesen

Im [Ausführungsverlauf](#run-history) sind Dry‑Run‑Läufe in der Statusspalte mit dem **Dry Run**‑Badge markiert. Die Aktionen innerhalb jedes Laufs sehen identisch zu Live‑Aktionen aus – derselbe Tool‑Name, dieselben Argumente, dieselbe Begründung und dasselbe Vertrauensmaß – außer dass keine von ihnen tatsächlich stattgefunden hat.

Die [Analytics‑Seite](#analytics-page) bricht „dry‑run vs live“ Läufe pro Monat auf, damit Sie sehen können, wie viel Ihrer Token‑Ausgaben für Beobachtungen verwendet wurden.

### Aus dem Dry Run wechseln

Bearbeiten Sie den Agenten und ändern Sie **Status** auf **Enabled**. Der nächste Trigger läuft live.

Sie können auch in die andere Richtung wechseln – von Enabled zurück zu Dry Run – falls der Agent anfängt, Dinge zu tun, die Ihnen nicht gefallen. Es gibt keine Strafe.

### Replays erzwingen Dry Run

Die Funktion [Test Runs (Replays)](#test-runs-replays) führt den Agenten gegen historische Kommentare **immer im Dry Run** aus, unabhängig vom gespeicherten Status des Agenten. Replays können keine realen Aktionen an vergangenen Kommentaren durchführen. Das ist beabsichtigt – Replay ist ein Vorschauwerkzeug, kein Moderationswerkzeug.
## Auslöser

Auslöser starten einen Zap, wenn etwas in FastComments passiert. Alle drei sind sofortig: FastComments liefert das
Ereignis an Zapier über einen Webhook im Moment des Auftretens. Es wird nichts Ihr Konto abgefragt und es werden
keine API-Guthaben für das Warten verbraucht.

| Auslöser | Wird ausgelöst, wenn |
|----------|----------------------|
| Neuer Kommentar | Ein Kommentar wird gepostet. Standardmäßig werden nur genehmigte, nicht‑Spam‑Kommentare ausgelöst. |
| Aktualisierter Kommentar | Ein Kommentar wird bearbeitet, genehmigt, abgestimmt, angeheftet, gesperrt oder anderweitig geändert. |
| Gelöschter Kommentar | Ein Kommentar wird gelöscht. |

Jeder Auslöser gibt den vollständigen Kommentar zurück: ID, Seiten‑URL und URL‑ID, Name und E‑Mail des Kommentators, der Kommentartext
als Markdown und als HTML, Abstimmungszahlen, Genehmigungs‑ und Spam‑Flags, die Locale, die Domain und etwaige Erwähnungen. Die
Felder entsprechen dem Webhook‑Payload, das unter Webhooks, Datenstrukturen dokumentiert ist.

## Optionen

**Domain.** Jeder Auslöser hat einen optionalen Domain‑Filter, der die auf Ihrem Konto konfigurierten Domains auflistet.
Lassen Sie das Feld leer, um Ereignisse von allen Domains zu erhalten.

**Ungeprüfte und Spam‑Kommentare einbeziehen.** Nur beim Auslöser „Neuer Kommentar“. Kommentare, die zur
Moderation zurückgehalten oder als Spam markiert werden, werden standardmäßig übersprungen. Wenn ein solcher Kommentar später genehmigt wird, löst der Auslöser
„Aktualisierter Kommentar“ aus, sodass ein Zap, der auf jeden Kommentar reagieren soll, der sichtbar wird, den
„Aktualisierter Kommentar“-Auslöser mit einem Filter auf das Feld „approved“ verwendet.

## Wie die Zustellung funktioniert

Das Aktivieren eines Zaps erstellt ein Webhook‑Abonnement in Ihrem Konto, das auf der Seite Webhooks mit der
Quelle **API** sichtbar ist. Das Deaktivieren des Zaps entfernt es. Die eigenen Limits von Zapier gelten dafür, wie viele Ereignisse pro
Minute akzeptiert werden; FastComments versucht eine fehlgeschlagene Zustellung erneut, mit zunehmender Verzögerung, und deaktiviert ein Abonnement, das
sechs Tage lang fehlschlägt. Ein deaktiviertes Abonnement kann auf der Seite Webhooks wieder aktiviert werden, oder Sie schalten den
Zap einfach aus und wieder ein, um ein neues zu erstellen.

Ein Konto kann bis zu 50 API‑Abonnements halten. Jeder Zap, der einen FastComments‑Auslöser verwendet, nutzt eines.
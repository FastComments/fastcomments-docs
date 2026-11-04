Jeder Agent-Webhook wird mit HMAC-SHA256 unter Verwendung des API-Secrets Ihres Tenants signiert. Dasselbe Signaturschema wird für die Kommentar-Webhooks von FastComments verwendet - wenn Sie diese bereits integriert haben, nutzen die Agent-Webhooks denselben Signatur-Header und denselben Verifizierungsablauf.

### Warum Signierung

Ohne eine Signatur könnte ein Angreifer, der Ihre Webhook-URL kennt, gefälschte Ereignisse POSTen, die so aussehen, als kämen sie von FastComments. Durch die Signierung kann Ihr Endpunkt überprüfen, ob jede Zustellung authentisch ist, bevor er darauf reagiert.

### Wie Signaturen funktionieren

Für jede Zustellung:

1. Die Plattform schlägt das API-Secret für den Tenant + die übereinstimmende Domain nach (siehe [Webhooks Overview](#webhooks-overview)).
2. Sie sendet den aktuellen Unix-Zeitstempel (in Millisekunden) im `X-FastComments-Timestamp`-Header.
3. Sie berechnet `HMAC-SHA256(api_secret, "${timestamp}.${raw_request_body}")` (Stripe-Stil) und sendet das Ergebnis als `sha256=<hex>` im `X-FastComments-Signature`-Header.
4. Ihr Endpunkt liest den Timestamp-Header, berechnet den HMAC über `${timestamp}.${body}` neu, wie er empfangen wurde, vergleicht ihn mit dem `sha256=<hex>`-Wert im Signatur-Header und lehnt Übereinstimmungsfehler ab.

Der signierte Body sind die **exakten Bytes**, die die Plattform gesendet hat, mit dem Präfix `${timestamp}.` - Ihr Verifizierer muss den rohen Request-Body verwenden, nicht eine neu serialisierte JSON-Zeichenkette (da sich sonst die Schlüsselreihenfolge und Leerzeichen unterscheiden würden).

### API-Secret

Dasselbe API-Secret, das von den [Kommentar-Webhooks](/guide-webhooks.html) verwendet wird. Es gilt pro (Tenant, Domain) und wird in den API-Einstellungen Ihres Tenants verwaltet. Wenn Sie das Secret rotieren, sollten Sie Ihren Verifizierer neu bereitstellen, um den neuen Wert vor der nächsten Zustellung zu lesen.

Wenn die Plattform **kein API-Secret** für die übereinstimmende Domain findet, erfolgt die Zustellung nicht. Das Webhook-Protokoll verzeichnet den Fehler mit dem Grund "no API secret".

### Verifizierungsbeispiel (Node.js)

[inline-code-attrs-start title = 'Beispiel zur Verifizierung der Webhook-Signatur'; type='javascript' inline-code-attrs-end]
[inline-code-start]
import crypto from 'crypto';

function verifyAgentWebhook(rawBody, signatureHeader, timestampHeader, secret) {
  const expected = 'sha256=' + crypto
    .createHmac('sha256', secret)
    .update(`${timestampHeader}.${rawBody}`)
    .digest('hex');
  return crypto.timingSafeEqual(
    Buffer.from(expected),
    Buffer.from(signatureHeader),
  );
}
[inline-code-end]

Verwenden Sie `timingSafeEqual` statt `===`, um Timing-Kanal-Lecks der Signatur zu vermeiden.

### Was ist im signierten Body enthalten

Der vollständige Umschlag plus der ereignisspezifische `data`-Block. Siehe [Webhook Payloads](#webhook-payloads).

### Empfehlungen

- **Verifizieren Sie bei jeder Zustellung.** Wenn Ihr Endpunkt unsignierte Anfragen akzeptiert, haben Sie keine Integritätsgarantie.
- **Lehnen Sie bei Signaturabweichung ab.** Geben Sie 401 oder 403 zurück; antworten Sie bei einer ungültigen Signatur nicht mit 200 OK, sonst verschleiern Sie Angriffe in Ihren Zustellungsprotokollen.
- **Verwenden Sie HTTPS.** Signaturen schützen die Integrität; TLS schützt die Vertraulichkeit (sowohl Ihr Secret als auch der Kommentartext in der Payload).
- **Rotieren Sie Secrets**, wenn Teammitglieder mit Zugriff ausscheiden, oder nach einem Zeitplan.

### Replay-Schutz

Signierung allein verhindert keine Replay-Angriffe - ein Angreifer, der eine echte signierte Zustellung erfasst hat, kann sie erneut senden. Der Replay-Schutz liegt in der Verantwortung Ihres Endpunkts:

- Verwenden Sie das `occurredAt`-Umschlagfeld und lehnen Sie Zustellungen ab, die älter sind als z. B. 5 Minuten.
- Verwenden Sie `triggerId` oder `approvalId` als Deduplizierungsschlüssel - wenn Sie ihn bereits verarbeitet haben, ignorieren Sie das Duplikat.

### Siehe auch

- [Webhooks Overview](#webhooks-overview).
- [Webhook Payloads](#webhook-payloads).
- Die Hauptanleitung zu [Webhooks](/guide-webhooks.html) für die umfassendere Signatur-Infrastruktur.
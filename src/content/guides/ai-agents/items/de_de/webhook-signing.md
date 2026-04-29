Jeder Agent-Webhook wird mit HMAC-SHA256 unter Verwendung des API Secret Ihres tenant signiert. Dasselbe Signaturverfahren wird für die Kommentar-Webhooks von FastComments verwendet – wenn Sie diese bereits integriert haben, verwenden die Agent-Webhooks denselben Signatur-Header und denselben Verifizierungsablauf.

### Warum Signaturen

Ohne Signatur könnte ein Angreifer, der Ihre Webhook-URL kennt, gefälschte Events per POST senden, die so aussehen, als kämen sie von FastComments. Durch Signieren kann Ihr Endpoint jede Zustellung verifizieren und prüfen, ob sie authentisch ist, bevor Sie darauf reagieren.

### Wie Signaturen funktionieren

Für jede Zustellung:

1. Die Plattform sucht das API Secret für den tenant + die zugehörige domain (siehe [Webhooks-Übersicht](#webhooks-overview)).
2. Sie setzt den aktuellen Unix-Timestamp (in Millisekunden) im Header `X-FastComments-Timestamp`.
3. Sie berechnet `HMAC-SHA256(api_secret, "${timestamp}.${raw_request_body}")` (im Stil von Stripe) und gibt das Ergebnis als `sha256=<hex>` im Header `X-FastComments-Signature` aus.
4. Ihr Endpoint liest den Timestamp-Header, berechnet das HMAC über `${timestamp}.${body}` erneut, vergleicht es mit dem `sha256=<hex>`-Wert im Signatur-Header und verwirft Nichtübereinstimmungen.

Der Body, der signiert wird, sind die **genauen Bytes**, die die Plattform gesendet hat, vorangestellt mit `${timestamp}.` - Ihr Verifizierer muss den rohen Request-Body verwenden, nicht einen neu serialisierten JSON-String (Schlüsselreihenfolge und Leerzeichen würden sonst abweichen).

### API secret

Das gleiche API Secret, das von [Kommentar-Webhooks](/guide-webhooks.html) verwendet wird. Es ist pro (tenant, domain) und wird in den API-Einstellungen Ihres tenant verwaltet. Wenn Sie das Secret rotieren, sollten Sie Ihren Verifizierer neu bereitstellen, damit er den neuen Wert vor der nächsten Zustellung liest.

Wenn die Plattform **kein API Secret** für die zugeordnete domain findet, erfolgt die Zustellung nicht. Das Webhook-Log protokolliert den Fehler mit dem Grund "no API secret".

### Verifizierungsbeispiel (Node.js)

[inline-code-attrs-start title = 'Webhook-Signatur-Verifizierungsbeispiel'; type='javascript' inline-code-attrs-end]
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

### Was im signierten Body enthalten ist

Der vollständige Envelope plus der ereignisspezifische `data`-Block. Siehe [Webhook-Nutzlasten](#webhook-payloads).

### Empfehlungen

- **Verifizieren Sie bei jeder Zustellung.** Wenn Ihr Endpoint unsignierte Anfragen akzeptiert, haben Sie keine Integritätsgarantie.
- **Ablehnen bei Signaturfehler.** Geben Sie 401 oder 403 zurück; antworten Sie nicht mit 200 OK bei einer fehlerhaften Signatur, sonst verschleiern Sie Angriffe in Ihren Zustellungsprotokollen.
- **Verwenden Sie HTTPS.** Signaturen schützen die Integrität; TLS schützt die Vertraulichkeit (sowohl Ihres Secrets als auch des Kommentartextes im Payload).
- **Rotieren Sie Secrets** wenn Teammitglieder mit Zugriff das Unternehmen verlassen, oder nach einem festen Zeitplan.

### Schutz vor Replay-Angriffen

Allein die Signatur verhindert keine Replay-Angriffe – ein Angreifer, der eine echte signierte Zustellung abgefangen hat, kann diese erneut senden. Der Schutz vor Replay-Angriffen obliegt Ihrem Endpoint:

- Verwenden Sie das Envelope-Feld `occurredAt` und lehnen Sie Zustellungen ab, die älter als, sagen wir, 5 Minuten sind.
- Verwenden Sie `triggerId` oder `approvalId` als Dedup-Schlüssel – wenn Sie es bereits verarbeitet haben, ignorieren Sie das Duplikat.

### Siehe auch

- [Webhooks-Übersicht](#webhooks-overview).
- [Webhook-Nutzlasten](#webhook-payloads).
- Der Haupt[Webhooks-Leitfaden](/guide-webhooks.html) für die umfassendere Signaturinfrastruktur.

---
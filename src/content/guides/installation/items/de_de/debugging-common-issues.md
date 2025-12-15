Hier sind einige häufig auftretende Symptome und gängige Lösungen.

### "This is a demo"-Meldung

Dies wird angezeigt, wenn Sie den Widget-Code von unserer Startseite kopiert haben, die unseren Demo-Mandanten verwendet. Um Ihren Mandanten zu verwenden, kopieren Sie den Widget-Code von [hier](https://fastcomments.com/auth/my-account/get-acct-code).

### "FastComments cannot load on this domain"-Fehler

FastComments muss wissen, welche Domains Ihnen gehören, um Anfragen zu authentifizieren, die mit Ihrem Konto verbunden sind. [Lesen Sie unsere Dokumentation](/guide-multiple-sites.html#add-domains-to-account), um zu erfahren, wie Sie diesen Fehler beheben können (fügen Sie einfach die genaue Subdomain + Domain zu Ihrem Konto hinzu).

Beachten Sie, dass dies nur nach Ablauf der Testphase auftreten sollte. Während der Testphase werden alle Anfragen von neuen Domains automatisch zu Ihrem Konto hinzugefügt.

### Migrierte Kommentare werden bei benutzerdefinierten Installationen nicht angezeigt

Dies geschieht normalerweise, wenn die importierten Kommentare an eine `Page ID` gebunden sind und Sie eine URL übergeben (oder keinen Wert, in diesem Fall wird standardmäßig die Seiten-URL verwendet).

Sie können dies debuggen, indem Sie [Ihre Kommentare exportieren](https://fastcomments.com/auth/my-account/manage-data/export) und die Spalte `URL ID` (derzeit Spalte `B`) anzeigen.

Stellen Sie sicher, dass die Werte in der Spalte `URL ID` dieselben Werte sind, die Sie als `urlId`-Parameter an die Widget-Konfiguration übergeben.

Für weitere Erklärungen lesen Sie unsere [Dokumentation darüber, wie Kommentare mit Seiten und Artikeln verknüpft werden](/guide-customizations-and-configuration.html#url-id).

Wenn alles andere fehlschlägt, [kontaktieren Sie uns](https://fastcomments.com/auth/my-account/help).

### Kommentar-Widget wird nicht angezeigt

Wenn das Kommentar-Widget nicht angezeigt wird, überprüfen Sie die Chrome-Entwicklerkonsole auf Fehler.

Bei den meisten Fehlkonfigurationen zeigt das Kommentar-Widget zumindest einen Fehler auf der Seite an, wenn es laden kann. Nichts zu sehen ist normalerweise ein Hinweis auf einen Scripting-Fehler.

### Gewünschte Konfiguration funktioniert nicht wie erwartet

Probieren Sie unsere [Chrome-Erweiterung](https://chrome.google.com/webstore/detail/fastcomments-debugger/cadggdemhfkjjghkdbfhonoccnplffjj?hl=en-US) aus, um zu sehen, welche Konfiguration an das Kommentar-Widget übergeben wird. Wenn alles andere fehlschlägt, machen Sie einen Screenshot von dem, was die Chrome-Erweiterung anzeigt, und [kontaktieren Sie uns](https://fastcomments.com/auth/my-account/help).

### Kommentare fehlen bei derselben URL mit unterschiedlichem Hash-Bang

Standardmäßig verwendet FastComments die Seiten-URL für den "Bucket", in dem Kommentare gespeichert werden. Wenn Ihre URLs `#hashbangs` enthalten und diese `#hashbangs` nicht Teil der Kennung sein sollten, die einen Kommentar-Thread identifiziert, können wir den Hash-Bang-Wert einfach ignorieren, zum Beispiel:

[inline-code-attrs-start title = 'Ignore Hash Bangs Example'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
window.FastCommentsUI(document.getElementById('fastcomments-widget'), {
    tenantId: "demo",
    url: location.href.replace(location.hash, ''),
    urlId: location.href.replace(location.hash, '')
});
</script>
[inline-code-end]

Beachten Sie, dass nach dieser Änderung eine Migration für bestehende Kommentare durchgeführt werden muss. [Kontaktieren Sie uns dafür.](https://fastcomments.com/auth/my-account/help)

### URL-Abfrageparameter beeinflussen das Widget

Standardmäßig verwendet FastComments die Seiten-URL für den "Bucket", in dem Kommentare gespeichert werden. Wenn Ihre URLs Abfrageparameter enthalten, die nicht Teil der Kennung sein sollten, die einen Kommentar-Thread identifiziert, können wir sie einfach ignorieren, zum Beispiel:

[inline-code-attrs-start title = 'Ignore Query Parameters'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
window.FastCommentsUI(document.getElementById('fastcomments-widget'), {
    tenantId: "demo",
    url: location.protocol + '//' + location.host + location.pathname,
    urlId: location.pathname
});
</script>
[inline-code-end]

Beachten Sie, dass nach dieser Änderung eine Migration für bestehende Kommentare durchgeführt werden muss. [Kontaktieren Sie uns dafür.](https://fastcomments.com/auth/my-account/help)

### E-Mails werden nicht empfangen

Bei FastComments investieren wir viel Arbeit, um sicherzustellen, dass unsere E-Mail-Zustellung so zuverlässig wie möglich ist. Einige E-Mail-Anbieter sind jedoch notorisch schwer zuverlässig zu erreichen. Überprüfen Sie Ihren Spam-Ordner auf Nachrichten von fastcomments.com.

Wenn Sie [uns kontaktieren](https://fastcomments.com/auth/my-account/help), können wir in der Regel mehr Einblick geben, warum Sie möglicherweise keine E-Mails von uns erhalten.

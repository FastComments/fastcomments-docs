Hier sind einige Symptome, die häufig auftreten, und gängige Lösungen. 

### "This is a demo" Meldung

Dies wird angezeigt, wenn Sie den Widget-Code von unserer Startseite kopiert haben, die unseren Demo-Tenant verwendet. Um Ihren Tenant zu verwenden, kopieren Sie den Widget-Code von [hier](https://fastcomments.com/auth/my-account/get-acct-code).

### "FastComments cannot load on this domain" Fehler

FastComments muss wissen, welche Domains Ihnen gehören, um Anfragen, die mit Ihrem Konto verbunden sind, zu authentifizieren. [Sehen Sie sich unsere Dokumentation an](/guide-multiple-sites.html#add-domains-to-account), um zu erfahren, wie Sie diesen Fehler beheben können (fügen Sie einfach die exakte Subdomain + Domain zu Ihrem Konto hinzu).

Beachten Sie, dass dies normalerweise erst nach Ablauf der Testphase auftreten sollte. Während der Testphase werden Anfragen von neuen Domains automatisch zu Ihrem Konto hinzugefügt.

### Migrierte Kommentare werden bei benutzerdefinierten Installationen nicht angezeigt

In der Regel tritt dies auf, wenn die importierten Kommentare an eine `Page ID` gebunden sind und Sie eine URL übergeben (oder keinen Wert, in welchem Fall standardmäßig die Seiten-URL verwendet wird).

Sie können dies debuggen, indem Sie [Ihre Kommentare exportieren](https://fastcomments.com/auth/my-account/manage-data/export) und die Spalte `URL ID` ansehen (derzeit Spalte `B`).

Stellen Sie sicher, dass die Werte, die Sie in der Spalte `URL ID` sehen, dieselben Werte sind, die Sie der Widget-Konfiguration als `urlId`-Parameter übergeben.

Für weitere Erläuterungen lesen Sie unsere [Dokumentation dazu, wie Kommentare an Seiten und Artikel gebunden sind](/guide-customizations-and-configuration.html#url-id).

Falls alles andere fehlschlägt, [kontaktieren Sie uns](https://fastcomments.com/auth/my-account/help).

### Kommentar-Widget wird nicht angezeigt

Wenn das Kommentar-Widget nicht angezeigt wird, überprüfen Sie die Entwicklerkonsole von Chrome auf Fehler.

Bei den meisten Fehlkonfigurationen zeigt das Kommentar-Widget zumindest einen Fehler auf der Seite an, wenn es geladen werden kann. Wenn gar nichts angezeigt wird, ist das normalerweise ein Hinweis auf einen Skriptfehler.

### Gewünschte Konfiguration funktioniert nicht wie erwartet

Versuchen Sie unsere [Chrome-Erweiterung](https://chrome.google.com/webstore/detail/fastcomments-debugger/cadggdemhfkjjghkdbfhonoccnplffjj?hl=en-US), um zu sehen, welche Konfiguration an das Kommentar-Widget übergeben wird. Wenn alles fehlschlägt, machen Sie einen Screenshot von dem, was die Chrome-Erweiterung anzeigt, und [kontaktieren Sie uns](https://fastcomments.com/auth/my-account/help).

### Kommentare fehlen bei gleicher URL mit unterschiedlichem Hashbang

Standardmäßig verwendet FastComments die Seiten-URL für den "Bucket", in dem Kommentare gespeichert werden. Wenn Ihre URLs `#hashbangs` enthalten und diese `#hashbangs` keinen Teil des Identifikators darstellen sollen, der einen Kommentar-Thread identifiziert, können wir den Hashbang-Wert einfach ignorieren, zum Beispiel:

[inline-code-attrs-start title = 'Beispiel: Hashbangs ignorieren'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script async src="https://cdn.fastcomments.com/js/embed-v2-async.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
window.fcConfigs = [{
    target: '#fastcomments-widget',
    tenantId: "demo",
    url: location.href.replace(location.hash, ''),
    urlId: location.href.replace(location.hash, '')
}];
</script>
[inline-code-end]

Beachten Sie, dass nach dieser Änderung eine Migration für vorhandene Kommentare durchgeführt werden muss. [Wenden Sie sich dafür an uns.](https://fastcomments.com/auth/my-account/help)

### URL-Abfrageparameter, die das Widget beeinflussen

Standardmäßig verwendet FastComments die Seiten-URL für den "Bucket", in dem Kommentare gespeichert werden. Wenn Ihre URLs Abfrageparameter enthalten, die keinen Teil des Identifikators darstellen sollen, der einen Kommentar-Thread identifiziert, können wir diese einfach ignorieren, zum Beispiel:

[inline-code-attrs-start title = 'Beispiel: Abfrageparameter ignorieren'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script async src="https://cdn.fastcomments.com/js/embed-v2-async.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
window.fcConfigs = [{
    target: '#fastcomments-widget',
    tenantId: "demo",
    url: location.protocol + '//' + location.host + location.pathname,
    urlId: location.pathname
}];
</script>
[inline-code-end]

Beachten Sie, dass nach dieser Änderung eine Migration für vorhandene Kommentare durchgeführt werden muss. [Wenden Sie sich dafür an uns.](https://fastcomments.com/auth/my-account/help)

### Keine E-Mails erhalten

Bei FastComments investieren wir viel Arbeit, um die Zustellung unserer E-Mails so zuverlässig wie möglich zu gestalten. Einige E-Mail-Anbieter sind jedoch notorisch schwer zuverlässig zu erreichen. Überprüfen Sie Ihren Spam-Ordner auf Nachrichten von fastcomments.com.

Wenn Sie [uns kontaktieren](https://fastcomments.com/auth/my-account/help), können wir in der Regel weitere Einblicke geben, warum Sie möglicherweise keine E-Mails von uns erhalten.
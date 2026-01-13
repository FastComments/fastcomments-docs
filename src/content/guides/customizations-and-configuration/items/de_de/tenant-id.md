[related-parameter-start name = 'tenantId'; type = 'string'; related-parameter-end]

Sie werden vielleicht feststellen, dass das Kommentar-Widget beispielsweise mit einer Tenant ID von "demo" verwendet werden kann:

[code-example-start config = {tenantId: 'demo'}; linesToHighlight = [5]; title = 'Demo Tenant ID'; code-example-end]

Dies dient nur dazu, das Kommentar-Widget auszuprobieren und damit zu spielen. In der Produktion würden Sie Ihre Tenant ID übergeben, wie folgt:

[code-example-start config = {tenantId: '{{{ExampleTenantId}}}'}; linesToHighlight = [5]; title = 'Defining Your Tenant ID'; code-example-end]

Ihre Tenant ID ist bereits im Kommentar-Widget <a href="https://fastcomments.com/auth/my-account/get-acct-code" target="_blank">Code-Snippet in Ihrem Konto</a> eingetragen.

Sie können Ihre Tenant ID auch finden und Ihre API-Schlüssel [auf der API-Anmeldeinformationen-Seite](https://fastcomments.com/auth/my-account/api-secret) verwalten.

Ab diesem Punkt verwenden die Codebeispiele Ihre reale Tenant ID, sofern Sie bei FastComments angemeldet sind (wenn Sie auf https://fastcomments.com eingeloggt sind).
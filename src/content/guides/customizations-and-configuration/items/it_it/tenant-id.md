[related-parameter-start name = 'tenantId'; type = 'string'; related-parameter-end]

Potresti notare che il widget dei commenti può essere utilizzato con un Tenant ID di "demo", ad esempio:

[code-example-start config = {tenantId: 'demo'}; linesToHighlight = [5]; title = 'Demo Tenant ID'; code-example-end]

Questo serve solo per provare e giocare con il widget dei commenti. In produzione, dovresti fornire il tuo Tenant ID, in questo modo:

[code-example-start config = {tenantId: '{{{ExampleTenantId}}}'}; linesToHighlight = [5]; title = 'Defining Your Tenant ID'; code-example-end]

Il tuo Tenant ID può essere trovato già applicato nello snippet di codice del widget dei commenti <a href="https://fastcomments.com/auth/my-account/get-acct-code" target="_blank">nel tuo account</a>.

Puoi anche trovare il tuo Tenant ID e gestire le tue chiavi API [nella pagina delle credenziali API](https://fastcomments.com/auth/my-account/api-secret).

Da questo momento in poi, se sei connesso a FastComments, gli esempi di codice utilizzeranno il tuo Tenant ID reale (se hai effettuato l'accesso su https://fastcomments.com).

---
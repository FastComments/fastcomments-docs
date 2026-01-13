[related-parameter-start name = 'tenantId'; type = 'string'; related-parameter-end]

Možda ćete primijetiti da se komentarski widget može koristiti s Tenant ID-jem "demo", na primjer:

[code-example-start config = {tenantId: 'demo'}; linesToHighlight = [5]; title = 'Demo Tenant ID'; code-example-end]

Ovo je namijenjeno samo za isprobavanje i igranje s komentarskim widgetom. U produkciji biste proslijedili svoj Tenant ID, ovako:

[code-example-start config = {tenantId: '{{{ExampleTenantId}}}'}; linesToHighlight = [5]; title = 'Defining Your Tenant ID'; code-example-end]

Vaš Tenant ID možete pronaći već primijenjen na isječku koda komentarskog widgeta <a href="https://fastcomments.com/auth/my-account/get-acct-code" target="_blank">u vašem računu</a>.

Također možete pronaći svoj Tenant ID i upravljati svojim API ključevima [na stranici s API vjerodajnicama](https://fastcomments.com/auth/my-account/api-secret).

Od ovog trenutka, ako ste prijavljeni u FastComments, primjeri koda će koristiti vaš stvarni Tenant ID (ako ste prijavljeni na https://fastcomments.com).

---
[related-parameter-start name = 'tenantId'; type = 'string'; related-parameter-end]

Možda ćete primetiti da se widget za komentare može koristiti sa Tenant ID-jem "demo", na primer:

[code-example-start config = {tenantId: 'demo'}; linesToHighlight = [5]; title = 'Demo Tenant ID'; code-example-end]

Ovo je namenjeno samo za isprobavanje i igranje sa widgetom za komentare. U produkciji biste prosledili svoj Tenant ID, na sledeći način:

[code-example-start config = {tenantId: '{{{ExampleTenantId}}}'}; linesToHighlight = [5]; title = 'Defining Your Tenant ID'; code-example-end]

Vaš Tenant ID već možete pronaći primenjen na widgetu za komentare <a href="https://fastcomments.com/auth/my-account/get-acct-code" target="_blank">isečak koda na vašem nalogu</a>.

Takođe možete pronaći svoj Tenant ID i upravljati svojim API ključevima [na stranici sa API akreditivima](https://fastcomments.com/auth/my-account/api-secret).

Od ovog trenutka, ako ste prijavljeni na FastComments, primeri koda će koristiti vaš stvarni Tenant ID (ako ste prijavljeni na https://fastcomments.com).
[related-parameter-start name = 'tenantId'; type = 'string'; related-parameter-end]

Možda ćete primijetiti da se widget za komentare može koristiti sa Tenant ID-om "demo", na primjer:

[code-example-start config = {tenantId: 'demo'}; linesToHighlight = [5]; title = 'Demo Tenant ID'; code-example-end]

Ovo je namijenjeno samo za isprobavanje i igranje sa widgetom za komentare. U produkciji biste proslijedili vaš Tenant ID, na sljedeći način:

[code-example-start config = {tenantId: '{{{ExampleTenantId}}}'}; linesToHighlight = [5]; title = 'Defining Your Tenant ID'; code-example-end]

Vaš Tenant ID već se može pronaći primijenjen na widgetu za komentare <a href="https://fastcomments.com/auth/my-account/get-acct-code" target="_blank">isječak koda u vašem nalogu</a>.

Takođe možete pronaći svoj Tenant ID i upravljati svojim API ključevima [na stranici za API vjerodajnice](https://fastcomments.com/auth/my-account/api-secret).

Od ovog trenutka, ako ste prijavljeni na FastComments, primjeri koda će koristiti vaš stvarni Tenant ID (ako ste prijavljeni na https://fastcomments.com).

---
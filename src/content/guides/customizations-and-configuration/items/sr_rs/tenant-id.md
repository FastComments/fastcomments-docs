[related-parameter-start name = 'tenantId'; type = 'string'; related-parameter-end]

Можете приметити да се видгет за коментаре може користити са Tenant ID-ом "demo", на пример:

[code-example-start config = {tenantId: 'demo'}; linesToHighlight = [5]; title = 'Демо Tenant ID'; code-example-end]

Ово је намењено само за испробавање видгета за коментаре. У продукцији бисте проследили ваш Tenant ID, на следећи начин:

[code-example-start config = {tenantId: '{{{ExampleTenantId}}}'}; linesToHighlight = [5]; title = 'Дефинисање вашег Tenant ID-а'; code-example-end]

Ваш Tenant ID се већ може пронаћи примењен у видгету за коментаре у <a href="https://fastcomments.com/auth/my-account/get-acct-code" target="_blank">кодном исечку у вашем налогу</a>.

Такође можете пронаћи ваш Tenant ID и управљати вашим API кључевима [на страници API акредитива](https://fastcomments.com/auth/my-account/api-secret).

Од ове тачке, ако сте пријављени у FastComments, примери кода ће користити ваш стварни Tenant ID (ако сте пријављени на https://fastcomments.com).

---
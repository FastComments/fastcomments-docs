[related-parameter-start name = 'tenantId'; type = 'string'; related-parameter-end]

Możesz zauważyć, że widżet komentarzy można użyć z Tenant ID ustawionym na "demo", na przykład:

[code-example-start config = {tenantId: 'demo'}; linesToHighlight = [5]; title = 'Przykładowy Tenant ID'; code-example-end]

To służy tylko do wypróbowania i zabawy z widżetem komentarzy. W środowisku produkcyjnym należy przekazać swój Tenant ID, w ten sposób:

[code-example-start config = {tenantId: '{{{ExampleTenantId}}}'}; linesToHighlight = [5]; title = 'Określanie Twojego Tenant ID'; code-example-end]

Twój Tenant ID jest już zastosowany w widżecie komentarzy <a href="https://fastcomments.com/auth/my-account/get-acct-code" target="_blank">fragmencie kodu w Twoim koncie</a>.

Swój Tenant ID możesz też znaleźć i zarządzać kluczami API [na stronie poświadczeń API](https://fastcomments.com/auth/my-account/api-secret).

Od tego momentu, jeśli jesteś zalogowany w FastComments, przykłady kodu będą używać Twojego rzeczywistego Tenant ID (jeśli jesteś zalogowany na https://fastcomments.com).
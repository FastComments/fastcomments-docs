A tabela `Past top-ups` na parte inferior da página de Credits lista suas recargas mais recentes, da mais nova para a mais antiga, com a `Date`, o `Amount` e o `Status` de cada uma. Antes da sua primeira recarga, ela exibe `No top-ups yet.`

Cada recarga tem um dos três status:

- `Credited`: o pagamento foi concluído com sucesso e o valor foi adicionado ao seu saldo.
- `Pending`: o pagamento foi iniciado, mas ainda não foi concluído, geralmente porque a confirmação do banco não foi finalizada. As recargas pendentes são verificadas junto ao Stripe na próxima vez que a página de Credits for aberta e passam para `Credited` ou `Failed`.
- `Failed`: o cartão foi recusado ou o banco não aprovou o pagamento. Nada foi cobrado e seu saldo não foi alterado.

A tabela registra as recargas feitas nesta página. As faturas pagas com crédito são exibidas junto com as próprias faturas, em `View Invoices` na página [Billing Info](https://fastcomments.com/auth/my-account/billing-info).
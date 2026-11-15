Cada cobrança, seja uma renovação de plano, um mês Flex ou uma mudança de plano, gera uma fatura. As faturas também servem como recibos após o pagamento.

### Encontrando Faturas

Abra [Histórico de Cobrança](https://fastcomments.com/auth/my-account/billing-history) ou clique em `View Invoices` na página de Informações de Cobrança. A tabela lista cada fatura com sua data, período de cobrança, número da fatura, descrição, total, status (`Paid`, `Unpaid`, `Draft` ou `Void`) e data de vencimento, dez por página, das mais recentes para as mais antigas.

Para cada fatura:

- `View` abre a fatura na página hospedada do Stripe, onde você também pode baixá-la.
- `PDF` baixa o PDF diretamente.
- `Pay Now` aparece em vez de `View` em uma fatura não paga e leva você à página de pagamento do Stripe para essa fatura, onde pode pagá-la diretamente.

As faturas também são enviadas por e-mail para seu e-mail de cobrança quando são criadas. Para que elas sejam enviadas para uma caixa de e-mail financeira em vez do proprietário da conta, defina o `Billing Email` em [Configurações de Fatura](#invoice-settings).

### O Que Há na Fatura

A fatura mostra o nome e endereço de cobrança e o VAT / Tax ID de [Configurações de Fatura](#invoice-settings), os itens da fatura, qualquer [crédito](#credits) pré-pago que foi aplicado e o valor cobrado no seu cartão. As faturas Flex listam uma linha por tipo de uso, veja [Como o Uso Flex É Medido](#how-flex-usage-is-measured). Para endereços de cobrança na União Europeia, as faturas Flex são emitidas com 0% de VAT e o rodapé `Reverse Charge Transaction: Customer to pay VAT`.

As alterações nas configurações de fatura se aplicam às faturas criadas após você salvá-las. Uma fatura que já foi emitida não pode ser editada, portanto defina os dados da sua empresa e o VAT / Tax ID antes da próxima data de cobrança.

### Contas Cobradas em Outro Lugar

Se sua cobrança for tratada externamente, por exemplo, por uma fatura combinada com o suporte, as faturas são enviadas a você para pagamento em vez de cobradas em um cartão e podem não aparecer no Histórico de Cobrança. Contas instaladas através do Shopify ou BigCommerce são cobradas por essa plataforma, e a página de Informações de Cobrança contém um link para a página do plano lá. Subcontas cobradas através de uma conta pai não têm faturas próprias; veja [Quando Você É Cobrado](#when-you-are-charged).
O que é cobrado no seu cartão, e quando, depende do tipo de plano que você possui.

### Planos Fixos

Quando você inicia uma assinatura `Creators` ou `Pro`, o primeiro mês ou ano é cobrado imediatamente, no navegador, quando você clica em `Pay Now & Start Subscription`. Se o seu banco pedir para confirmar o pagamento, conclua a verificação na janela que se abre. A assinatura só começa depois que o pagamento for processado.

Depois disso, a assinatura é renovada no mesmo dia de cada mês, ou de cada ano se você escolheu `Pay Annually`, contado a partir do dia em que você assinou. A renovação é cobrada automaticamente no cartão cadastrado e a fatura é enviada por e-mail para o seu e-mail de cobrança. Não há nenhuma ação a ser tomada.

O faturamento anual é 15% mais barato que o mensal. O Flex não pode ser faturado anualmente.

### Flex

O Flex é faturado postumamente: você usa o serviço durante um mês civil, e a fatura desse mês é criada no último dia do mês (UTC) e cobrada no seu cartão imediatamente. A data de faturamento é sempre o último dia do mês e não pode ser alterada.

A fatura cobre apenas aquele mês civil. O uso no mês civil em que você se inscreveu nunca é faturado, então sua primeira fatura é, no mínimo, no final do mês seguinte ao da sua inscrição.

Se o seu uso ficar abaixo do mínimo do Flex, uma linha com o texto `FastComments Flex. (minimum ... /mo)` completa a fatura até o valor mínimo. O mínimo atual é exibido na sua página [Billing Info](https://fastcomments.com/auth/my-account/billing-info) e na [página de preços](https://fastcomments.com/traffic-pricing).

Você pode ver qual será a próxima fatura a qualquer momento. O `Next Invoice Amount` está na página [Billing Info](https://fastcomments.com/auth/my-account/billing-info), e o detalhamento completo linha por linha está na página [Billing Analytics](https://fastcomments.com/auth/my-account/analytics/billing). O que é contabilizado está coberto em [Como o Uso do Flex é Medido](#how-flex-usage-is-measured).

### Salvar um Cartão Não Gera Cobrança

Depois que seu faturamento estiver configurado, salvar ou atualizar um cartão sem mudar de plano nunca gera cobrança, a menos que você tenha uma fatura não paga. O formulário diz `You won't be charged now unless you change plans.` e o cartão é simplesmente armazenado no Stripe para a próxima fatura. Somente quando você ainda não tem faturamento válido o formulário diz `Your card will be charged now.`, e mesmo assim uma conta Flex não é cobrada até o final do mês. Qualquer [crédito](#credits) pré-pago na sua conta é aplicado a cada fatura antes que o cartão seja cobrado.

### Sub-Contas Gerenciadas

Se a sua conta gerencia sub-contas (por exemplo, uma por site ou ambiente, ou tenants com marca própria), apenas a conta pai é faturada. O uso de cada sub-conta aparece como suas próprias linhas na fatura Flex da conta pai, prefixadas com o nome da sub-conta, e as sub-contas compartilham o método de pagamento e o status de faturamento da conta pai.
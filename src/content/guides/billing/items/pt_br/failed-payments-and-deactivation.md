Um pagamento pode falhar porque o cartão expirou, foi substituído, não tem fundos ou o banco bloqueou a cobrança. O FastComments dá a você tempo para corrigir isso antes que algo pare de funcionar, e avisa as pessoas que podem resolver.

### O Que Acontece

1. **A cobrança falha.** A fatura permanece `Unpaid` no [Histórico de Cobrança](https://fastcomments.com/auth/my-account/billing-history) com um botão `Pay Now`. Em um plano fixo, o Stripe pode tentar a cobrança novamente. No Flex, o FastComments tenta novamente qualquer fatura não paga com mais tempo que o período de tolerância todas as noites.
2. **Um aviso é enviado.** Todas as noites, o FastComments verifica cada conta. Se uma fatura Flex ainda estiver não paga após o período de tolerância e a nova tentativa falhar, ou se uma assinatura de plano fixo foi encerrada porque não pôde ser cobrada, todo usuário com a permissão `Billing Admin` recebe o e-mail `Your FastComments Tenant Will Be Deactivated Soon!` com a data, e a conta é agendada para desativação um período de tolerância depois. O widget continua funcionando nesse meio-tempo.
3. **Desativação.** Se nada mudar até a data agendada, a conta é desativada: os comentários ficam ocultos no seu site e os visitantes veem `This FastComments account's billing information is more than sixty days out of date. Comments hidden until the issue is resolved.` Os administradores de cobrança recebem um e-mail novamente. Seus dados são mantidos.

O período de tolerância é de uma semana para a maioria das contas e de duas semanas para contas com mais de um ano. No Flex, ele se aplica duas vezes: uma antes do aviso e outra entre o aviso e a desativação, de modo que há pelo menos duas semanas entre uma cobrança falha e os comentários ficarem ocultos.

### Como Corrigir

Salve um cartão funcional na página [Informações de Cobrança](https://fastcomments.com/auth/my-account/billing-info). Isso por si só reativa a conta imediatamente e cancela qualquer desativação agendada; a fatura não paga é então cobrada com o novo cartão, imediatamente em um plano fixo e durante a verificação noturna no Flex. Você também pode pagar a fatura imediatamente com `Pay Now` no Histórico de Cobrança.

Se o cartão registrado estiver correto e a falha foi temporária, você não precisa fazer nada: a verificação noturna percebe a fatura paga ou a assinatura ativa, cancela a desativação e envia um e-mail aos administradores de cobrança para confirmar. Uma conta já desativada é reativada da mesma forma.

### Várias Faturas ao Mesmo Tempo

Se o seu cartão estava inválido por alguns meses e depois foi corrigido, cada mês não pago é cobrado separadamente, então você pode ver várias cobranças no mesmo dia. Verifique o período de cobrança de cada fatura no Histórico de Cobrança: elas são de meses diferentes, não duplicatas.

### Ainda Vendo um Aviso de Cobrança Depois de Pagar?

O aviso pertence a uma conta específica. Se você pagou, mas um site ainda mostra `billing information is out of date`, o widget nesse site provavelmente está usando uma conta diferente daquela em que você pagou. Isso acontece quando um site foi configurado duas vezes, ou quando há uma conta em `fastcomments.com` e outra na implantação da UE em `eu.fastcomments.com`, que são cobradas separadamente. Compare o `tenantId` no código da sua página com o que está em `Settings -> API/SSO`, ou [entre em contato com o suporte](https://fastcomments.com/auth/my-account/help) e nós resolveremos isso.
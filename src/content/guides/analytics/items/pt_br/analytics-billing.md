Quaisquer métricas pelas quais você possa ser limitado por taxa ou cobrado são reportadas na [Página de Análises de Faturamento](https://fastcomments.com/auth/my-account/analytics/billing).

Alguns exemplos de métricas que são rastreadas:

- Carregamentos de Página (para o mês atual)
- Créditos de API (para o mês atual)
- Novos Comentários Armazenados (para o mês atual)
- Usuários do Tenant
- Usuários SSO (usuários SSO regulares sem permissões de admin/moderador)
- Admins SSO (usuários SSO com as flags `isAccountOwner` ou `isAdminAdmin` - cobrados separadamente dos usuários SSO regulares)
- Moderadores SSO (usuários SSO com a flag `isCommentModeratorAdmin` - cobrados separadamente dos usuários SSO regulares)
- Moderadores
- Domínios

Para os planos `Creators` e `Pro`, essas métricas servem simplesmente como um guia para saber se você está próximo de ou atingiu os limites da sua conta.

Para o plano `Flex`, esta página mostrará um detalhamento de custos do uso atual para a **próxima** fatura. Isso facilita entender como você será cobrado antes que isso aconteça. Você pode ver faturas anteriores em [Histórico de Faturamento](https://fastcomments.com/auth/my-account/billing-history). Como cada linha é contada e quando a fatura é cobrada é explicado no [Guia de Faturamento](/guide-billing.html#how-flex-usage-is-measured).

As métricas `Page Loads`, `API Credits` e `New Comments Stored` podem estar desatualizadas em até um minuto. As outras métricas são em tempo real.

Essas informações são armazenadas mensalmente por um ano, ou até você excluir seu tenant, para fins históricos, mas a interface exibirá apenas o período atual.
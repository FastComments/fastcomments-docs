Uma fatura Flex é uma lista de itens de linha, um por tipo de uso, cada um mostrando a quantidade e a tarifa, por exemplo `213,000 Page Loads. ($1.00 per 100,000 Page Loads)`. As tarifas estão na [página de preços](https://fastcomments.com/traffic-pricing) e na sua página [Billing Info](https://fastcomments.com/auth/my-account/billing-info). Esta seção explica o que cada quantidade significa.

### Contados Ao Longo Do Mês

Estes são zerados todo mês e contados do primeiro ao último dia do mês do calendário (UTC):

- **Page Loads**: o número de vezes que um widget de comentários ou feed foi carregado nas suas páginas. Cada carregamento do widget de comentários conta, não importa quantos comentários ele exiba. Não há cobrança por thread de comentários ou por página.
- **API Credits**: chamadas à REST API com a sua chave de API.
- **Small Widgets Credits**: carregamentos dos widgets pequenos, como contagens de comentários e badges.
- **LLM Tokens**: tokens usados por [AI Agents](/guide-ai-agents.html) e outros recursos de LLM.

### Contados No Fim Do Mês

Estes são um snapshot capturado quando a fatura é criada:

- **Comments Stored**: o número total de comentários na sua conta, não apenas os adicionados neste mês. Excluir comentários o reduz, exceto que um comentário excluído com respostas é mantido como placeholder para que as respostas continuem fazendo sentido.
- **SSO Users**: o número total de usuários SSO que o seu site criou, incluindo pessoas que comentaram uma vez há muito tempo. Se esta for a maior parte da sua fatura, pergunte ao suporte sobre cobrança por usuários ativos mensais, onde apenas usuários que fizeram login durante o mês contam.
- **Moderators**, **Admins**, **SSO Moderators** e **SSO Admins**: assentos, cobertos em [Admin and Moderator Seats](#admin-and-moderator-seats). O primeiro admin é gratuito.
- **Domains**: os sites na sua conta. O primeiro site é gratuito e cada adicional é cobrado por mês, enquanto estiver na conta. Um subdomínio só conta se você o adicionar como um site próprio; um único domínio de nível superior cobre os seus subdomínios. Veja [Multiple Sites](/guide-multiple-sites.html#domain-limits).
- **Managed Tenants**: subcontas gerenciadas por esta conta, se houver, mais o uso delas como linhas separadas.

### Mínimo

Se as linhas somarem menos que o mínimo do Flex, a fatura é completada até o mínimo. Um site pequeno com um admin e um domínio paga apenas o mínimo.

### Visualizando A Fatura

A página [Billing Analytics](https://fastcomments.com/auth/my-account/analytics/billing) mostra cada linha da próxima fatura com os números de hoje, para que não haja surpresas. As contagens de page loads, API e widgets pequenos podem estar até um minuto atrasadas; o resto é em tempo real.

### Outras Moedas

O uso do Flex é precificado em dólares americanos. Se a moeda da sua fatura for outra, cada linha é convertida pela taxa de câmbio do dia em que a fatura é criada, então o mesmo uso pode custar valores ligeiramente diferentes de mês para mês. Veja [Currencies](#currencies).
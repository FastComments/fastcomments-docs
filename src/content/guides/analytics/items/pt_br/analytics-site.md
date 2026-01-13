Analytics do site, ou simplesmente chamado `Analytics` no painel, fornece uma visão geral de como sua comunidade está usando o FastComments em todos os seus domínios.

FastComments fornece alguns recursos únicos que muitas outras plataformas não oferecem, como relatórios **ao vivo** de usuários online em cada página, e ordenação de páginas pelo número de usuários online. Para fazer isso, simplesmente visite a [Página de Analytics](https://fastcomments.com/auth/my-account/analytics) e clique em `Ordenar por usuários online` em `Páginas principais`.

Tanto as métricas de `Usuários Online` quanto de `Páginas principais` são ao vivo e reportadas sem atraso.

`Páginas principais` por padrão ordenará pelo número de comentários em cada página.

Finalmente, é fornecido um detalhamento para métricas totais em seu tenant, por dia, ao longo do tempo para:

- Carregamentos de página
  - Este é o número de vezes que um usuário abriu uma página que contém um ou mais widgets FastComments. Se a página contiver múltiplos widgets, este número será incrementado pelo número de widgets naquela página. Se você tiver uma SPA, cada vez que a aplicação abrir um novo thread de comentários, este número seria incrementado. Isso se aplica também à biblioteca React Native.
  - Esta métrica também é usada para fins de faturamento nos planos Flex.
- Comentários deixados
  - Isso inclui todos os comentários, independentemente do estado de verificação ou aprovação, ou se são spam ou não.
- Votos deixados
  - Isso é para o número de votos deixados. Contará apenas votos verificados, a menos que a votação anônima esteja habilitada.
- Contas criadas
  - Esta métrica é para quando um novo usuário SSO é adicionado, ou um comentarista comenta com FastComments pela primeira vez usando seu site.

Essas métricas são quase em tempo real, com atraso de até um minuto.

Por padrão, o FastComments vem com detecção de spam treinável.

À medida que você modera comentários e os marca como **Spam**, ou marca comentários detectados automaticamente como **Spam** como **Não é Spam**, o sistema de detecção de spam aprenderá com essas ações para determinar com mais precisão o que você considera spam.

Comentários marcados como **Spam** não serão aprovados automaticamente, portanto não serão exibidos até serem explicitamente marcados como **Não é Spam**.

A detecção de spam pode ser desativada na página de Configurações de Moderação de Comentários.

### Diferentes Detectores de Spam

FastComments suporta três formas de detectar spam:

1. Um classificador Naïve-Bayes tradicional que é continuamente treinado e é compartilhado entre todos os tenants do FastComments.com.
2. Um classificador Naïve-Bayes tradicional que é continuamente treinado e que é **isolado** ao seu tenant.
3. Uso do ChatGPT 4.

Todos têm acesso aos classificadores Naïve-Bayes compartilhados e isolados.

A opção ChatGPT 4 pode ser selecionada na página de Configurações de Moderação de Comentários se você estiver no Flex billing, pois ela é cobrada com base nos tokens usados.

### Fator de Confiança

O FastComments ajusta o filtro de spam de um usuário com base em quanto ele é confiável para o site em questão.

Por exemplo, se administradores fixaram muitos de seus comentários, provavelmente esse usuário é muito confiável. Ou, se tiver sido membro do site por muito tempo e possuir muitos comentários, seu fator de confiança também pode ser alto.

### SSO

Comentários postados por usuários SSO podem ser considerados spam e serão verificados como tal. A exceção é se o usuário SSO tiver o mesmo e-mail que um usuário do tenant que possua uma ou mais das seguintes permissões:

- Account Owner
- Super Admin
- Comment Moderator Admin

Usuários SSO com essas permissões não terão seus comentários verificados quanto a spam.

### Mensagens Repetidas

FastComments detectará e impedirá mensagens repetidas. Também detectará mensagens repetidas que sejam muito semelhantes para ajudar a prevenir spam. Isso não pode ser desativado, pois evita que nossa plataforma seja usada para abuso. Se você tiver um alto fator de confiança, isso é levado em conta ao aplicar a prevenção de mensagens repetidas.
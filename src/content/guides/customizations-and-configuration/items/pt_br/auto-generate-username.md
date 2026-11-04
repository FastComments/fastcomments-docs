Quando os usuários comentam ou votam, e não estão logados, eles serão solicitados a fornecer seu e-mail e nome de usuário.

Para alguns sites, pedir a um visitante que invente um nome de usuário único é uma barreira, especialmente no celular. O FastComments pode
gerar um nome de usuário neutro para cada novo visitante e pré-preencher o campo de nome de usuário, como `BraveOtter4172`.

O visitante pode mantê-lo como está, ou substituí-lo por um nome de sua escolha.

Isso pode ser ativado na UI de Customização, na configuração chamada `Generate Usernames Automatically`:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.auto-generate-username'; alt='A opção Generate Usernames Automatically na UI de customização do widget'; title='Gerar Nomes de Usuário Automaticamente' app-screenshot-end]

#### Como Funciona

- Cada nome gerado é único. Ele é verificado contra contas existentes e reservado para a sessão do navegador daquele visitante, então dois visitantes não recebem o mesmo nome.
- O nome só é gerado para visitantes que ainda não têm um. Usuários logados, usuários SSO e visitantes que já comentaram mantêm seu nome existente.
- Funciona com ou sem [comentários anônimos](/guide-customizations-and-configuration.html#allow-anon). Com os comentários anônimos desativados, o visitante ainda insere seu e-mail, mas não precisa mais pensar em um nome de usuário.
- Um visitante recorrente que insere um e-mail que já usou antes é correspondido à sua conta existente e mantém o nome dessa conta.
- Se um [Nome de Usuário Padrão](/guide-customizations-and-configuration.html#default-username) também estiver definido, o nome gerado tem precedência.
When users comment or vote, and they are not logged in, they will be asked to provide their email and username.

For some sites, asking a visitor to invent a unique username is a hurdle, particularly on mobile. FastComments can
generate a neutral username for each new visitor and prefill it in the username field, like `BraveOtter4172`.

The visitor can leave it as-is, or replace it with a name of their choosing.

This can be enabled from the Customization UI, under the setting called `Generate Usernames Automatically`:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.auto-generate-username'; alt='A opção Gerar nomes de usuário automaticamente na UI de personalização do widget'; title='Gerar nomes de usuário automaticamente' app-screenshot-end]

#### Como funciona

- Cada nome gerado é único. Ele é verificado em relação às contas existentes e reservado para a sessão do navegador desse visitante, de modo que dois visitantes não recebam o mesmo nome.
- O nome é gerado apenas para visitantes que ainda não têm um. Usuários conectados, usuários SSO e visitantes que já comentaram mantêm seu nome existente.
- Ele funciona com ou sem [anonymous commenting](/guide-customizations-and-configuration.html#allow-anon). Com o comentário anônimo desativado, o visitante ainda insere seu e‑mail, mas não precisa mais pensar em um nome de usuário.
- Um visitante que retorna e insere um e‑mail que já usou anteriormente é associado à sua conta existente e mantém o nome nessa conta.
- Se um [Default Username](/guide-customizations-and-configuration.html#default-username) também estiver definido, o nome gerado tem precedência.
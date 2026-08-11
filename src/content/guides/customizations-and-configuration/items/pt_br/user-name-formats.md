Por padrão, o FastComments exibirá o nome do usuário exatamente como ele foi inserido, ou como foi passado para nós via SSO.

Entretanto, pode ser desejável mascarar ou exibir o nome do usuário de uma forma diferente. Por exemplo, se o nome do usuário for Allen Rex, talvez você queira exibir apenas “Allen R.”.

Isso pode ser feito sem código na UI de Personalização do Widget, na configuração chamada `Commenter Name Format`:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.commenter-name-format select'; selector = '.commenter-name-format'; alt='Menu suspenso Formato do Nome do Comentador aberto com opções como Capitalizar, Inicial do Último e Todas as Iniciais'; title='Alterar Formato do Nome' app-screenshot-end]

Os formatos disponíveis são:

- Capitalizar (exibir usuário de exemplo como Exemplo Usuário)
- Inicial do Último (exibir Exemplo Usuário como Exemplo U.)
- Todas as Iniciais (exibir Exemplo Usuário como E. U.)
- Mostrar "Anônimo"

O efeito da alteração é imediato. Os usuários ainda verão seu nome de usuário completo no topo da área de comentários, para eles mesmos, mas seus comentários exibirão o nome de usuário modificado.

Os nomes de usuário são mascarados no lado do servidor para proteger os usuários.
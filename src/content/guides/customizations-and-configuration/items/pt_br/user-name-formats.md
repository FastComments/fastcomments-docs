Por padrão, FastComments exibirá o nome do usuário como ele o inseriu, ou como foi nos fornecido via SSO.

No entanto, pode ser desejável mascarar ou exibir o nome do usuário de uma forma diferente. Por exemplo, se o nome do usuário for Allen Rex, talvez você queira mostrar apenas "Allen R.".

Isso pode ser feito sem código na Interface de Personalização do Widget, na configuração chamada `Commenter Name Format`:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.commenter-name-format select'; selector = '.commenter-name-format'; title='Change Name Format' app-screenshot-end]

The available formats are:

- Capitalize (exibe example user como Example User)
- Last Initial (exibe Example User como Example U.)
- All Initials (exibe Example User como E. U.)
- Show "Anonymous"

O efeito dessa alteração é imediato. Os usuários ainda verão seu nome de usuário completo no topo da área de comentários, para si mesmos, mas seus comentários exibirão o nome de usuário modificado.

Os nomes de usuário são mascarados no servidor para proteger os usuários.
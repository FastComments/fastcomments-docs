#### Como os Comentários Aparecem em Seus Cursos

Assim que a integração LTI estiver habilitada e o Aplicativo Externo instalado, o FastComments funciona automaticamente com base nas posições (placements) que você configurou:

#### Visualização de Atividade (Assignment View)

Se a posição **Assignment View** estiver habilitada, os comentários aparecem automaticamente abaixo de cada atividade do curso. Alunos e instrutores veem uma seção de comentários encadeados ao visualizar uma atividade — nenhuma configuração extra é necessária por atividade.

Cada atividade recebe sua própria thread de comentários separada.

#### Botão do Editor de Conteúdo Rico

Se a posição **Editor Button** estiver habilitada, os instrutores podem incorporar o FastComments em qualquer conteúdo que use o Rich Content Editor:

1. Edite uma **Página**, um **Quiz** ou um **Anúncio**.
2. Na barra de ferramentas do Rich Content Editor, clique no botão **FastComments**.
3. O FastComments é incorporado automaticamente ao conteúdo.
4. Salve a página.

Quando os alunos visualizarem a página, o widget do FastComments incorporado será carregado com uma thread de comentários exclusiva daquela página.

#### SSO Automático

Em ambas as posições, os alunos entram automaticamente via sua conta do Canvas. Nomes, e-mails e avatares são sincronizados por meio do lançamento LTI, sem necessidade de login separado.

#### Restrinja o Acesso Público (Recomendado)

Por padrão, os dados de comentários do FastComments são legíveis publicamente. Qualquer pessoa que adivinhe a URL de uma thread ou um endpoint da API pode visualizar seus comentários, mesmo fora do Canvas. Para discussões de cursos, você quase certamente desejará restringir a visualização apenas aos alunos matriculados.

Abra sua <a href="https://fastcomments.com/auth/my-account/customize-widget" target="_blank">página de personalização do widget</a> e crie uma regra com **Require SSO To View Comments** habilitado e, em seguida, defina o nível de segurança como **Secure SSO** para que as threads só possam ser carregadas por meio do lançamento LTI assinado.

Consulte [Protegendo Threads de Comentários com Single-Sign-On](/guide-customizations-and-configuration.html#sso-require-to-view-comments) para o passo a passo completo, incluindo como limitar a regra a um único domínio ou página.
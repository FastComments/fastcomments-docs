FastComments SSO (<a href="#sso">detalhes aqui</a>) oferece aos seus usuários uma maneira de comentar sem precisar fazer login em outra plataforma.

No entanto, isso por si só não protege seus tópicos de comentários, já que, por padrão, os dados dos comentários são informações publicamente disponíveis – qualquer pessoa que possa visualizar a página pode ver os comentários.

Alterando uma configuração, podemos restringir a obtenção de comentários, a menos que seja feita por um administrador ou usuário SSO válido.

#### Configuração sem Código

Podemos impedir a visualização e a interação com nossos tópicos de comentários, quando o SSO está configurado, criando uma <a href="https://fastcomments.com/auth/my-account/customize-widget" target="_blank">regra de personalização</a>.

Ao fazer isso, procure por SSO, e você encontrará esta opção:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.require-sso'; selector = '.require-sso'; alt='Opção Exigir SSO para visualizar comentários ativada em uma regra de personalização, com a escolha do nível de segurança'; title='Exigir SSO para Visualizar Comentários' app-screenshot-end]

Ative-a e salve a regra de personalização.

#### Proteger Apenas um Domínio ou Página Específico

Para proteger apenas um determinado Domínio ou Página, basta configurar a regra de personalização para isso.

No topo da interface de personalização, encontraremos dois campos de entrada, Domínio e ID da URL.

Para proteger apenas um domínio específico, insira o domínio em questão no campo "domain".

Para proteger uma página específica, insira a URL da página no campo "URL ID". Se você tem uma integração personalizada com o FastComments, pode inserir um tipo de ID aqui em vez de uma URL.

#### Níveis de Segurança

Ao exigir SSO, você precisará decidir se requer SSO Simples ou SSO Seguro. Se você exigir SSO Simples, ambos são permitidos, mas se exigir SSO Seguro, o conteúdo deve ser obtido com um payload de SSO Seguro hashado com sua chave de API para ser visualizado.

A opção de nível de segurança aparecerá quando você selecionar "Exigir SSO para Visualizar Comentários".

#### Proteção Além da Leitura

Ativar esta opção protegerá a página ou domínio de ser comentado, a menos que o usuário esteja conectado via SSO.

#### Armadilhas

Qualquer usuário que tenha criado comentários antes da sua integração SSO não poderá vê-los, a menos que faça login via sua integração SSO.
FastComments SSO (<a href="#sso">detalhes aqui</a>) fornece aos seus usuários uma maneira de comentar sem precisar fazer login em outra plataforma.

No entanto, isso por si só não protege seus tópicos de comentários, já que por padrão os dados dos comentários são informação pública - qualquer pessoa que possa ver a página pode ver os comentários.

Ao alterar uma configuração, podemos restringir o acesso aos comentários para que só possam ser recuperados por um administrador ou por um usuário SSO válido.

#### No-Code Setup

Podemos impedir a visualização e interação com nossos tópicos de comentários, quando o SSO estiver configurado, criando uma <a href="https://fastcomments.com/auth/my-account/customize-widget" target="_blank">regra de personalização</a>.

Ao fazer isso, pesquise por SSO, e você encontrará esta opção:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.require-sso'; selector = '.require-sso'; title='Require SSO To View Comments' app-screenshot-end]

Ative-a e salve a regra de personalização.

#### Only Protect a Certain Domain or Page

Para proteger apenas um determinado domínio ou página, nós simplesmente configuraremos a regra de personalização para isso.

No topo da interface de personalização, encontraremos dois campos, Domain e URL ID.

Para proteger apenas um domínio específico, insira o domínio em questão no campo "domain".

Para proteger uma página em particular, insira a URL da página no campo "URL ID". Se você tiver uma integração personalizada com o FastComments, você pode inserir um tipo de ID aqui em vez de uma URL.

#### Security Levels

Ao exigir SSO, você deverá decidir se exige Simple SSO ou Secure SSO. Se você exigir Simple SSO, então ambos são permitidos, mas se você exigir Secure SSO então
o conteúdo deve ser buscado com um payload Secure SSO hashed com sua API key para que possa ser visualizado.

A opção de nível de segurança aparecerá quando você selecionar "Require SSO To View Comments".

#### Protection Beyond Reading

Habilitar essa opção protegerá a página ou domínio de receber comentários, a menos que o usuário esteja logado via SSO.

#### Observações

Quaisquer usuários que criaram comentários antes da sua integração SSO não poderão vê-los, a menos que façam login através da sua integração SSO.
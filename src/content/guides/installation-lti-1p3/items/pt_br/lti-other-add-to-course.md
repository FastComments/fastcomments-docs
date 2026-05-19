Uma vez que o FastComments esteja registrado na plataforma, os instrutores o adicionam ao conteúdo do curso usando os fluxos padrão de ferramenta externa da plataforma. Esta página cobre Sakai 23.x e Schoology Enterprise.

#### Restringir Acesso Público (Recomendado)

Por padrão, os dados de comentários do FastComments são legíveis publicamente em ambas as plataformas. Qualquer pessoa que consiga adivinhar a URL do thread ou o endpoint da API pode ver os comentários, mesmo fora do Sakai ou Schoology. Para discussões de curso, você quase certamente vai querer restringir a visualização apenas aos estudantes matriculados.

Abra sua <a href="https://fastcomments.com/auth/my-account/customize-widget" target="_blank">página de personalização do widget</a> e crie uma regra com **Require SSO To View Comments** habilitado, então defina o nível de segurança para **Secure SSO** para que os threads só possam ser carregados através do lançamento LTI assinado.

Veja [Protecting Comment Threads With Single-Sign-On](/guide-customizations-and-configuration.html#sso-require-to-view-comments) para o passo a passo completo, incluindo como escopar a regra para um único domínio ou página.

#### Sakai

**1. Adicionar o FastComments a um site**

O mantenedor do site habilita a ferramenta por site:

1. Abra o site e clique em **Site Info** na navegação à esquerda.
2. Clique em **Manage Tools**.
3. Role até a lista **External Tools** e ative **FastComments**.
4. Clique em **Continue**, revise a lista de ferramentas e então clique em **Finish**.

O FastComments agora aparece como um item na navegação à esquerda no site.

**2. Reordenar a entrada na navegação à esquerda**

Vá em **Site Info** > **Tool Order**. Arraste **FastComments** para a posição desejada e clique em **Save**. Você também pode renomear o rótulo da navegação e ocultá-lo dos estudantes nesta tela.

**3. Incorporar inline em uma página Lessons**

Para colocar o FastComments diretamente dentro de uma página Lessons em vez de como uma ferramenta independente na navegação à esquerda:

1. Abra a ferramenta **Lessons** no site.
2. Clique em **Add Content** > **Add External Tool**.
3. Selecione **FastComments** na lista.
4. Se o FastComments declarou Deep Linking durante o registro, o Sakai abre o seletor de conteúdo da ferramenta para que você possa escolher ou nomear o thread. Se o Deep Linking não foi declarado, o Sakai insere um link de lançamento padrão.
5. Salve o item do Lessons.

Cada instância incorporada recebe seu próprio thread, escopado para esse link de recurso.

**4. Ajustes de permissão para acesso dos estudantes**

O Sakai controla os lançamentos de ferramentas externas através de Realms. Para confirmar que os estudantes podem iniciar o FastComments:

1. Faça login como administrador do Sakai e abra **Administration Workspace** > **Realms**.
2. Abra o realm relevante (por exemplo, `!site.template.course` ou o realm do site específico).
3. Confirme que o papel `access` tem `lti.launch` habilitado e que as permissões de papel no grupo **external.tools** estão concedidas.
4. Salve o realm.

Para sobrescritas em nível de site, o mantenedor pode ajustar a visibilidade da ferramenta por papel em **Site Info** > **Tool Order** ocultando ou mostrando o FastComments por papel.

**5. O que os estudantes veem**

Os estudantes clicam no item FastComments na navegação à esquerda (ou rolam até o bloco Lessons incorporado) e caem diretamente na visualização de comentários em thread. O SSO é automático: o Sakai envia a identidade do usuário no lançamento LTI e o FastComments os autentica sob a conta do Sakai.

Mapeamento de papéis:

- Sakai `Instructor` -> FastComments moderator
- Sakai `Admin` (admin in Administration Workspace) -> FastComments admin
- Sakai `Student` / `access` -> FastComments commenter

**6. Armadilhas do Sakai**

- **Ferramenta não visível em Manage Tools.** Se o FastComments não aparecer na lista External Tools, o administrador do Sakai precisa abrir o registro de ferramentas (**Administration Workspace** > **External Tools** > **FastComments**) e definir **Stealthed** como `false`. Ferramentas stealthed são ocultas do seletor Manage Tools por site.
- **Lançamentos quebrando em navegadores com sessão compartilhada.** O token CSRF do portal do Sakai está vinculado à sessão do navegador. Se um estudante estiver logado em dois sites Sakai em abas diferentes ou tiver uma sessão obsoleta, o lançamento retorna um 403. Correção: feche outras abas do Sakai, faça logout, entre novamente e relance. Administradores também podem aumentar `sakai.csrf.token.cache.ttl` se isso ocorrer em todo o cluster.
- **Incorporação em iframe.** Confirme que `lti.frameheight` em `sakai.properties` seja grande o suficiente (600 ou mais) para que o thread de comentários não fique cortado dentro de uma página Lessons.

#### Schoology

O Schoology Enterprise tem dois cenários de instalação. Confirme qual se aplica antes de adicionar a ferramenta a um curso.

**1. Dois cenários de instalação**

- **(a) Instalação a nível de Enterprise.** O System Administrator do Schoology instalou o FastComments no nível da organização e o atribuiu a todos os cursos ou a modelos de curso específicos. Os instrutores pulam a instalação e vão direto para "Add Materials".
- **(b) Auto-instalação pelo instrutor.** O instrutor instala a ferramenta em um único curso em **Course Options** > **External Tools** > **Install LTI Apps**. A auto-instalação requer que o System Administrator tenha aprovado o app FastComments no nível da organização primeiro.

**2. Adicionar o FastComments como material do curso**

Dentro do curso:

1. Abra o curso e vá para **Materials**.
2. Clique em **Add Materials** > **Add File/Link/External Tool**.
3. Escolha **External Tool**.
4. Selecione **FastComments** na lista de ferramentas registradas.
5. Defina um **Name** (é isso que os estudantes veem na lista de materiais) e uma **Description** opcional.
6. Deixe **Enable Grading** (grade passback) **OFF**. O FastComments não reporta notas de volta ao Schoology, então ativar o envio de notas cria uma coluna vazia no gradebook.
7. Clique em **Submit**.

O material agora aparece na lista de materiais do curso e abre o thread do FastComments quando clicado.

**3. Incorporação inline via o editor Rich Text**

Se o System Administrator habilitou a colocação Deep Linking para o FastComments durante o registro, os instrutores podem incorporar o thread de comentários dentro de qualquer campo Rich Text (instruções de tarefa, corpos de página, prompts de discussão):

1. Abra o editor Rich Text na página alvo.
2. Clique no ícone **External Tool** (peça de quebra-cabeça) na barra de ferramentas.
3. Escolha **FastComments**.
4. Configure a incorporação no diálogo de deep-linking e clique em **Insert**.
5. Salve a página.

Se o botão External Tool não aparecer no editor Rich Text, o Deep Linking está desabilitado para esta ferramenta nesse tenant. Veja as armadilhas abaixo.

**4. Visibilidade e atribuições por seção**

O Schoology define a disponibilidade da ferramenta por seção através de Course Options:

1. No curso, clique em **Course Options** > **External Tools**.
2. Para cada app LTI instalado, você controla se ele está disponível para todas as seções do curso ou para seções específicas.
3. Para restringir o FastComments a certas seções, desmarque as seções que não devem ver a ferramenta.
4. O acesso por seção também controla quais seções veem a entrada **Add Materials** > **External Tool** para o FastComments.

**5. O que os estudantes veem**

Os estudantes clicam no material FastComments (ou rolam até a incorporação inline) e caem na discussão em thread. O SSO é automático via o lançamento LTI do Schoology sob a conta deles no Schoology.

Mapeamento de papéis:

- Schoology `Administrator` -> FastComments admin
- Schoology `Instructor` -> FastComments moderator
- Schoology `Student` -> FastComments commenter

**6. Armadilhas do Schoology**

- **Somente Enterprise.** Contas pessoais e gratuitas do Schoology não conseguem instalar ferramentas LTI 1.3. Se seu tenant estiver no nível gratuito, a opção **External Tools** estará ausente em Course Options. Faça upgrade para Schoology Enterprise para usar o FastComments.
- **Deep Linking desabilitado por padrão no tenant.** Alguns tenants do Schoology restringem a colocação Deep Linking no nível da organização. Quando isso acontece, os instrutores veem apenas o fluxo **Add Materials** > **External Tool** e não o botão External Tool no editor Rich Text. Para habilitar a incorporação inline, o System Administrator vai em **System Settings** > **Integration** > **LTI 1.3** > **FastComments** e habilita a colocação **Content Item / Deep Linking**, então salva.
- **Sobrescrita de atribuição por seção.** Se o FastComments estiver atribuído no nível da enterprise mas o instrutor não conseguir vê-lo em **Add Materials**, a seção do curso está excluída na atribuição do app no nível da organização. Peça ao System Administrator para adicionar a seção à atribuição do app FastComments.
- **Nome do material vs. identidade do thread.** Renomear o material no Schoology não move o thread de comentários. Threads são indexados pelo LTI resource link ID, então uma renomeação mantém o mesmo thread; excluir e recriar o material cria um thread novo e vazio.
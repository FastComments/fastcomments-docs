Esta página aborda como adicionar o FastComments a um curso do Brightspace depois que um administrador registrou a ferramenta e criou uma implantação. Se a ferramenta ainda não estiver registrada, consulte primeiro o guia de registro do D2L.

<div class="screenshot white-bg">
    <div class="title">FastComments incorporado como um tópico de unidade no Brightspace</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-d2l-comments-in-unit.png" alt="FastComments em execução dentro de uma unidade do Brightspace, mostrando comentários encadeados e um seletor de @menções" />
</div>

O Brightspace oferece duas experiências de criação de conteúdo: **Classic Content** e a **New Content Experience** (também chamada de **Lessons**). Ambas expõem o FastComments, mas os caminhos dos menus diferem. Cada seção abaixo aborda ambos onde eles divergem.

#### Localizar a ferramenta FastComments

A ferramenta FastComments aparece em dois lugares dentro de um editor de conteúdo do curso:

1. O seletor de atividades, acessado pelo botão **Add Existing** de um módulo/unidade (rotulado como **Add Existing Activities** em versões mais antigas do Brightspace). O FastComments aparece diretamente no seletor nas versões atuais do Brightspace; versões mais antigas o aninham sob um submenu **External Learning Tools**. Qualquer um dos caminhos adiciona o FastComments como um tópico independente.
2. A caixa de diálogo **Insert Stuff** dentro do editor HTML, em **LTI Advantage**. Isso incorpora o FastComments inline em um tópico HTML por meio do fluxo de deep linking do LTI.

Se o FastComments não aparecer em nenhum dos seletores, a implantação não está habilitada para a unidade organizacional (org unit) que contém o curso. Peça ao seu administrador do Brightspace para abrir **Admin Tools** > **Manage Extensibility** > **LTI Advantage** > ferramenta FastComments > **View Deployments**, abrir a implantação e adicionar a unidade organizacional do curso (ou uma unidade organizacional pai) em **Org Units**.

#### Adicionar o FastComments como um tópico em um módulo

Classic Content:

1. Abra o curso e clique em **Content** na barra de navegação.
2. Selecione o módulo que deve conter a discussão (ou crie um via **Add a module**).
3. Clique em **Add Existing** (Brightspace mais antigo: **Add Existing Activities** > **External Learning Tools**).
4. No seletor, clique em **FastComments**. O Brightspace cria um tópico no módulo e retorna você à visualização de conteúdo.
5. Clique no novo tópico. Renomeie-o para algo descritivo como `FastComments Discussion` usando o editor de título inline.

New Content Experience (Lessons):

1. Abra o curso e clique em **Content**.
2. Abra a unidade e a lição que devem conter a discussão.
3. Clique em **Add** > **Existing Activity** e selecione **FastComments** (Brightspace mais antigo: aninhado em **External Learning Tools**).
4. A atividade é adicionada à lição.
5. Clique no título da atividade para renomeá-la.

Na primeira vez que qualquer usuário (instrutor ou estudante) abrir o tópico, o FastComments inicializa a thread para aquele resource link. A thread está vinculada ao ID do resource link, portanto renomear ou mover o tópico não altera qual thread é carregada.

#### Incorporar o FastComments inline em um tópico HTML

Use este fluxo quando quiser que os comentários apareçam abaixo de uma leitura, vídeo ou outro conteúdo dentro da mesma página do tópico, em vez de como um tópico separado.

1. Abra ou crie um tópico HTML no módulo/lição.
2. Clique em **Edit HTML** para abrir o editor HTML do Brightspace.
3. Posicione o cursor onde a thread de comentários deve aparecer.
4. Clique no botão **Insert Stuff** (ícone de peça de quebra-cabeça na barra de ferramentas do editor).
5. Na caixa de diálogo Insert Stuff, role até **LTI Advantage** e clique em **FastComments**.
6. O FastComments abre um seletor de deep linking. Confirme o posicionamento (as opções padrão funcionam para discussões de conteúdo); clique em **Insert** ou **Continue**.
7. O Brightspace retorna ao editor HTML com um bloco de espaço reservado representando o lançamento LTI. Clique em **Save and Close** no tópico.

Quando o tópico é carregado, o Brightspace substitui o espaço reservado por um iframe que lança automaticamente o FastComments via LTI. Os estudantes veem a thread de discussão inline.

Um único tópico HTML pode conter múltiplas incorporações do FastComments com deep link. Cada incorporação tem sua própria thread porque cada deep link produz um ID de resource link distinto.

#### Tópico de módulo vs. Quicklink inline

Escolha a abordagem de **tópico de módulo** quando:

- A discussão é a atividade principal daquela etapa do módulo.
- Você quer que o tópico apareça no sumário do Brightspace, no rastreamento de conclusão e no Class Progress.

Escolha a abordagem de **incorporação inline** quando:

- Os comentários devem ficar abaixo de outro conteúdo na mesma página.
- Você não quer um item separado rastreável para conclusão no sumário.

#### Visibilidade, rascunho e condições de liberação

Um novo tópico do FastComments é visível para os estudantes por padrão. Para ocultá-lo enquanto você o configura:

1. No editor de conteúdo, clique no título do tópico (Classic) ou no menu de três pontos da atividade (New Content Experience).
2. Defina o status como **Draft** (Classic) ou desative **Visibility** (New Content Experience).

Tópicos em rascunho ficam invisíveis para os estudantes. Instrutores e TAs ainda os veem com um selo "Draft".

Para restringir o tópico a um grupo ou seção específica:

1. Abra o tópico.
2. Clique no menu do título do tópico > **Edit Properties In-place** (Classic) ou **Edit** > **Restrictions** (New Content Experience).
3. Em **Release Conditions**, clique em **Create**.
4. Escolha **Group enrollment** ou **Section enrollment**, selecione o grupo/seção e salve.

As condições de liberação se acumulam com o mapeamento de papéis do próprio FastComments. Estudantes que não podem ver o tópico não recebem um lançamento LTI.

#### O que os estudantes veem no primeiro lançamento

Quando um estudante clica no tópico (ou carrega um tópico HTML com uma incorporação):

1. O Brightspace realiza o lançamento LTI 1.3 em segundo plano.
2. O FastComments recebe o nome, e-mail, URL do avatar e papel no LMS do estudante, e o conecta automaticamente. Não há tela de login do FastComments.
3. A thread de comentários daquele resource link é renderizada dentro do iframe do Brightspace.

Mapeamento de papéis no lançamento:

- O `Administrator` do Brightspace torna-se um **admin** do FastComments para a thread (moderação completa, exclusão, banimento e acesso à configuração).
- O `Instructor` do Brightspace torna-se um **moderador** do FastComments (fixar, ocultar, excluir, banir).
- Todos os outros papéis (`Learner`, `TeachingAssistant`, etc.) tornam-se comentaristas padrão.

Os comentários são atribuídos à conta do Brightspace do estudante. Se o estudante editar seu nome ou avatar no Brightspace, o próximo lançamento LTI sincroniza a alteração.

#### Restringir o acesso público (recomendado)

Por padrão, os dados de comentários do FastComments são publicamente legíveis. Qualquer pessoa que adivinhe a URL de uma thread ou um endpoint de API pode visualizar seus comentários, mesmo fora do Brightspace. Para discussões de curso, você quase certamente deseja restringir a visualização apenas aos alunos matriculados.

Abra sua <a href="https://fastcomments.com/auth/my-account/customize-widget" target="_blank">página de personalização do widget</a> e crie uma regra com **Require SSO To View Comments** habilitado, depois defina o nível de segurança como **Secure SSO** para que as threads só possam ser carregadas por meio do lançamento LTI assinado.

Consulte [Protecting Comment Threads With Single-Sign-On](/guide-customizations-and-configuration.html#sso-require-to-view-comments) para o passo a passo completo, incluindo como limitar a regra a um único domínio ou página.

#### Altura do iframe e redimensionamento

O FastComments emite o postMessage `org.imsglobal.lti.frameResize` em cada renderização de thread e em mudanças de conteúdo (novo comentário, expandir respostas). O Brightspace escuta essa mensagem e ajusta a altura do iframe para que a thread não seja cortada e não mostre uma barra de rolagem interna.

Se o iframe permanecer em uma altura fixa e curta:

- Confirme que o curso é carregado via HTTPS. O listener de postMessage do Brightspace rejeita frames de conteúdo misto.
- Confirme que nenhuma extensão do navegador está bloqueando o canal de postMessage.
- Para incorporações inline em um tópico HTML, o HTML ao redor não deve envolver o iframe em um contêiner de altura fixa. Remova qualquer `style="height: ..."` inline do elemento pai.

#### Particularidades específicas do Brightspace

**Ferramenta não aparece no seletor Add Existing.** A implantação não está habilitada para a unidade organizacional deste curso. Um administrador precisa adicionar a unidade organizacional (ou uma pai) à lista **Org Units** da implantação. Só o registro da ferramenta não é suficiente; a implantação define quais cursos veem a ferramenta.

**Incompatibilidade de `deployment_id` no lançamento.** O FastComments fixa por TOFU o primeiro `deployment_id` que vê para um registro. Se um administrador excluir a implantação original e criar uma nova, os lançamentos da nova implantação serão rejeitados com um erro de incompatibilidade de implantação. A solução é registrar novamente o FastComments (gere uma nova URL de registro (<a href="https://fastcomments.com/auth/my-account/lti-config" target="_blank">obtenha-a aqui</a>) e execute o Dynamic Registration novamente); o registro de configuração antigo é substituído.

**A ferramenta lança, mas mostra "Invalid LTI launch".** O curso está em uma estrutura de tenant/org diferente da coberta pela implantação, ou a implantação foi desabilitada após o registro. Verifique novamente o interruptor **Admin Tools** > **Manage Extensibility** > **LTI Advantage** > FastComments > **Enabled** e a lista de unidades organizacionais da implantação.

**Nomes e papéis ausentes dentro do FastComments.** O Brightspace envia os lançamentos LTI com claims de Names and Role Provisioning Services (NRPS). Se um curso foi atualizado a partir de um link LTI 1.1 mais antigo, o lançamento carece dos claims `name` e `email`. Adicione novamente o tópico do FastComments via **Add Existing** (não migre o link antigo) para que o lançamento use LTI 1.3.

**A incorporação mostra uma tela de login em vez de SSO automático.** O tópico HTML foi inserido como um `<iframe>` simples apontando para o FastComments em vez de via **Insert Stuff** > **LTI Advantage**. Iframes simples pulam o lançamento LTI e levam os usuários à página pública do FastComments. Exclua o iframe e reinsira-o pelo fluxo Insert Stuff.
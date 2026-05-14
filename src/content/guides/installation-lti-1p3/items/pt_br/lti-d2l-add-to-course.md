Esta página descreve como adicionar o FastComments a um curso do Brightspace depois que um administrador registrou a ferramenta e criou uma implantação. Se a ferramenta ainda não estiver registrada, veja primeiro o guia de registro do D2L.

<div class="screenshot white-bg">
    <div class="title">FastComments incorporado como um tópico de unidade no Brightspace</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-d2l-comments-in-unit.png" alt="FastComments em execução dentro de uma unidade do Brightspace, mostrando comentários em threads e um seletor de menções @-mention" />
</div>

O Brightspace oferece duas experiências de autoria de conteúdo: **Classic Content** e a **New Content Experience** (também chamada de **Lessons**). Ambas expõem o FastComments, mas os caminhos dos menus diferem. Cada seção abaixo cobre ambas onde houver divergência.

#### Localizar a ferramenta FastComments

A ferramenta FastComments aparece em dois lugares dentro do editor de conteúdo do curso:

1. No seletor de atividades, acessado a partir do botão **Add Existing** de um módulo/unidade (rotulado **Add Existing Activities** em versões antigas do Brightspace). O FastComments aparece diretamente no seletor nas versões atuais do Brightspace; versões antigas o colocavam dentro de um submenu **External Learning Tools**. Qualquer caminho adiciona o FastComments como um tópico independente.
2. Na caixa de diálogo **Insert Stuff** dentro do editor HTML, em **LTI Advantage**. Isso incorpora o FastComments inline em um tópico HTML via o fluxo de deep linking do LTI.

Se o FastComments não aparecer em nenhum dos seletores, a implantação não está habilitada para a unidade organizacional que contém o curso. Peça ao administrador do Brightspace para abrir **Admin Tools** > **Manage Extensibility** > **LTI Advantage** > ferramenta FastComments > **View Deployments**, abrir a implantação e adicionar a unidade organizacional do curso (ou uma unidade organizacional pai) em **Org Units**.

#### Adicionar o FastComments como um Tópico em um Módulo

Classic Content:

1. Abra o curso e clique em **Content** na barra de navegação.
2. Selecione o módulo que deve conter a discussão (ou crie um via **Add a module**).
3. Clique em **Add Existing** (Brightspace antigo: **Add Existing Activities** > **External Learning Tools**).
4. No seletor, clique em **FastComments**. O Brightspace cria um tópico no módulo e retorna você para a visualização de conteúdo.
5. Clique no novo tópico. Renomeie-o para algo descritivo, como `FastComments Discussion`, usando o editor de título inline.

New Content Experience (Lessons):

1. Abra o curso e clique em **Content**.
2. Abra a unidade e a lesson que devem conter a discussão.
3. Clique em **Add** > **Existing Activity** e selecione **FastComments** (Brightspace antigo: aninhado em **External Learning Tools**).
4. A atividade é adicionada à lesson.
5. Clique no título da atividade para renomeá-la.

Na primeira vez que qualquer usuário (instrutor ou aluno) abrir o tópico, o FastComments inicializa o thread para esse resource link. O thread é vinculado ao resource link ID, então renomear ou mover o tópico não altera qual thread é carregada.

#### Incorporar o FastComments Inline em um Tópico HTML

Use este fluxo quando você quiser que os comentários apareçam abaixo de uma leitura, vídeo ou outro conteúdo dentro da mesma página de tópico, em vez de como um tópico separado.

1. Abra ou crie um tópico HTML no módulo/lesson.
2. Clique em **Edit HTML** para abrir o editor HTML do Brightspace.
3. Posicione o cursor onde o thread de comentários deve aparecer.
4. Clique no botão **Insert Stuff** (ícone de peça de quebra-cabeça na barra de ferramentas do editor).
5. Na caixa de diálogo Insert Stuff, role até **LTI Advantage** e clique em **FastComments**.
6. O FastComments abre um seletor de deep linking. Confirme a colocação (as opções padrão funcionam para discussões de conteúdo); clique em **Insert** ou **Continue**.
7. O Brightspace retorna ao editor HTML com um bloco placeholder que representa o LTI launch. Clique em **Save and Close** no tópico.

Quando o tópico carregar, o Brightspace substitui o placeholder por um iframe que inicia automaticamente o FastComments via LTI. Os alunos veem o thread de discussão inline.

Um único tópico HTML comporta múltiplas incorporações deep-linked do FastComments. Cada incorporação recebe seu próprio thread porque cada deep link produz um resource link ID distinto.

#### Tópico do Módulo vs Incorporação Inline

Escolha a abordagem de **tópico do módulo** quando:

- A discussão for a atividade principal para aquela etapa do módulo.
- Você quiser que o tópico apareça no índice do Brightspace, no rastreamento de conclusão e no Class Progress.

Escolha a abordagem de **incorporação inline** quando:

- Os comentários devem ficar abaixo de outro conteúdo na mesma página.
- Você não quer um item separado rastreável por conclusão no índice.

#### Visibilidade, Rascunho e Condições de Liberação

Um novo tópico do FastComments é visível para os alunos por padrão. Para ocultá-lo enquanto você o configura:

1. No editor de conteúdo, clique no título do tópico (Classic) ou no menu de três pontos na atividade (New Content Experience).
2. Defina o status para **Draft** (Classic) ou desative a **Visibility** (New Content Experience).

Tópicos em rascunho são invisíveis para os alunos. Instrutores e TAs ainda os veem com um selo "Draft".

Para restringir o tópico a um grupo ou seção específica:

1. Abra o tópico.
2. Clique no menu do título do tópico > **Edit Properties In-place** (Classic) ou **Edit** > **Restrictions** (New Content Experience).
3. Em **Release Conditions**, clique em **Create**.
4. Escolha **Group enrollment** ou **Section enrollment**, selecione o grupo/seção e salve.

As condições de liberação se somam ao próprio mapeamento de funções do FastComments. Estudantes que não puderem ver o tópico não recebem um LTI launch.

#### O que os Alunos Veem no Primeiro Launch

Quando um aluno clica no tópico (ou carrega um tópico HTML com uma incorporação):

1. O Brightspace executa o LTI 1.3 launch em segundo plano.
2. O FastComments recebe o nome do aluno, email, URL do avatar e a função no LMS, e os autentica automaticamente. Não há prompt de login do FastComments.
3. O thread de comentários para aquele resource link é renderizado dentro do iframe do Brightspace.

Mapeamento de funções no launch:

- Brightspace `Administrator` torna-se um **admin** do FastComments para o thread (moderação completa, excluir, banir e acesso à configuração).
- Brightspace `Instructor` torna-se um **moderator** do FastComments (pin, hide, delete, ban).
- Todas as outras funções (`Learner`, `TeachingAssistant`, etc.) tornam-se comentaristas padrão.

Os comentários são atribuídos à conta Brightspace do aluno. Se o aluno editar seu nome ou avatar no Brightspace, o próximo LTI launch sincroniza a alteração.

#### Altura do Iframe e Redimensionamento

O FastComments emite a postMessage `org.imsglobal.lti.frameResize` em toda renderização de thread e em alterações de conteúdo (novo comentário, expandir respostas). O Brightspace escuta essa mensagem e ajusta a altura do iframe para que o thread não seja cortado e para evitar uma barra de rolagem interna.

Se o iframe permanecer com altura fixa e curta:

- Confirme que o curso está sendo carregado via HTTPS. O listener postMessage do Brightspace rejeita frames com conteúdo misto.
- Confirme se nenhuma extensão do navegador está bloqueando o canal postMessage.
- Para incorporações inline em um tópico HTML, o HTML ao redor não deve envolver o iframe em um contêiner de altura fixa. Remova qualquer `style="height: ..."` inline do elemento pai.

#### Particularidades do Brightspace

**Tool not showing in the Add Existing picker.** A implantação não está habilitada para a unidade organizacional deste curso. Um administrador precisa adicionar a unidade organizacional (ou um pai) à lista de Unidades da implantação. O registro da ferramenta por si só não é suficiente; a implantação define o escopo de quais cursos veem a ferramenta.

**`deployment_id` mismatch on launch.** O FastComments faz TOFU-pin no primeiro `deployment_id` que encontra para um registro. Se um administrador excluir a implantação original e criar uma nova, launches da nova implantação são rejeitados com um erro de mismatch de implantação. A correção é re-registrar o FastComments (gerar uma nova URL de registro e executar o Dynamic Registration novamente); o registro de configuração antigo é substituído.

**Tool launches but shows "Invalid LTI launch".** O curso está em uma estrutura de tenant/org diferente daquela coberta pela implantação, ou a implantação foi desativada após o registro. Verifique novamente **Admin Tools** > **Manage Extensibility** > **LTI Advantage** > FastComments > alternador **Enabled** e a lista de unidades organizacionais da implantação.

**Names and roles missing inside FastComments.** O Brightspace envia launches LTI com as claims NRPS (Names and Role Provisioning Services). Se um curso foi atualizado a partir de um link LTI 1.1 antigo, o launch pode não conter as claims `name` e `email`. Re-adicione o tópico do FastComments via **Add Existing** (não migre o link antigo) para que o launch use LTI 1.3.

**Embed shows a login screen instead of auto-SSO.** O tópico HTML foi inserido como um `<iframe>` simples apontando para o FastComments em vez de via **Insert Stuff** > **LTI Advantage**. Iframes simples ignoram o LTI launch e direcionam os usuários para a página pública do FastComments. Exclua o iframe e reinserir via o fluxo Insert Stuff.
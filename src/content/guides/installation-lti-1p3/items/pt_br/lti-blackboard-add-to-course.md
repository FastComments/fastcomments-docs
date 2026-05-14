Uma vez que um administrador registrou o FastComments como uma ferramenta LTI 1.3 Advantage e aprovou as políticas da instituição, os instrutores o adicionam aos cursos através dos pontos de posicionamento padrão do Blackboard. Os passos exatos diferem entre Ultra Course View e Original Course View, então ambos são cobertos abaixo.

#### Ultra Course View

Ultra Course View é o padrão no Blackboard Learn SaaS a partir de 2026.

1. Abra o curso e vá para a página **Conteúdo do Curso**.
2. Passe o mouse ou toque onde você quer que o fio de comentários apareça no sumário e clique no botão roxo **+** (Adicionar conteúdo).
3. Escolha **Mercado de Conteúdo**. O painel do Mercado de Conteúdo lista todas as ferramentas LTI aprovadas e os posicionamentos de Building Block para sua instituição.
4. Encontre o bloco **FastComments** e clique nele. O Blackboard cria um item de conteúdo na posição onde você abriu o menu **+**.
5. O item aparece no sumário como uma entrada "Visível para estudantes" por padrão para instrutores que tenham **Ocultar dos estudantes** desativado como padrão pessoal. Se seu padrão for **Oculto**, o item é criado como oculto e você alterna o seletor de visibilidade na linha do item quando estiver pronto.
6. Para renomear o item, clique no título no sumário e digite um novo rótulo. O título que os estudantes veem no sumário é independente do identificador do thread do FastComments, então renomear é seguro a qualquer momento.

Se você não vir **Mercado de Conteúdo** como uma opção, sua instituição manteve o posicionamento oculto. Você também acessa o mesmo seletor através de **Mais ferramentas** no mesmo menu **+** sob o grupo **LTI Tools**.

#### Original Course View

Original Course View ainda é suportado no Learn SaaS e permanece a experiência primária para sites Learn 9.1 auto-hospedados na linha de release CU de Q4 2024.

1. Abra o curso e entre em uma **Área de Conteúdo** (por exemplo, a área padrão **Informações** ou **Conteúdo** no menu do curso).
2. Ative o **Modo de Edição** com o interruptor no canto superior direito da página.
3. Clique em **Build Content** na barra de ações.
4. No submenu **Learning Tools**, clique em **FastComments**. O submenu Learning Tools é preenchido a partir dos posicionamentos de ferramenta LTI 1.3 depois que um administrador registra a ferramenta. Se você não o vir, veja a seção de problemas abaixo.
5. No formulário **Create FastComments**, defina:
   - **Name**: o rótulo que os estudantes veem na área de conteúdo.
   - **Description**: texto opcional mostrado acima do thread incorporado.
   - **Permit Users to View this Content**: alternador de disponibilidade Sim/Não.
   - **Track Number of Views**: habilite se você quiser as estatísticas de visualização por item do Blackboard. O FastComments executa suas próprias análises de forma independente.
   - **Date and Time Restrictions**: janelas opcionais **Display After** / **Display Until**.
6. Envie. A ferramenta aparece como um item clicável na área de conteúdo.

#### Incorporação Dentro de um Item ou Documento

Em ambas as visualizações de curso, os instrutores incorporam o FastComments inline dentro do corpo de um Item, Documento ou qualquer campo de texto rico através do botão LTI Advantage do Editor de Conteúdo.

Ultra Course View:

1. Crie ou edite um **Documento**.
2. Clique em **Adicionar conteúdo** dentro do corpo do documento onde você quer que o thread apareça.
3. Na barra de ferramentas do editor, abra o menu **Inserir conteúdo** e clique em **Mercado de Conteúdo** (o ponto de entrada LTI Advantage / Deep Linking).
4. Escolha **FastComments**. O FastComments retorna uma carga útil de deep-link e o Blackboard insere um bloco incorporado no corpo do documento na posição do cursor.
5. Salve o documento. Os estudantes vêem o thread renderizado inline conforme rolam a página.

Original Course View:

1. Edite qualquer item com um corpo de texto rico.
2. Na barra de ferramentas do Editor de Conteúdo, clique no ícone de mais **Adicionar Conteúdo** e escolha **Mercado de Conteúdo** (rotulado **Add Content from External Tool** em CUs mais antigas de Q4 2024).
3. Escolha **FastComments**. O editor insere um bloco de espaço reservado referenciando o recurso deep-linked.
4. Envie o item.

Cada incorporação via deep-link produz seu próprio thread FastComments, então um Item com dois blocos FastComments incorporados tem dois fluxos de comentários independentes.

#### Visibilidade, Condições de Liberação e Restrições de Grupo

Os itens de conteúdo do FastComments se comportam como qualquer outro item de conteúdo do Blackboard em relação às regras de controle de acesso aplicadas a eles.

- Ultra: clique no seletor de visibilidade na linha (**Visível para estudantes**, **Oculto dos estudantes**, **Disponibilidade condicional**). A disponibilidade condicional suporta janelas de data/hora, regras de desempenho contra itens do gradebook e regras de membros contra grupos do curso.
- Original: abra o menu de contexto do item e escolha **Liberação Adaptativa** ou **Liberação Adaptativa: Avançado** para restringir a ferramenta por data, associação, nota ou status de revisão. Use **Definir Disponibilidade por Grupo** no item para restringir a grupos específicos do curso.

O FastComments respeita o que quer que a restrição do Blackboard decida. Se o Blackboard ocultar o item de um estudante, o lançamento LTI nunca ocorre para esse estudante e ele não aparece na visualização de moderador.

#### Comportamento no Livro de Notas

O FastComments não envia notas de volta via LTI Advantage Assignment and Grade Services. Nenhuma coluna de nota é criada automaticamente para itens de conteúdo do FastComments.

Se seu tenant do Blackboard estiver configurado para criar automaticamente uma coluna do livro de notas para cada novo item de conteúdo independentemente dos metadados de avaliação, uma coluna vazia aparece mesmo assim. Para ocultá-la:

- Ultra: abra o **Gradebook**, clique no cabeçalho da coluna, escolha **Editar**, e desative **Mostrar para estudantes** e **Incluir nos cálculos**. Ou use **Excluir** se sua instituição permitir a exclusão de colunas para itens sem avaliação.
- Original: abra o **Grade Center**, clique na chevron da coluna, escolha **Ocultar de Usuários (on/off)**, e opcionalmente **Ocultar da Visualização do Instrutor** em **Organização de Colunas**.

#### O Que os Estudantes Veem

Quando um estudante abre o item do FastComments ou rola até um bloco incorporado:

1. O Blackboard envia a mensagem LTI 1.3 para o FastComments. O estudante é autenticado via SSO usando sua identidade do Blackboard (nome, email, avatar, função) sem ver um formulário de login.
2. O fio de comentários é renderizado no iframe. Encadeamento, respostas, menções e reações estão disponíveis conforme as configurações do widget de comentários configuradas no FastComments.
3. Seus comentários são atribuídos à conta do Blackboard. Se o estudante editar seu nome ou foto no Blackboard depois, o próximo lançamento atualiza o perfil do FastComments.

Mapeamento de funções do Blackboard para o FastComments:

- **Administrador do Sistema** e **Construtor de Curso** mapeiam para FastComments **admin**.
- **Instrutor** e **Assistente de Ensino** mapeiam para FastComments **moderator**.
- **Estudante**, **Convidado** e **Observador** mapeiam para FastComments **commenter**.

Moderadores veem controles de moderação (fixar, ocultar, banir, excluir) inline em cada comentário no thread.

#### Escopo do Thread

O FastComments define o escopo de cada thread por **(host do Blackboard, ID do curso, ID do resource link)**. Dois itens FastComments no mesmo curso produzem dois threads. O mesmo item copiado entre duas turmas (por exemplo, através de cópia de curso) produz dois threads, porque o Blackboard emite um novo resource link ID durante a cópia. Para manter um thread compartilhado através de cópias de curso, use Deep Linking com um URN de thread explícito configurado no FastComments antes de executar a cópia.

#### Particularidades Específicas do Blackboard

**Bloco FastComments ausente do menu Build Content (Original) ou do Mercado de Conteúdo (Ultra).** O administrador aprovou a ferramenta mas deixou uma política da instituição bloqueando o posicionamento relevante. Vá para **Painel do Administrador** > **Integrações** > **LTI Tool Providers**, edite a entrada do FastComments e confirme que os posicionamentos **Course Content Tool** (Original) e **Course Content Tool - allow students** / **Deep Linking content tool** (Ultra) estão habilitados. Salve e atualize a página do curso.

**Erro "Tool not configured for this context" ou "Tool is not deployed" no lançamento.** O escopo de deployment registrado durante o registro dinâmico não corresponde ao contexto da instituição ao qual o curso pertence. Na entrada do provedor de ferramenta do Blackboard, verifique se o **Deployment ID** corresponde ao que o FastComments mostra na sua página de Configuração LTI 1.3 para este tenant. Se diferirem, exclua o posicionamento e reexecute o registro dinâmico a partir de uma URL de registro nova.

**Altura do iframe parece fixa ou o conteúdo é cortado.** Alguns tenants do Blackboard vêm com uma Content Security Policy rigorosa que bloqueia o postMessage de redimensionamento de iframe padrão do LTI. O FastComments emite tanto a mensagem no estilo Canvas `lti.frameResize` quanto a mensagem no formato da especificação IMS `org.imsglobal.lti.frameResize` para maximizar a compatibilidade, mas uma substituição de CSP em nível de tenant bloqueia o listener no parent. Peça ao seu administrador para confirmar que `*.fastcomments.com` está na allowlist de ferramentas LTI e que nenhum cabeçalho CSP personalizado está removendo eventos postMessage. O redimensionamento então funciona sem configuração adicional.

**Cópia de curso duplica threads.** A cópia de curso do Blackboard emite novos resource link IDs para posicionamentos LTI, então cursos copiados começam com threads vazios. Isso é esperado. Se você precisar que o curso copiado herde o thread original, configure Deep Linking com um URN de thread explícito antes de copiar, ou contate o suporte do FastComments para remapear IDs de thread em massa.

**O estudante vê um erro genérico do Blackboard no lançamento.** A causa é uma claim `email` ausente ou obsoleta. Confirme a política da instituição para o FastComments e que **Role**, **Name**, e **Email Address** estão habilitados em **User Fields to Send**. Salve, então inicie novamente em uma sessão de navegador nova.
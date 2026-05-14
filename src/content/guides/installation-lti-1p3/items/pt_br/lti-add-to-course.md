Uma vez que o FastComments esteja registrado no seu LMS, os instrutores o adicionam aos cursos da mesma forma que adicionam qualquer outra ferramenta externa LTI.

#### D2L Brightspace

Na área de conteúdo de um curso:

1. Clique em **Adicionar Atividades Existentes** > **Ferramentas de Aprendizagem Externas**.
2. Selecione **FastComments** na lista.
3. A ferramenta aparece como um tópico na área de conteúdo. Abra-a uma vez para inicializar o tópico de comentários para esse recurso.

#### Moodle

Em um curso:

1. Ative o **modo de edição**.
2. Na seção onde você quer comentários, clique em **Adicionar uma atividade ou recurso**.
3. Escolha **FastComments** no seletor de atividades.
4. Salve. Os estudantes veem o tópico de comentários embutido na seção.

#### Blackboard Learn

Em um curso:

1. Navegue até uma Área de Conteúdo.
2. Clique em **Criar Conteúdo** > **FastComments** (em "Ferramentas de Aprendizagem").
3. Configure um nome e envie.

#### Sakai

Os mantenedores do site adicionam a ferramenta por meio de **Informações do Site** > **Gerenciar Ferramentas** > role até **Ferramentas Externas** > selecione **FastComments** > **Continuar**.

#### Como os tópicos são definidos

O FastComments cria um tópico de comentários separado por **(instância do LMS, curso, link de recurso)**. Isso significa:

- Dois cursos diferentes no mesmo LMS recebem tópicos separados, mesmo que usem o mesmo nome de ferramenta.
- A mesma ferramenta FastComments usada em dois locais dentro de um curso cria dois tópicos.
- Duas instalações diferentes do Brightspace sob a mesma conta FastComments obtêm tópicos distintos - o nome do host do LMS faz parte do identificador do tópico.

#### SSO

Os estudantes não veem uma tela de login. O LMS envia sua identidade (nome, e-mail, avatar, função) para o FastComments via o lançamento LTI, e o FastComments os autentica automaticamente. Seus comentários são atribuídos à conta do LMS.

Usuários com as funções do LMS **Instrutor** ou **Administrador** são mapeados automaticamente para os papéis de moderador/administrador do FastComments no tópico.
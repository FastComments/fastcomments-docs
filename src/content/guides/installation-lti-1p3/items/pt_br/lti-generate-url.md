#### Navegue até a Configuração LTI 1.3

Faça login no FastComments e vá para <a href="https://fastcomments.com/auth/my-account/lti-config" target="_blank">sua página de Configuração LTI 1.3</a>.

Se sua conta ainda não tiver acesso ao LTI, você verá "LTI not enabled for this account" - entre em contato com o suporte para habilitá-lo no seu plano.

#### Escolha uma Plataforma (Opcional)

Sob **Gerar uma URL de Registro Dinâmico**, use o dropdown **Plataforma** para informar ao FastComments a qual LMS você está se conectando:

- D2L Brightspace
- Moodle
- Blackboard Learn
- Sakai
- Schoology
- Outra plataforma LTI 1.3

Você também pode deixá-lo em **Auto-detect**. A plataforma é lida a partir do openid-configuration do seu LMS durante o registro; o menu suspenso apenas preenche o rótulo de exibição para a configuração resultante.

#### Gerar a URL

Clique em **Generate URL**. O FastComments cria um token de registro de uso único e mostra uma URL que se parece com:

`https://fastcomments.com/lti/v1p3/register/<long-token>`

Copie-a. Esta URL:

- É de **uso único** - uma vez que seu LMS a chamar com sucesso, o token é consumido.
- Expira após **30 minutos** se não for usada.
- Deve ser mantida privada - qualquer pessoa com a URL pode registrar uma ferramenta no seu tenant dentro desses 30 minutos.

#### Configurações Existentes

Uma vez que um registro seja concluído com sucesso, a nova configuração aparece na tabela **Configurações Existentes** na mesma página, com sua Plataforma, Emissor, ID do Cliente e Status. Você pode excluir configurações desta tabela se precisar cancelar o registro.
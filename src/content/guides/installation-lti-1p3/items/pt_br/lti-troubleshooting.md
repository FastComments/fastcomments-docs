#### "Token de registro não encontrado, expirado ou já utilizado"

O token na sua URL de registro (<a href="https://fastcomments.com/auth/my-account/lti-config" target="_blank">obtenha aqui</a>) é válido por 30 minutos e só pode ser usado uma vez. Se o seu LMS demorou mais do que isso, ou se o registro foi repetido após ter sido concluído com sucesso, o token será rejeitado. Gere uma nova URL na página de Configuração do LTI 1.3 do FastComments e comece novamente.

#### "Plataforma rejeitou o registro"

Seu LMS recusou o handshake de registro. As causas mais comuns:

- **Ferramenta já registrada com o mesmo nome de cliente.** Algumas plataformas (notavelmente o D2L) rejeitam um segundo registro de "FastComments" até que o anterior seja excluído. Remova a ferramenta antiga no seu LMS e tente novamente.
- **Campo errado no LMS.** Certifique-se de ter colado a URL no campo **registration / tool initiation registration endpoint**, e não no campo de URL de launch ou de login.
- **O LMS na verdade não suporta Registro Dinâmico.** Versões mais antigas do Moodle e do Blackboard anunciam LTI 1.3, mas só permitem configuração manual. Consulte a documentação da sua plataforma.

#### "Falha ao buscar a configuração da plataforma"

O FastComments não conseguiu ler o documento openid-configuration do seu LMS. Isso é raro e geralmente significa que o LMS forneceu uma URL de descoberta malformada ou inacessível. Entre em contato com o suporte do seu LMS.

#### O launch mostra "Configuration not found"

Ou a configuração no FastComments foi excluída, ou o launch veio de um par `iss`/`client_id` que não reconhecemos. Se você excluiu e registrou novamente, instrua seu LMS a remover e readicionar a ferramenta FastComments para que ela receba o novo client_id.

#### O launch mostra "Deployment not registered"

Você iniciou o FastComments a partir de um deployment do Brightspace/Moodle/Blackboard diferente daquele em que foi iniciado pela primeira vez. O FastComments fixa o `deployment_id` no primeiro launch como uma verificação de segurança. Para adicionar um novo deployment sob o mesmo cliente, entre em contato com o suporte - adicionaremos o ID do deployment à configuração.

#### O launch mostra "Unsupported message_type"

O LMS enviou uma mensagem LTI que o FastComments não trata (ex.: `LtiSubmissionReviewRequest`). O FastComments suporta apenas o launch padrão de resource-link e os fluxos de deep-linking. Fale conosco se precisar que um tipo de mensagem específico seja adicionado.

#### O iframe não é redimensionado

A maioria dos LMSs dimensiona automaticamente os iframes LTI. Se o seu não o faz, verifique se as configurações de launch do LMS permitem que a ferramenta envie eventos postMessage para o frame pai. O FastComments emite mensagens de redimensionamento tanto no estilo Canvas (`lti.frameResize`) quanto na especificação IMS (`org.imsglobal.lti.frameResize).
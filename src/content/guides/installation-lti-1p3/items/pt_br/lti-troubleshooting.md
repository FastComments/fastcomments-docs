#### "Registration token not found, expired, or already used"

O token na sua URL de registro (<a href="https://fastcomments.com/auth/my-account/lti-config" target="_blank">obtenha-o aqui</a>) é válido por 30 minutos e só pode ser usado uma vez. Se seu LMS levou mais tempo do que isso, ou se o registro foi reenviado após ter sido bem-sucedido, o token será rejeitado. Gere uma URL nova na página FastComments LTI 1.3 Configuration e recomece.

#### "Platform rejected registration"

Seu LMS recusou o handshake de registro. As causas mais comuns:

- **Tool already registered with the same client name.** Algumas plataformas (notavelmente D2L) rejeitam um segundo registro de "FastComments" até que o anterior seja excluído. Remova a ferramenta antiga no seu LMS e tente novamente.
- **Wrong field in the LMS.** Certifique-se de que você colou a URL no campo **registration / tool initiation registration endpoint**, e não no campo launch URL ou login URL.
- **The LMS doesn't actually support Dynamic Registration.** Versões mais antigas do Moodle e do Blackboard anunciam LTI 1.3 mas só permitem configuração manual. Consulte a documentação da sua plataforma.

#### "Failed to fetch platform configuration"

O FastComments não conseguiu ler o documento openid-configuration do seu LMS. Isso é raro e geralmente significa que o LMS forneceu uma URL de discovery malformada ou inacessível. Contate o suporte do seu LMS.

#### Launch shows "Configuration not found"

Ou a configuração no FastComments foi excluída, ou o lançamento veio de um par `iss`/`client_id` que não reconhecemos. Se você excluiu e se registrou novamente, instrua seu LMS a remover e readicionar a ferramenta FastComments para que ela receba o novo `client_id`.

#### Launch shows "Deployment not registered"

Você lançou o FastComments a partir de um deployment do Brightspace/Moodle/Blackboard diferente daquele em que ele foi lançado pela primeira vez. O FastComments fixa o `deployment_id` no primeiro lançamento como uma verificação de segurança. Para adicionar um novo deployment sob o mesmo cliente, contate o suporte — nós adicionaremos o deployment ID à configuração.

#### Launch shows "Unsupported message_type"

O LMS enviou uma mensagem LTI que o FastComments não trata (por exemplo, `LtiSubmissionReviewRequest`). O FastComments suporta apenas os fluxos padrão de lançamento resource-link e de deep-linking. Entre em contato se precisar que um tipo de mensagem específico seja adicionado.

#### Iframe doesn't resize

A maioria dos LMSs redimensiona automaticamente iframes LTI. Se o seu não faz isso, verifique se as configurações de lançamento do LMS permitem que a ferramenta envie eventos postMessage para o frame pai. O FastComments emite mensagens de redimensionamento no estilo Canvas (`lti.frameResize`) e conforme a especificação IMS (`org.imsglobal.lti.frameResize`).
#### "Token de registro não encontrado, expirado ou já usado"

O token na sua URL de registro é válido por 30 minutos e só pode ser usado uma vez. Se seu LMS demorou mais que isso, ou se o registro foi tentado novamente depois de ter sido concluído com sucesso, o token será rejeitado. Gere uma URL nova na página de Configuração do FastComments LTI 1.3 e recomece.

#### "Plataforma rejeitou o registro"

Seu LMS recusou o handshake de registro. As causas mais comuns:

- **Ferramenta já registrada com o mesmo nome de cliente.** Algumas plataformas (notadamente D2L) rejeitam um segundo registro de "FastComments" até que o anterior seja excluído. Remova a ferramenta antiga no seu LMS e tente novamente.
- **Campo errado no LMS.** Certifique-se de que você colou a URL no campo **registration / tool initiation registration endpoint**, e não no campo launch URL ou login URL.
- **O LMS na verdade não suporta Dynamic Registration.** Versões mais antigas do Moodle e do Blackboard anunciam LTI 1.3, mas só permitem configuração manual. Verifique a documentação da sua plataforma.

#### "Falha ao buscar a configuração da plataforma"

O FastComments não conseguiu ler o documento openid-configuration do seu LMS. Isso é raro e geralmente significa que o LMS forneceu uma URL de discovery malformada ou inacessível. Entre em contato com o suporte do seu LMS.

#### O lançamento mostra "Configuration not found"

Ou a configuração no FastComments foi excluída, ou o lançamento veio de um par `iss`/`client_id` que não reconhecemos. Se você excluiu e registrou novamente, instrua seu LMS a remover e readicionar a ferramenta FastComments para que ela receba o novo client_id.

#### O lançamento mostra "Deployment not registered"

Você iniciou o FastComments a partir de um deployment do Brightspace/Moodle/Blackboard diferente daquele em que ele foi lançado pela primeira vez. O FastComments fixa o `deployment_id` no primeiro lançamento como verificação de segurança. Para adicionar um novo deployment sob o mesmo cliente, entre em contato com o suporte — adicionaremos o deployment ID à configuração.

#### O lançamento mostra "Unsupported message_type"

O LMS enviou uma mensagem LTI que o FastComments não processa (por exemplo, `LtiSubmissionReviewRequest`). O FastComments suporta apenas o lançamento padrão de resource-link e os fluxos de deep-linking. Entre em contato se você precisar que um tipo de mensagem específico seja adicionado.

#### O iframe não redimensiona

A maioria dos LMSs redimensiona automaticamente iframes LTI. Se o seu não faz isso, verifique se as configurações de lançamento do LMS permitem que a ferramenta envie eventos postMessage para o frame pai. O FastComments emite mensagens de redimensionamento no estilo Canvas (`lti.frameResize`) e conforme a especificação IMS (`org.imsglobal.lti.frameResize`).
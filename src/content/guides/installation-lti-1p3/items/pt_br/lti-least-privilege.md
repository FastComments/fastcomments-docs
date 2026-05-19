A integração FastComments LTI 1.3 segue o princípio do menor privilégio: ela usa apenas as claims do launch necessárias para identificar o usuário, associar comentários ao curso e recurso corretos e aplicar permissões baseadas em função.

O resto desta página mapeia cada claim que a integração consome, cada serviço do LTI Advantage que ela não solicita e cada categoria de dados que ela não coleta. Revisores de segurança e compras podem extrair respostas diretamente das tabelas abaixo.

## Elementos de Dados Recebidos do LMS

Todo launch LTI 1.3 carrega um JWT assinado pelo LMS. O FastComments extrai as seguintes claims desse JWT e não usa mais nada:

| Campo | Reivindicação LTI | Propósito | Obrigatório | Armazenado |
|-------|-----------|---------|----------|--------|
| Identificador do usuário | `sub` | Identifica o usuário de forma consistente entre launches para que a mesma pessoa seja resolvida para o mesmo usuário SSO interno do FastComments | Sim | Sim, como parte de um ID SSO interno estável |
| Nome de exibição | `name` | Atribuição exibida ao lado dos comentários do usuário | Sim (cai para "Usuário do LMS" se ausente) | Sim |
| E-mail | `email` | Correspondência de conta, notificações, moderação, comunicação de suporte | Opcional (a integração funciona sem ele) | Sim quando fornecido |
| URL do avatar | `picture` | Exibido nos comentários do usuário | Opcional | Apenas URL; o FastComments não baixa nem re-hospeda a imagem |
| Funções | `https://purl.imsglobal.org/spec/lti/claim/roles` | Determina se o usuário é administrador, instrutor (moderador) ou aprendiz | Sim | Flags derivadas `isAdmin` / `isModerator` na sessão SSO |
| Contexto do curso | `https://purl.imsglobal.org/spec/lti/claim/context` (`id`, `title`) | Associa o thread de comentários ao curso correto do LMS | Sim | Sim, como parte do identificador de página resolvido |
| Link do recurso | `https://purl.imsglobal.org/spec/lti/claim/resource_link` (`id`) | Associa comentários à atividade ou à posição correta da ferramenta dentro do curso | Sim quando presente | Sim, como parte do identificador de página resolvido |
| Deployment ID | `https://purl.imsglobal.org/spec/lti/claim/deployment_id` | Roteia o launch para a configuração de tenant correta do FastComments | Sim | Sim, no registro de configuração LTI do FastComments |

## Reivindicações e Escopos Declarados no Registro

Durante o Registro Dinâmico LTI 1.3, o FastComments se registra com `scope: ""` (nenhum escopo OAuth adicional) e declara apenas estas claims do OpenID Connect:

`iss`, `sub`, `name`, `email`, `picture`

Registra dois tipos de mensagem:

- `LtiResourceLinkRequest` - o lançamento padrão de curso no FastComments.
- `LtiDeepLinkingRequest` - permite que instrutores posicionem a ferramenta FastComments dentro de um curso.

Nenhum token de acesso adicional é solicitado ao LMS.

## Serviços do LTI Advantage Não Solicitados

| Serviço / escopo | Solicitado? | Motivo |
|------------------|------------|--------|
| Serviços de Nomes e Provisionamento de Funções (NRPS) | Não | A integração não precisa de uma lista de participantes do curso; a identidade do usuário chega com cada lançamento |
| Assignment and Grade Services (AGS) - lineitem, score, result scopes | Não | A integração não interage com o sistema de notas |
| Deep Linking beyond the standard placement return | Nenhum dado adicional | O Deep Linking é usado apenas para o posicionamento da ferramenta por instrutores; nenhum conteúdo do curso é enumerado |

## Dados Não Coletados

Além do próprio LTI, o FastComments não solicita nem recebe o seguinte do LMS ou do usuário:

| Categoria | Coletado? |
|----------|------------|
| Notas dos estudantes | Não |
| Submissões de atividades | Não |
| Registros de presença | Não |
| Listas completas de participantes do curso | Não |
| Identificadores governamentais | Não |
| Data de nascimento | Não |
| Endereço postal ou número de telefone | Não |
| Informações financeiras | Não |
| Credenciais de administrador do LMS | Não |

## Limites de Acesso

- O FastComments recebe dados apenas dentro de um launch LTI 1.3 autorizado assinado pelas chaves registradas do LMS. A integração não faz chamadas de retorno ao LMS para informações adicionais.
- Os tokens de launch são de uso único e de curta duração. Tokens reaproveitados ou expirados são rejeitados.
- Os administradores do LMS controlam onde a ferramenta é implantada dentro de sua plataforma. O D2L Brightspace, por exemplo, suporta escopo de org-unit por implantação e configurações de segurança por implantação, o que permite aos administradores restringir a ferramenta a cursos ou org units específicos em vez de torná-la disponível globalmente. Moodle, Blackboard, Sakai e Schoology oferecem controles por implantação equivalentes em suas implementações LTI 1.3.

## Armazenamento e Retenção

O FastComments retém dados derivados do LTI durante a duração do serviço de comentários ativo e de acordo com as configurações de retenção configuradas pelo cliente. Os dados de comentários são armazenados em armazenamento de produção criptografado em repouso. Em encerramento de conta ou mediante solicitação de exclusão por escrito, o FastComments exclui ou anonimiza os dados do cliente conforme o contrato aplicável.

Para detalhes completos sobre armazenamento e tratamento de dados, veja a <a href="https://fastcomments.com/privacy-policy" target="_blank">Política de Privacidade do FastComments</a>.

## Cadência de Revisão

Qualquer novo recurso LTI que exija claims adicionais, escopos ou serviços do LTI Advantage é revisado antes do lançamento para confirmar que o acesso solicitado é necessário e proporcional ao recurso que será fornecido.

## Declaração curta para questionários de segurança

> O FastComments aplica o princípio do menor privilégio e a minimização de dados à sua integração LTI 1.3. A integração usa apenas as claims do launch LTI necessárias para autenticar o usuário (`sub`, `name`, `email`, `picture`), determinar sua função e identificar o curso e o recurso aos quais os comentários pertencem. O FastComments não solicita Serviços de Nomes e Provisionamento de Funções, Serviços de Atribuição e Notas, dados do livro de notas, presença, listas completas de participantes ou acesso administrativo ao LMS. Os administradores do LMS mantêm o controle sobre em quais org units, cursos e implantações a ferramenta está disponível.
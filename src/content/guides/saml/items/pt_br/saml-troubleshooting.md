Este guia cobre problemas comuns de autenticação SAML e suas soluções.

### Problemas de Certificado e Segurança

#### Erro de Certificado Inválido

**Sintomas**:
- Erro "Certificate validation failed"
- Usuários não conseguem completar a autenticação SAML
- Respostas SAML são rejeitadas

**Causas Comuns**:
- Formato do certificado está incorreto
- Certificado expirou
- Foi fornecido o certificado errado
- Caracteres extras ou espaços em branco no certificado

**Soluções**:
1. **Verificar Formato do Certificado**:
   - Certifique-se de que o certificado inclua os marcadores `-----BEGIN CERTIFICATE-----` e `-----END CERTIFICATE-----`
   - Remova qualquer espaço em branco ou quebras de linha extras
   - Copie o certificado diretamente dos metadados ou da configuração do IdP

2. **Checar Validade do Certificado**:
   - Verifique se o certificado não expirou
   - Confirme se o certificado é para o IdP correto
   - Use validadores de certificado online para checar o formato

3. **Rebaixar o Certificado**:
   - Faça download de um certificado novo do IdP
   - Use a URL de metadados do IdP, se disponível
   - Confirme se o certificado corresponde à configuração atual do IdP

#### Falha na Verificação da Assinatura

**Sintomas**:
- Erros de validação da assinatura da asserção SAML
- Autenticação falha após login no IdP
- Mensagens de erro "Invalid signature"

**Soluções**:
1. **Incompatibilidade de Algoritmo**:
   - Verifique se o algoritmo de assinatura em FastComments corresponde ao do IdP
   - Tente diferentes algoritmos de assinatura (SHA-256, SHA-1, SHA-512)
   - Verifique se o algoritmo de digest corresponde à configuração do IdP

2. **Problemas de Certificado**:
   - Garanta que o certificado de assinatura correto esteja configurado
   - Verifique se o certificado corresponde à chave privada usada pelo IdP
   - Cheque por rotação de certificado no IdP

### Problemas de Configuração

#### Entity ID ou ACS URL Incorretos

**Sintomas**:
- IdP relata "Unknown Service Provider"
- Respostas SAML vão para o endpoint errado
- Autenticação não é concluída

**Soluções**:
1. **Verificar Informações do SP**:
   - Copie exatamente o Entity ID da configuração do FastComments
   - Garanta que o ACS URL corresponda ao formato: `https://fastcomments.com/saml/callback/{tenant-id}`
   - Verifique por erros de digitação no tenant ID

2. **Configuração do IdP**:
   - Atualize o IdP com o Entity ID correto do SP
   - Configure o ACS/Reply URL correto
   - Verifique as configurações de binding do IdP (HTTP-POST preferido)

#### Atributos Ausentes ou Incorretos

**Sintomas**:
- Usuários criados sem funções adequadas
- Informações de perfil do usuário ausentes
- Erros "Email required"

**Soluções**:
1. **Atributo de Email**:
   - Certifique-se de que o IdP envia o atributo de email
   - Verifique o mapeamento do nome do atributo (email, emailAddress, etc.)
   - Confirme que o valor do email é um endereço de email válido

2. **Atributos de Função**:
   - Confirme que o IdP envia informações de função/grupo
   - Verifique se os nomes dos atributos de função correspondem às expectativas do FastComments
   - Verifique se os valores das funções correspondem exatamente aos nomes de função do FastComments

3. **Formato do Atributo**:
   - Teste tanto o formato em array quanto o formato separado por vírgulas para funções
   - Garanta que os valores dos atributos não tenham espaços em branco extras
   - Verifique sensibilidade a maiúsculas/minúsculas nos nomes das funções

### Problemas no Fluxo de Autenticação

#### Loop de Redirecionamento

**Sintomas**:
- O navegador redireciona sem parar entre FastComments e IdP
- Autenticação nunca é concluída
- Múltiplos redirecionamentos mostrados nas ferramentas de desenvolvedor do navegador

**Soluções**:
1. **Verificar Configuração do SP**:
   - Verifique se o Entity ID bate exatamente com a configuração do IdP
   - Garanta que o ACS URL esteja corretamente configurado no IdP
   - Cheque por barras finais (trailing slashes) nas URLs

2. **Problemas de Sessão**:
   - Limpe os cookies do navegador e tente novamente
   - Teste em uma janela anônima/privada do navegador
   - Verifique as configurações de timeout de sessão

#### Acesso Negado Após Autenticação

**Sintomas**:
- Autenticação SAML é bem-sucedida
- Usuário é redirecionado para o FastComments
- Exibe "Access denied" ou erro de permissões

**Soluções**:
1. **Atribuição de Função**:
   - Verifique se o usuário tem as funções apropriadas no IdP
   - Cheque se o atributo de função está sendo enviado na resposta SAML
   - Confirme se os nomes das funções correspondem exatamente aos requisitos do FastComments

2. **Limitações do Pacote**:
   - Verifique se a conta possui o plano Flex ou Pro
   - Cheque se o recurso SAML está habilitado para o pacote
   - Contate o suporte se o pacote inclui SAML mas o recurso não estiver disponível

### Problemas Específicos do Provedor de Identidade

#### Microsoft Azure AD

**Problemas Comuns**:
- Atribuições de função do aplicativo não refletidas nos tokens
- Claims não sendo enviadas corretamente
- Requisitos de atribuição de usuário

**Soluções**:
- Verifique a atribuição de usuários ao aplicativo FastComments
- Confirme se as funções do aplicativo estão configuradas corretamente
- Garanta que o mapeamento de claims inclua os atributos necessários

#### Okta

**Problemas Comuns**:
- Filtros de grupo não funcionando corretamente
- Declarações de atributo configuradas incorretamente
- Problemas de atribuição de aplicativo

**Soluções**:
- Revise a configuração da declaração de atributo
- Verifique as regras de atribuição e filtragem de grupo
- Confirme se o aplicativo está atribuído aos usuários/grupos apropriados

#### Google Workspace

**Problemas Comuns**:
- Atributos personalizados não mapeando corretamente
- Membros de grupo não sendo enviados
- Erros de configuração da aplicação SAML

**Soluções**:
- Configure o esquema personalizado para atributos de função
- Verifique a propagação da associação de grupos
- Confirme o mapeamento de atributos da aplicação SAML

### Problemas de Rede e Conectividade

#### Erros de Timeout

**Sintomas**:
- Processo de autenticação expira por timeout
- Erros "Request timeout" ou similares
- Fluxo de autenticação lento

**Soluções**:
1. **Conectividade de Rede**:
   - Verifique se regras de firewall permitem comunicação com FastComments
   - Verifique a resolução de DNS para fastcomments.com
   - Teste a conectividade de rede do IdP para o FastComments

2. **Problemas de Desempenho**:
   - Verifique os tempos de resposta do IdP
   - Confirme se a validação da cadeia de certificados não está lenta
   - Considere a latência de rede entre o IdP e os usuários

#### Problemas de SSL/TLS

**Sintomas**:
- Avisos de certificado durante a autenticação
- Falhas no handshake SSL
- Erros "Secure connection failed"

**Soluções**:
- Garanta que todos os endpoints SAML usem HTTPS
- Verifique a validade dos certificados para todos os domínios envolvidos
- Verifique a compatibilidade de versão TLS

### Depuração e Logs

#### Habilitar Informações de Depuração

1. **Ferramentas de Desenvolvedor do Navegador**:
   - Monitore a aba Network durante o fluxo SAML
   - Verifique o Console para erros de JavaScript
   - Examine as requisições POST SAML (se visíveis)

2. **Logs do IdP**:
   - Habilite a depuração SAML no seu IdP
   - Revise os logs do IdP para detalhes das requisições/respostas SAML
   - Verifique problemas de mapeamento de atributos

#### Mensagens de Log Comuns

**Logs do FastComments**:
- "SAML config not found" - SAML não habilitado ou configurado incorretamente
- "Invalid certificate" - Falha na validação do certificado
- "Missing email attribute" - Email obrigatório não fornecido na resposta SAML

**Logs do IdP**:
- "Unknown service provider" - Entity ID não corresponde
- "Invalid ACS URL" - Assertion Consumer Service URL incorreto
- "User not assigned" - Usuário não tem acesso à aplicação SAML

### Obter Ajuda

#### Informações a Coletar

Ao contatar o suporte, forneça:
- Mensagens de erro exatas e timestamps
- Detalhes da configuração SAML (sem dados sensíveis)
- Tipo e versão do IdP
- Passos para reproduzir o problema
- Informações do navegador e da rede

#### Suporte FastComments

Para problemas relacionados a SAML:
1. Use o [support portal](https://fastcomments.com/auth/my-account/help)
2. Inclua o tenant ID e os emails dos usuários afetados
3. Forneça mensagens de erro e detalhes da configuração
4. Especifique o tipo de IdP e a abordagem de configuração

#### Suporte do IdP

Para problemas específicos do IdP:
- Consulte a documentação do IdP para troubleshooting SAML
- Use os canais de suporte do IdP para problemas de configuração
- Aproveite os fóruns da comunidade do IdP para problemas comuns

### Dicas de Prevenção

#### Boas Práticas

1. **Testar Exaustivamente**:
   - Teste mudanças de configuração em ambiente não produtivo
   - Verifique com múltiplos usuários de teste
   - Documente configurações que funcionam

2. **Monitorar Regularmente**:
   - Configure monitoramento para falhas de autenticação SAML
   - Revise datas de expiração de certificado
   - Monitore mudanças de configuração no IdP

3. **Documentação**:
   - Mantenha documentação da configuração SAML
   - Documente quaisquer configurações personalizadas ou soluções alternativas
   - Mantenha informações de contato dos administradores do IdP

#### Manutenção Proativa

1. **Gerenciamento de Certificados**:
   - Monitore datas de expiração de certificados
   - Planeje procedimentos de rotação de certificados
   - Teste atualizações de certificado antes da expiração

2. **Revisões de Configuração**:
   - Revise regularmente a configuração SAML
   - Verifique se a configuração do IdP permanece atual
   - Atualize a documentação conforme alterações forem feitas
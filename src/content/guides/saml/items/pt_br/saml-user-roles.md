FastComments mapeia funções de usuário SAML para permissões internas, permitindo controle de acesso baseado em funções para sua organização.

### Sistema de Funções do FastComments

O FastComments usa um sistema de permissões baseado em funções onde os usuários podem ter uma ou mais funções que determinam seus níveis de acesso e capacidades.

### Funções Disponíveis no FastComments

#### Funções Administrativas

**fc-account-owner**
- **Permissões**: Acesso administrativo completo
- **Capacidades**: Todos os recursos, gerenciamento de faturamento, gerenciamento de usuários
- **Caso de uso**: Administradores primários da conta e proprietários

**fc-admin-admin**  
- **Permissões**: Acesso administrativo à maioria dos recursos
- **Capacidades**: Gerenciamento de usuários, configuração, moderação. **Pode administrar outros administradores.**
- **Caso de uso**: Administradores secundários e equipe de TI

**fc-billing-admin**
- **Permissões**: Gerenciamento de faturamento e assinaturas
- **Capacidades**: Métodos de pagamento, faturas, alterações de assinatura
- **Caso de uso**: Membros da equipe financeira e contatos de faturamento

#### Funções Especializadas

**fc-analytics-admin**
- **Permissões**: Acesso a análises e relatórios
- **Capacidades**: Visualizar estatísticas do site, dados de engajamento de usuários
- **Caso de uso**: Equipes de marketing e analistas de dados

**fc-api-admin**
- **Permissões**: Acesso e gerenciamento da API
- **Capacidades**: Credenciais de API, configuração de webhooks
- **Caso de uso**: Desenvolvedores e integradores técnicos

**fc-moderator**
- **Permissões**: Capacidades de moderação de comentários
- **Capacidades**: Aprovar/rejeitar comentários, gerenciar spam
- **Caso de uso**: Moderadores da comunidade e gestores de conteúdo

### Configuração de Mapeamento de Funções

#### Fontes de Atributos SAML

O FastComments aceita informações de função de vários nomes de atributos SAML para garantir compatibilidade com diferentes provedores de identidade:

**Nomes de Atributos Padrão**:
- `roles`
- `groups` 
- `memberOf`
- `role`
- `group`

**Atributos Microsoft/ADFS**:
- `http://schemas.microsoft.com/ws/2008/06/identity/claims/role`
- `http://schemas.xmlsoap.org/ws/2005/05/identity/claims/role`

#### Suporte a Formatos de Função

**Formato Array** *(Preferido)*:
```xml
<saml:Attribute Name="roles">
    <saml:AttributeValue>fc-admin-admin</saml:AttributeValue>
    <saml:AttributeValue>fc-moderator</saml:AttributeValue>
</saml:Attribute>
```

**Formato Separado por Vírgulas**:
```xml
<saml:Attribute Name="roles">
    <saml:AttributeValue>fc-admin-admin,fc-moderator</saml:AttributeValue>
</saml:Attribute>
```

**Formato de Função Única**:
```xml
<saml:Attribute Name="roles">
    <saml:AttributeValue>fc-admin-admin</saml:AttributeValue>
</saml:Attribute>
```

### Configuração de Funções no Provedor de Identidade

#### Microsoft Azure AD

1. **Configuração de App Roles**:
   - Defina as funções do FastComments em sua aplicação do Azure AD
   - Atribua usuários às funções de aplicativo apropriadas
   - Configure as claims para incluir as funções atribuídas

2. **Mapeamento de Atributos**:
   ```
   Attribute Name: roles
   Source Attribute: user.assignedroles
   ```

#### Okta

1. **Atribuição de Grupos**:
   - Crie grupos que correspondam aos nomes de funções do FastComments
   - Atribua usuários aos grupos apropriados
   - Configure statements de atributos

2. **Statement de Atributo**:
   ```
   Name: roles
   Value: user.groups
   Filter: Starts with "fc-"
   ```

#### Google Workspace

1. **Mapeamento de Grupos**:
   - Crie unidades organizacionais ou grupos
   - Nomeie os grupos com prefixos de funções do FastComments
   - Configure o mapeamento de atributos

2. **Atributos Personalizados**:
   ```
   Attribute Name: roles
   Value: Groups or custom schema attribute
   ```

### Comportamento Padrão do Usuário

#### Usuários Sem Funções

Quando um usuário SAML não possui funções ou possui funções não reconhecidas:
- O usuário é criado como um comentarista padrão
- Nenhum acesso administrativo é concedido
- Pode publicar e gerenciar seus próprios comentários
- Não pode acessar recursos do painel administrativo

#### Herança de Funções

- Os usuários podem ter múltiplas funções simultaneamente
- As permissões são cumulativas (aplica-se o nível de permissão mais alto)
- Mudanças de função no IdP são refletidas no próximo login

### Gerenciando Usuários SAML

#### Criação de Usuário

Quando um usuário faz login via SAML pela primeira vez:
1. **Conta de Usuário**: Criada automaticamente com o email como identificador
2. **Atribuição de Função**: Funções aplicadas com base nos atributos SAML
3. **Informações de Perfil**: Nome e sobrenome preenchidos se fornecidos
4. **Ativação de Permissões**: As funções tornam-se ativas imediatamente

#### Atualizações de Função

Usuários SAML existentes recebem atualizações de função:
1. **Gatilho de Login**: Atualizações de função ocorrem durante cada login SAML
2. **Efeito Imediato**: Novas permissões aplicam-se imediatamente
3. **Remoção de Função**: Funções removidas são revogadas automaticamente
4. **Registro de Auditoria**: Mudanças de função são registradas nos logs de auditoria

### Mapeamento de Funções Personalizado

#### Personalização Empresarial

Para clientes empresariais com requisitos específicos:
- Nomes de função personalizados podem ser mapeados para permissões do FastComments
- Hierarquias complexas de funções podem ser implementadas
- Controles de acesso específicos por departamento podem ser configurados

Entre em contato com o suporte do FastComments para configurações de mapeamento de função personalizadas.

#### Validação de Funções

O FastComments valida as funções recebidas:
- Funções não reconhecidas são ignoradas (não rejeitadas)
- Atributos de função malformados são registrados para solução de problemas
- Os usuários mantêm funções existentes se a asserção SAML não contiver informações de função

### Melhores Práticas

#### Gerenciamento de Funções

1. **Princípio do Menor Privilégio**: Atribua as permissões mínimas necessárias
2. **Auditoria Regular**: Revise periodicamente as funções e acessos dos usuários  
3. **Nomenclatura Clara**: Use nomes de grupo descritivos em seu IdP
4. **Documentação**: Mantenha documentação das atribuições de funções

#### Considerações de Segurança

1. **Atributos de Função**: Garanta que os atributos de função estejam devidamente protegidos nas respostas SAML
2. **Validação de Atributos**: Verifique que apenas sistemas autorizados possam atribuir funções
3. **Revisões de Acesso**: Revise regularmente as atribuições de funções administrativas
4. **Monitoramento**: Monitore mudanças de função e ações administrativas

### Solução de Problemas de Funções

#### Problemas Comuns

**Funções Não Aplicadas**:
- Verifique se os nomes de atributos SAML correspondem aos formatos suportados
- Verifique se o IdP está enviando informações de função
- Confirme se os valores de função correspondem exatamente aos nomes de função do FastComments

**Acesso Negado**:
- Verifique se o usuário tem a função apropriada atribuída no IdP
- Verifique a ortografia e a sensibilidade a maiúsculas/minúsculas da função
- Confirme se a função está corretamente formatada na resposta SAML

**Permissões Ausentes**:
- Revise as definições de função e as permissões necessárias
- Verifique se há atribuições de função conflitantes
- Verifique se o usuário fez login após as mudanças de função

---
#### Sakai

Sakai oferece suporte ao Registro Dinâmico LTI 1.3 em versões com LTI Advantage. A partir do **Espaço de Administração**:

1. Faça login como administrador do Sakai e abra o **Espaço de Administração**.
2. Escolha **External Tools** > **Install LTI 1.3 Tool**.
3. Cole a URL de registro do FastComments e envie.
4. Aprove a ferramenta quando o handshake for concluído.

A ferramenta então aparece em **External Tools** e pode ser adicionada aos sites pelos mantenedores.

#### Schoology

Instâncias Enterprise do Schoology suportam LTI 1.3, mas a disponibilidade do Registro Dinâmico varia conforme a implantação. Consulte seu gerente de conta do Schoology.

Se o Registro Dinâmico não estiver disponível em sua instância do Schoology, você precisará configurar a integração manualmente usando estes endpoints:

- **OIDC Login URL**: `https://fastcomments.com/lti/v1p3/login`
- **Target Link URL**: `https://fastcomments.com/lti/v1p3/launch`
- **Public Keyset URL (JWKS)**: `https://fastcomments.com/lti/v1p3/jwks`
- **Redirect URLs**: `https://fastcomments.com/lti/v1p3/launch`

Após o Schoology fornecer um Client ID e um Deployment ID, entre em contato com o suporte do FastComments para registrar a configuração no seu tenant.

#### Other LTI 1.3 Platforms

Qualquer LMS que siga a especificação IMS LTI 1.3 Advantage deve funcionar com a mesma URL de registro. Procure uma configuração rotulada como "Dynamic Registration", "Tool Registration URL", "Tool initiation registration endpoint" ou similar.

Se sua plataforma suportar apenas a configuração manual do LTI 1.3, use os quatro endpoints listados na seção do Schoology acima e contate o suporte para finalizar.
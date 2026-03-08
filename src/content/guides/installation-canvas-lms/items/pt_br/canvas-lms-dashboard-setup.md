#### Navigate to Canvas LTI Config

Faça login na sua conta FastComments e vá para <a href="https://fastcomments.com/auth/my-account/canvas-lti-config" target="_blank">My Account > Canvas LTI Config</a>.

#### Create a New LTI Configuration

Marque a caixa de seleção **Enable LTI**. Os campos de configuração aparecerão:

- **Configuration Name** - um rótulo opcional para identificar esta configuração (útil se você conectar várias instâncias do Canvas).
- **Platform URL** - a URL da sua instância do Canvas (por exemplo, `https://yourschool.instructure.com`). Você pode deixar isso em branco por enquanto e preencher depois de criar o Developer Key.
- **Client ID** - deixe isso em branco por enquanto. Você preencherá após criar o Developer Key no Canvas.
- **Deployment ID** - deixe isso em branco por enquanto.
- **Comment Style** - escolha entre Comments, Collab Chat, ou Both. Veja [Commenting Styles](#canvas-lms-commenting-styles) para detalhes.

Clique em **Add** para criar a configuração.

#### Copy the Configuration URLs

Após salvar, três URLs aparecerão:

- **Configuration URL** - você colará isto no Canvas ao criar o Developer Key.
- **OIDC Login URL** - usado pelo Canvas para o fluxo de login LTI (configurado automaticamente via Configuration URL).
- **Launch URL** - o endpoint que o Canvas chama quando um estudante abre o FastComments (configurado automaticamente via Configuration URL).

Copie a **Configuration URL**. Você precisará dela no próximo passo.
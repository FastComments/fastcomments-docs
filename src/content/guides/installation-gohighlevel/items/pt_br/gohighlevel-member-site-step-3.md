Agora vamos gerar o seu código personalizado do FastComments. Use o assistente abaixo para configurar como você quer que o FastComments funcione no seu site GoHighLevel:

[snippet id="gohighlevel-wizard"]

### Diferentes Tipos de Caixa de Comentários

Você pode configurar a linha `TYPE = 'commenting'` para alternar o produto usado (por exemplo, você pode alterá-la para `live` para chat de transmissão ao vivo ou `collab` para chat colaborativo).

### Colocando a Caixa de Comentários Onde Você Quiser

Digamos que você queira colocar caixas de comentários em partes específicas da página e não nos locais padrão.
Altere esta linha:

    const TARGET_ELEMENT_ID = ''; // set to use target div mode

Para:

    const TARGET_ELEMENT_ID = 'fc_box'; // set to use target div mode

Em seguida, no editor do GHL, clique no botão "code" e adicione onde você quer que os comentários apareçam:

[inline-code-attrs-start title = 'Div do FastComments para GoHighLevel'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<div
  id="fc_box"
  type="commenting"
  urlid="custom-chat-id"
></div>
[inline-code-end]

### Tipo Diferente de Caixa de Comentários Por Página

Digamos que você queira que os usuários destaquem e discutam trechos de texto, ou usem a interface de chat de transmissão ao vivo.

Primeiro, siga os passos acima em "Colocando a Caixa de Comentários Onde Você Quiser".

Note que nesse pequeno trecho há `type="commenting"`.

Se você quiser habilitar o chat colaborativo, por exemplo, altere o type para `type="collab"`.

### Mostrar Apenas Em Páginas Específicas

Se você não definir `TARGET_ELEMENT_ID`, você pode, em vez disso, configurar a variável `VALID_PATTERNS`, para definir em quais rotas de URL os comentários devem aparecer. Por padrão, eles serão exibidos
em páginas que contenham `/post` na URL.

### Configurando o Chat Colaborativo

Você pode instruir o chat colaborativo a adicionar funcionalidades colaborativas apenas em torno do HTML dentro de uma área específica. Por exemplo, digamos que você
adicione o código de rodapé acima e, em seguida, adicione esta div no conteúdo da postagem/página para habilitar o chat colaborativo:

[inline-code-attrs-start title = 'Chat Colaborativo Com Conteúdo Especificado'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<div
  id="fc_box"
  type="collab"
  urlid="custom-chat-id"
><p>This content will have collab chat!</p></div>
[inline-code-end]

Então o elemento de parágrafo dentro da `<div>` terá o chat colaborativo habilitado, e nada mais na página. Se você não
colocar nenhum conteúdo na `<div>`, o chat colaborativo será habilitado em todo o corpo da postagem.
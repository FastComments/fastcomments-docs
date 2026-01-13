Todos os botões e elementos de UI no FastComments SDK são personalizáveis. Use o `FastCommentsTheme.Builder` para controle completo da identidade visual do seu app.

### Tematização Programática (Recomendado)

```kotlin
val theme = FastCommentsTheme.Builder()
    // Botões de ação: Enviar, votar, menu, botões curtir/compartilhar
    .setActionButtonColor(Color.parseColor("#FF1976D2"))
    
    // Botões de resposta: Botões para responder comentários  
    .setReplyButtonColor(Color.parseColor("#FF4CAF50"))
    
    // Botões de alternância: Botões de mostrar/ocultar respostas
    .setToggleRepliesButtonColor(Color.parseColor("#FFFF5722"))
    
    // Botões carregar mais: Botões de paginação
    .setLoadMoreButtonTextColor(Color.parseColor("#FF9C27B0"))
    
    .setPrimaryColor(Color.parseColor("#FF6200EE"))
    .setLinkColor(Color.parseColor("#FF1976D2"))
    .setDialogHeaderBackgroundColor(Color.parseColor("#FF333333"))
    .build()

// Aplique o tema
sdk.setTheme(theme)
```

### Substituição Rápida de Cores

Override color resources in your `colors.xml` for simple branding:

```xml
<!-- No res/values/colors.xml do seu app -->
<resources>
    <!-- Altere todos os elementos principais da UI -->
    <color name="primary">#FF1976D2</color>
    
    <!-- Ou personalize tipos específicos de botões -->
    <color name="fastcomments_action_button_color">#FF1976D2</color>
    <color name="fastcomments_reply_button_color">#FF4CAF50</color>
    <color name="fastcomments_toggle_replies_button_color">#FFFF5722</color>
    <color name="fastcomments_load_more_button_text_color">#FF9C27B0</color>
</resources>
```

### Cobertura de Botões Tematizados

**Todos os botões no SDK suportam temas:**
- Botões de envio, botões de voto, botões de menu, botões de resposta
- Botões de mostrar/ocultar respostas, botões de carregar mais  
- Botões de ação do feed (curtir, comentar, compartilhar)
- Botões de diálogo (enviar, cancelar, salvar)
- Botões de tarefa dinâmica em publicações do feed

For detailed theming documentation, see [THEMING.md](https://github.com/FastComments/fastcomments-android/blob/main/THEMING.md).
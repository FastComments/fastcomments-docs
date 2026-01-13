Todos los botones y elementos de la interfaz de usuario en el SDK de FastComments son personalizables mediante temas. Usa `FastCommentsTheme.Builder` para tener control completo sobre la identidad visual de tu aplicación.

### Tematización programática (recomendado)

```kotlin
val theme = FastCommentsTheme.Builder()
    // Botones de acción: enviar, votar, menú, botones de me gusta/compartir
    .setActionButtonColor(Color.parseColor("#FF1976D2"))
    
    // Botones de respuesta: botones de respuesta a comentarios  
    .setReplyButtonColor(Color.parseColor("#FF4CAF50"))
    
    // Botones alternables: botones de mostrar/ocultar respuestas
    .setToggleRepliesButtonColor(Color.parseColor("#FFFF5722"))
    
    // Botones de cargar más: botones de paginación
    .setLoadMoreButtonTextColor(Color.parseColor("#FF9C27B0"))
    
    .setPrimaryColor(Color.parseColor("#FF6200EE"))
    .setLinkColor(Color.parseColor("#FF1976D2"))
    .setDialogHeaderBackgroundColor(Color.parseColor("#FF333333"))
    .build()

// Aplicar el tema
sdk.setTheme(theme)
```

### Anulación rápida de colores

Override color resources in your `colors.xml` for simple branding:

```xml
<!-- En el res/values/colors.xml de tu aplicación -->
<resources>
    <!-- Cambia todos los elementos principales de la interfaz de usuario -->
    <color name="primary">#FF1976D2</color>
    
    <!-- O personaliza tipos de botones específicos -->
    <color name="fastcomments_action_button_color">#FF1976D2</color>
    <color name="fastcomments_reply_button_color">#FF4CAF50</color>
    <color name="fastcomments_toggle_replies_button_color">#FFFF5722</color>
    <color name="fastcomments_load_more_button_text_color">#FF9C27B0</color>
</resources>
```

### Cobertura de botones tematizados

**Cada botón en el SDK admite tematización:**
- Botones de enviar, botones de voto, botones de menú, botones de respuesta
- Botones de mostrar/ocultar respuestas, botones de cargar más  
- Botones de acción del feed (me gusta, comentar, compartir)
- Botones de diálogo (enviar, cancelar, guardar)
- Botones de tareas dinámicas en entradas del feed

Para documentación detallada sobre tematización, consulta [THEMING.md](https://github.com/FastComments/fastcomments-android/blob/main/THEMING.md).
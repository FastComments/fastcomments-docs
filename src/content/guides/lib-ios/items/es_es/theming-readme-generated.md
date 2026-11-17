### Temas predefinidos

Hay cuatro temas predefinidos integrados disponibles:

```swift
// Valores predeterminados del sistema
sdk.theme = FastCommentsTheme.default

// Tarjetas con sombras y esquinas muy redondeadas
sdk.theme = FastCommentsTheme.modern

// Plano, sin sombras, radio de esquina pequeño, sin líneas de hilo
sdk.theme = FastCommentsTheme.minimal

// Establece todos los colores de acción en un solo color de marca
sdk.theme = FastCommentsTheme.allPrimary(.indigo)
```

### Estilos de visualización de comentarios

```swift
var theme = FastCommentsTheme()
theme.commentStyle = .flat    // Lista plana con divisores (predeterminado)
theme.commentStyle = .card    // Tarjetas redondeadas con sombras
theme.commentStyle = .bubble  // Estilo de burbuja de chat
```

### Colores

Todas las propiedades de color son opcionales. Los valores no establecidos recurren a valores predeterminados sensatos del sistema.

```swift
var theme = FastCommentsTheme()

// Colores de marca
theme.primaryColor = .indigo
theme.primaryLightColor = .indigo.opacity(0.6)
theme.primaryDarkColor = Color(red: 0.2, green: 0.1, blue: 0.5)

// Fondos
theme.commentBackgroundColor = Color(.secondarySystemGroupedBackground)
theme.containerBackgroundColor = Color(.systemGroupedBackground)

// Botones de acción
theme.actionButtonColor = .indigo
theme.replyButtonColor = .indigo
theme.toggleRepliesButtonColor = .indigo.opacity(0.8)
theme.loadMoreButtonTextColor = .indigo

// Votos
theme.voteActiveColor = .red
theme.voteCountColor = .primary
theme.voteCountZeroColor = .secondary
theme.voteDividerColor = Color(.separator)

// Enlaces
theme.linkColor = .indigo
theme.linkColorPressed = .indigo.opacity(0.5)

// Diálogos
theme.dialogHeaderBackgroundColor = .indigo
theme.dialogHeaderTextColor = .white

// Barra de entrada
theme.inputBarBackgroundColor = Color(.systemBackground)
theme.inputBarBorderColor = Color(.separator)

// Otros
theme.onlineIndicatorColor = .green
theme.separatorColor = Color(.separator)
theme.badgeBackgroundColor = .gray.opacity(0.2)
theme.threadLineColor = .indigo.opacity(0.15)
```

### Tipografía

```swift
theme.commenterNameFont = .subheadline.weight(.bold)
theme.bodyFont = .body
theme.captionFont = .caption
theme.actionFont = .caption.weight(.medium)
```

### Diseño y espaciado

```swift
theme.cornerRadius = .large       // .none, .small, .medium, .large
theme.commentSpacing = 4          // Puntos entre filas de comentarios
theme.nestingIndent = 20          // Puntos de sangría por nivel de anidamiento
theme.avatarSize = 36             // Diámetro del avatar para comentarios raíz
theme.replyAvatarSize = 28        // Diámetro del avatar para respuestas anidadas
```

### Efectos visuales

```swift
theme.showShadows = true          // Sombras sutiles en las tarjetas
theme.showThreadLine = true       // Línea vertical que conecta las respuestas anidadas
theme.animateVotes = true         // Animación de resorte en los cambios de voto
```

### Aplicación de temas

Dos enfoques:

```swift
// A través del entorno de SwiftUI (recomendado para la jerarquía de vistas)
FastCommentsView(sdk: sdk)
    .fastCommentsTheme(theme)

// Directamente en el SDK
sdk.theme = theme
```

---


### Colores de las encuestas

Las encuestas usan por defecto los colores de la tarjeta de encuesta del widget web, en modo claro y oscuro. Establecer `primaryColor` también cambia el color de acento y la barra de la opción que elegiste. Para establecerlos directamente:

```swift
var theme = FastCommentsTheme()
theme.pollAccentColor = .indigo
```

| Propiedad | Uso |
|----------|----------|
| `pollAccentColor` | Borde y marca de verificación de la opción seleccionada, duración elegida, "Tu voto ha sido registrado" |
| `pollAccentTextColor` | Marca de verificación y texto dibujado sobre el color de acento |
| `pollOptionBackgroundColor` | Fondo de la ficha de opción |
| `pollOptionPressedBackgroundColor` | Fondo de la ficha de opción mientras está presionada |
| `pollBarColor` | Barra de resultados para las opciones que no elegiste |
| `pollSelectedBarColor` | Barra de resultados para la opción que elegiste |
| `pollBorderColor` | Marca de la casilla de verificación y píldoras con contorno |
| `pollDangerColor` | Errores y "Eliminar encuesta" |
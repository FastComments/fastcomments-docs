### Preajustes de tema

Cuatro preajustes integrados están disponibles:

```swift
// Valores predeterminados del sistema
sdk.theme = FastCommentsTheme.default

// Tarjetas con sombras y esquinas grandes redondeadas
sdk.theme = FastCommentsTheme.modern

// Plano, sin sombras, radio de esquina pequeño, sin líneas de hilo
sdk.theme = FastCommentsTheme.minimal

// Establecer todos los colores de acción a un único color de marca
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

Todas las propiedades de color son opcionales. Los valores no establecidos vuelven a los valores predeterminados sensatos del sistema.

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
theme.showThreadLine = true       // Línea vertical que conecta respuestas anidadas
theme.animateVotes = true         // Animación tipo resorte en cambios de votación
```

### Aplicación de temas

Dos enfoques:

```swift
// A través del entorno SwiftUI (recomendado para la jerarquía de vistas)
FastCommentsView(sdk: sdk)
    .fastCommentsTheme(theme)

// Directamente en el SDK
sdk.theme = theme
```

---
---
Cuando los Perfiles de Usuario se abren en el contexto de tu sitio (a través del widget de comentarios), cualquier estilo CSS personalizado que hayas aplicado a tu widget de FastComments se inyecta automáticamente en el modal de perfil.

### Cómo Funciona

Cuando un usuario hace clic en un enlace de perfil desde tu widget de comentarios, se abre un modal de perfil con la clase `.fast-comments-profile`. El CSS personalizado de tu widget se inyecta automáticamente en la vista de perfil. Si ya has estilizado tu widget de comentarios, esos estilos se aplicarán a los perfiles.

### Clases CSS

Los perfiles de FastComments usan una arquitectura CSS basada en clases. No utiliza propiedades personalizadas de CSS.

La página principal del perfil usa `.user-profile` como el contenedor raíz. La sección de encabezado es `.profile-header` con `.profile-header-background` para la imagen de fondo. El contenido del perfil se encuentra en `.profile-content`.

El avatar usa `.profile-avatar` y `.profile-avatar-wrapper`. El nombre del usuario es `.profile-name` y el texto de biografía es `.profile-bio`. Las estadísticas están en `.profile-stats` con estadísticas individuales usando `.stat`.

Los enlaces sociales están en `.profile-social-links` con enlaces individuales como `.social-link`. Las insignias usan `.profile-badges` y `.badge`. Las barras de progreso de las insignias usan `.progress-outer` y `.progress-bar`.

Las pestañas usan `.profile-tabs` para el contenedor, `.tab` para pestañas individuales, y `.tab.active` para la pestaña seleccionada. El contenido de las pestañas usa `.tab-body` y `.tab-body.active`. Los contadores de notificaciones en las pestañas usan `.tab .count`.

Las notificaciones usan `.notification` y las conversaciones de mensajes directos usan `.conversation`. El estado en línea es `.activity-indicator` con `.activity-indicator.online` para el estado activo. Los contadores de no leídos usan `.unread-count`.

El contenedor del modal de perfil es `.fast-comments-profile` con `.fast-comments-profile-close` para el botón de cierre.

### Modo Oscuro

El modo oscuro usa el modificador de clase `.dark` en `.user-profile`.

```css
.user-profile.dark {
    background-color: #181a1b;
    color: #fff;
}
```

### Ejemplos

**Encabezado:**
```css
.user-profile .profile-header-background {
    background: linear-gradient(to right, #667eea, #764ba2);
}
```

**Insignias:**
```css
.user-profile .badge {
    background: #007bff;
    color: white;
    border-radius: 24px;
}
```

**Pestañas:**
```css
.user-profile .tab.active {
    color: #007bff;
    border-bottom: 3px solid #007bff;
}
```

**Modal:**
```css
.fast-comments-profile {
    border-radius: 12px 0 0 12px;
}
```
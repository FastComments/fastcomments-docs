Después de llamar a `sdk.load()`, el SDK se suscribe automáticamente a los eventos de WebSocket para el `urlId` configurado. Se manejan los siguientes eventos:

- Nuevos comentarios, ediciones y eliminaciones
- Votos (nuevos y eliminados)
- Cambios de estado de pin, lock, flag y block
- Presencia de usuarios (entrada/salida)
- Apertura/cierre de hilo
- Otorgamiento de insignias
- Actualizaciones de configuración del servidor

### Controlar la visualización en vivo

Por defecto, los nuevos comentarios de otros usuarios aparecen inmediatamente:

```swift
sdk.showLiveRightAway = true   // Predeterminado: mostrarse inmediatamente
```

Establece esto en `false` para almacenar en búfer los nuevos comentarios detrás de un botón "N nuevos comentarios", permitiendo que el usuario elija cuándo revelarlos:

```swift
sdk.showLiveRightAway = false
```

### Presencia de usuario

Los indicadores de en línea/sin conexión aparecen automáticamente en los avatares de usuario cuando el servidor habilita el seguimiento de presencia. No se necesita configuración adicional en el cliente.

---
---
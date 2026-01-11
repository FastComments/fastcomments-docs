### Errores 401 — No autorizado

Si obtienes errores 401 al usar la API autenticada:

1. **Comprueba tu clave de API**: Asegúrate de que estás usando la clave de API correcta desde tu panel de FastComments
2. **Verifica el tenant ID**: Asegúrate de que el tenant ID coincida con tu cuenta
3. **Formato de la clave de API**: La clave de API debe pasarse en la Configuration:

```rust
let mut config = Configuration::new();
config.api_key = Some(ApiKey {
    prefix: None,
    key: "YOUR_API_KEY".to_string(),
});
```

### Problemas con tokens SSO

Si los tokens SSO no funcionan:

1. **Usa el modo seguro en producción**: Siempre usa `FastCommentsSSO::new_secure()` con tu clave de API en producción
2. **Sólo en el servidor**: Genera los tokens SSO en tu servidor, nunca expongas tu clave de API a los clientes
3. **Verifica los datos del usuario**: Asegúrate de que se proporcionen todos los campos requeridos (id, email, username)

### Errores del runtime asíncrono

El SDK usa tokio para operaciones asíncronas. Asegúrate de:

1. Añade tokio a tus dependencias:
```toml
[dependencies]
tokio = { version = "1", features = ["full"] }
```

2. Usa el runtime de tokio:
```rust
#[tokio::main]
async fn main() {
    // Tu código asíncrono aquí
}
```
## Cómo personalizar los estilos del widget de comentarios

Puedes personalizar el estilo del widget de comentarios de dos maneras:

### Opción 1: Mediante el parámetro `customCSS`

Pasa tu CSS personalizado como una cadena al parámetro `customCSS` al inicializar el widget:

```javascript
window.FastCommentsUI(document.getElementById('fastcomments-widget'), {
    tenantId: 'your-tenant-id',
    customCSS: `
        .fast-comments .comment {
            background-color: #f0f0f0 !important;
            border-radius: 8px !important;
        }
    `
});
```

### Opción 2: A través del Panel de Administración

1. Ve a la [página de personalización del widget](https://fastcomments.com/auth/my-account/customize-widget) en tu panel de administración
2. Desplázate a la sección "CSS personalizado" bajo "Avanzado"
3. Introduce tu CSS personalizado
4. Haz clic en "Guardar"

Tu CSS personalizado se aplicará a todos los widgets de comentarios en tu sitio.

## Consejos

- Usa `!important` para anular los estilos predeterminados si es necesario
- Apunta a selectores específicos para evitar afectar otras partes de tu sitio
- Prueba tu CSS en diferentes navegadores para comprobar la compatibilidad
- El widget utiliza CSS estándar - no se requieren preprocesadores especiales
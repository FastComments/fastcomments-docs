El módulo añade tres permisos de Drupal que puede asignar por rol en `People > Permissions`.

- **Administer FastComments** - Acceso al formulario de configuración de FastComments en `/admin/config/content/fastcomments`.
- **View FastComments** - Requerido para ver el widget de comentarios. Sin este permiso, el widget no se renderiza.
- **Toggle FastComments** - Permite a los usuarios habilitar o deshabilitar los comentarios por entidad mediante el widget de campo.

Por defecto, solo los usuarios con el permiso `administer site configuration` pueden cambiar la configuración de FastComments. Conceda `View FastComments` a usuarios anónimos y autenticados si desea que los visitantes vean el widget.

---
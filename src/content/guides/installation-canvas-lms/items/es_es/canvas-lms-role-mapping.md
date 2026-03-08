Los roles de Canvas se asignan automáticamente a los roles de FastComments durante el lanzamiento LTI. No se necesita configuración manual.

#### Mapeo de roles

| Rol de Canvas | Rol de FastComments | Permisos |
|---|---|---|
| **Administrator** | Admin | Acceso completo a la cuenta, gestionar todos los comentarios y la configuración |
| **Instructor** | Moderator | Editar y eliminar comentarios, fijar hilos, gestionar discusiones |
| **Learner** | Commenter | Publicar comentarios, responder, votar y usar menciones |

#### Cómo funciona

Cuando un usuario inicia FastComments desde Canvas, el protocolo LTI 1.3 incluye su rol en Canvas. FastComments lee este rol y asigna automáticamente los permisos correspondientes.

Si un usuario tiene varios roles (p. ej., un Instructor que también es Admin), se utiliza el rol con mayores privilegios.

---
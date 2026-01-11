---
Para desarrolladores que quizás no quieras que sean `Administrators`, considera crear un usuario `Administrator` con los siguientes permisos:

1. Administrador de Analíticas
2. Administrador de Personalizaciones
3. Administrador de Gestión de Datos
4. Administrador de Moderación de Comentarios
5. Administrador de API/SSO

Este conjunto de permisos le dará a un desarrollador todo lo que necesita para configurar FastComments, así como la visibilidad en el sistema necesaria para asegurarse de que está funcionando.

La razón de estos permisos es la siguiente:

1. **Administrador de Analíticas**: Esto puede usarse para depurar el uso de FastComments.
2. **Administrador de Personalizaciones**: Esto será necesario para aplicar estilos personalizados al widget de comentarios.
3. **Administrador de Gestión de Datos**: Esto será necesario para gestionar importaciones y exportaciones, y configurar webhooks.
4. **Administrador de Moderación de Comentarios**: Esto será necesario para ver los datos de los comentarios, al menos durante la configuración.
5. **Administrador de API/SSO**: Esto les permitirá obtener las claves de la API directamente desde nuestra plataforma. Consideramos que esto es más seguro que un `Administrator` copiándolas por ellos y enviando el secreto de la API por correo electrónico, lo cual puede no ser muy seguro.
---
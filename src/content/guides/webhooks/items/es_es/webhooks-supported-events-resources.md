---
FastComments admite webhooks solo para el recurso Comentario.

Admitimos webhooks para la creación, eliminación y actualización de comentarios.

Cada uno de estos se considera un evento separado en nuestro sistema y, como tal, tiene diferentes semánticas y estructuras para los eventos de webhook.

Cualquier número de endpoints puede suscribirse al mismo evento: se puede configurar un webhook por dominio en el panel de control, y se pueden crear suscripciones adicionales a través de la API (ver Gestión de Webhooks mediante la API).

---
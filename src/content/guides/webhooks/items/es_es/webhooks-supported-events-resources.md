FastComments admite webhooks solo para el recurso Comentario.

Admitimos webhooks para la creación, eliminación y actualización de comentarios.

Cada uno de estos se considera un evento separado en nuestro sistema y, como tal, tiene diferentes semánticas
y estructuras para los eventos de webhook.

Cualquier número de endpoints puede suscribirse al mismo evento, desde el panel de control o a través de la API
(vea Administrar Webhooks a través de la API). Cada webhook se entrega de forma independiente.
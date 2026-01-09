---
Por defecto, FastComments permite enlazar a cualquier sitio externo.

Esto puede restringirse a una lista deseada de sitios o dominios. Intentar publicar un enlace a un sitio o dominio,
que no esté en la lista definida mostrará un error al usuario.

Esta validación es solo para el Widget de Comentarios y la API. Las importaciones no se ven afectadas.

Esto se hace sin código, en la página de personalización del widget:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.restricted-link-domains-list'; selector = '.external-link-settings'; title='Restrict External Link Domains' app-screenshot-end]

---
---
Para la mayoría de los sitios, la forma más sencilla de añadir comentarios es adjuntar el campo `FastComments` a sus tipos de contenido. Vaya a `Structure > Content types > [type] > Manage fields` y añada el campo.

Cada entidad que tenga el campo obtiene:

- Un **conmutador de estado** para que los editores puedan activar o desactivar los comentarios por entidad.
- Un **identificador personalizado** opcional para que pueda usar un ID estable que no esté vinculado a la ruta de la entidad de Drupal.

El bloque principal `FastComments Widget` reconoce este campo y omitirá las entidades que ya lo tengan adjunto. De este modo, puede combinar comentarios por entidad con el bloque sin ver el widget dos veces en la misma página.
---
Existen múltiples aspectos de seguridad al permitir que las personas añadan contenido a un sitio web y luego renderizar ese contenido en muchos tipos diferentes de dispositivos.

### Prevención del abuso del formato

Las personas pueden escribir contenido que sea intencionalmente visualmente distractor
y que reste valor a las discusiones abusando del formato de texto.

FastComments hace varias cosas para prevenir el abuso con respecto al formato:

- Se colapsan grandes saltos de línea consecutivos repetidos.
- No renderizamos encabezados (se convierten en texto normal).
- No permitimos CSS ni colores personalizados.

### Prevención de exploits

Se pueden crear exploits en sistemas que renderizan HTML. FastComments hace varias cosas para prevenir esto:

- Solo permitimos un conjunto de etiquetas HTML explícitamente definido.
- Solo permitimos un conjunto de atributos de etiquetas HTML explícitamente definido.
- Purificamos y saneamos todas las entradas.
  - Esto se hace mediante las bibliotecas [DOMPurify](https://www.npmjs.com/package/dompurify) y [sanitizeHtml](https://www.npmjs.com/package/sanitize-html).
  - Elegimos estas bibliotecas por estar bien probadas (con más de 4 millones y 1 millón de descargas por semana, respectivamente).

Esto significa que los usuarios no pueden hacer cosas como escribir `<script>` o `<style>` etiquetas, o intentar añadir scripts del tipo `onload=alert()` a imágenes u otro contenido.

Las etiquetas HTML que permitimos son las siguientes:

- `<b>`
- `<em>`
- `<u>`
- `<i>`
- `<strike>`
- `<pre>`
- `<span>`
- `<code>`
- `<img>`
- `<a>`
- `<strong>`
- `<ul>`
- `<ol>`
- `<li>`
- `<br>`

La etiqueta `<iframe>` no está permitida por defecto. Si activas Allow Media Embeds, los iframes también están permitidos, pero solo cuando su fuente sea una de una lista incorporada de proveedores de confianza (como YouTube, Vimeo, SoundCloud y Spotify) o un nombre de host que hayas añadido explícitamente. Los iframes procedentes de cualquier otra fuente se eliminan.
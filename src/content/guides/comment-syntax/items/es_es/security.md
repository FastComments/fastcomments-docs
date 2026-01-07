Hay múltiples aspectos de seguridad cuando se permite a las personas agregar contenido a un sitio web y luego renderizar ese contenido en muchos tipos diferentes de dispositivos.

### Prevención del abuso de formato

Las personas pueden escribir contenido que es intencionalmente distractor visualmente y resta valor a las discusiones al abusar del formato de texto.

FastComments hace varias cosas para prevenir el abuso en relación con el formato:

- Los saltos de línea grandes y consecutivos repetidos se colapsan.
- No renderizamos encabezados (se convierten en texto normal).
- No permitimos CSS ni colores personalizados.

### Prevención de exploits

Los exploits se pueden crear en sistemas que renderizan HTML. FastComments hace varias cosas para prevenir esto:

- Solo permitimos un conjunto explícitamente definido de etiquetas HTML.
- Solo permitimos un conjunto explícitamente definido de atributos de etiquetas HTML.
- Purificamos y saneamos todas las entradas.
  - Esto se hace a través de las bibliotecas [DOMPurify](https://www.npmjs.com/package/dompurify) y [sanitizeHtml](https://www.npmjs.com/package/sanitize-html).
  - Elegimos estas bibliotecas por estar bien probadas (con más de 4 y 1 millón de descargas por semana, respectivamente).

Esto significa que los usuarios no pueden hacer cosas como escribir etiquetas `<script>` o `<style>`, o intentar agregar scripts tipo `onload=alert()` a imágenes u otro contenido.

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
